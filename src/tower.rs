//! Tower middleware recording RED metrics (feature `tower`).
//!
//! [`RedLayer`] wraps any [`tower_service::Service`] that handles
//! `http::Request`s and returns `http::Response`s (axum, tonic, hyper
//! services, …) and records, per request:
//!
//! | Metric | Kind | Labels |
//! |---|---|---|
//! | `<prefix>http_requests_total` | counter | `method`, `status`, optional `route` |
//! | `<prefix>http_request_duration_seconds` | histogram | `method`, `status`, optional `route` |
//! | `<prefix>http_requests_in_flight` | up/down counter | none |
//!
//! `status` is the numeric code, or `error` when the inner service failed.
//! `method` is the HTTP method for the nine standard methods and `other` for
//! anything else, so a client cannot inflate cardinality with made-up
//! methods. Raw paths are never used as labels; supply a *route template*
//! with [`RedLayer::with_route_label`] (for axum, the `MatchedPath`
//! extension).
//!
//! A request whose future is dropped before completing (client went away)
//! still decrements the in-flight counter, but records no status.
//!
//! # Example
//!
//! ```
//! use metrics_lib::tower::RedLayer;
//! use metrics_lib::MetricsCore;
//! use std::sync::Arc;
//! use tower::{Layer, Service, ServiceExt};
//!
//! # let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
//! # rt.block_on(async {
//! let core = Arc::new(MetricsCore::new());
//! let svc = tower::service_fn(|_req: http::Request<()>| async {
//!     Ok::<_, std::convert::Infallible>(http::Response::new(()))
//! });
//! let mut svc = RedLayer::new(Arc::clone(&core)).layer(svc);
//! let _ = svc.ready().await.unwrap().call(http::Request::new(())).await;
//!
//! let labels = metrics_lib::labels!("method" => "GET", "status" => "200");
//! assert_eq!(core.counter_with("http_requests_total", &labels).get(), 1);
//! # });
//! ```

use crate::{LabelSet, MetricsCore, UpDownCounter};
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Instant;

type RouteFn = dyn Fn(&http::Uri, &http::Extensions) -> Option<String> + Send + Sync;

#[derive(Clone)]
struct Config {
    core: Arc<MetricsCore>,
    requests: Arc<str>,
    duration: Arc<str>,
    in_flight: Arc<str>,
    route: Option<Arc<RouteFn>>,
}

/// Layer that adds RED metrics to a service. See the module docs.
#[derive(Clone)]
pub struct RedLayer {
    config: Config,
}

impl RedLayer {
    /// Record into `core` with unprefixed metric names.
    #[must_use]
    pub fn new(core: Arc<MetricsCore>) -> Self {
        Self {
            config: Config {
                core,
                requests: "http_requests_total".into(),
                duration: "http_request_duration_seconds".into(),
                in_flight: "http_requests_in_flight".into(),
                route: None,
            },
        }
    }

    /// Prefix every metric name (for example `"api_"`).
    #[must_use]
    pub fn with_prefix(mut self, prefix: &str) -> Self {
        self.config.requests = format!("{prefix}http_requests_total").into();
        self.config.duration = format!("{prefix}http_request_duration_seconds").into();
        self.config.in_flight = format!("{prefix}http_requests_in_flight").into();
        self
    }

    /// Add a `route` label computed from the request URI and extensions.
    /// Return a route *template* (`/users/{id}`), never the raw path, or
    /// `None` to omit the label.
    #[must_use]
    pub fn with_route_label(
        mut self,
        route: impl Fn(&http::Uri, &http::Extensions) -> Option<String> + Send + Sync + 'static,
    ) -> Self {
        self.config.route = Some(Arc::new(route));
        self
    }
}

impl fmt::Debug for RedLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RedLayer")
            .field("requests", &self.config.requests)
            .finish_non_exhaustive()
    }
}

impl<S> tower_layer::Layer<S> for RedLayer {
    type Service = RedService<S>;

    fn layer(&self, inner: S) -> RedService<S> {
        RedService {
            inner,
            config: self.config.clone(),
        }
    }
}

/// Service produced by [`RedLayer`].
#[derive(Clone)]
pub struct RedService<S> {
    inner: S,
    config: Config,
}

impl<S> fmt::Debug for RedService<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RedService").finish_non_exhaustive()
    }
}

fn method_label(method: &http::Method) -> &'static str {
    match *method {
        http::Method::GET => "GET",
        http::Method::POST => "POST",
        http::Method::PUT => "PUT",
        http::Method::DELETE => "DELETE",
        http::Method::HEAD => "HEAD",
        http::Method::OPTIONS => "OPTIONS",
        http::Method::CONNECT => "CONNECT",
        http::Method::PATCH => "PATCH",
        http::Method::TRACE => "TRACE",
        _ => "other",
    }
}

impl<S, ReqBody, ResBody> tower_service::Service<http::Request<ReqBody>> for RedService<S>
where
    S: tower_service::Service<http::Request<ReqBody>, Response = http::Response<ResBody>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = RedFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: http::Request<ReqBody>) -> Self::Future {
        let mut labels = LabelSet::new();
        let _ = labels.add("method", method_label(req.method()));
        if let Some(route) = self
            .config
            .route
            .as_ref()
            .and_then(|f| f(req.uri(), req.extensions()))
        {
            let _ = labels.add("route", route);
        }
        let in_flight = self.config.core.up_down_counter(&self.config.in_flight);
        in_flight.inc();
        RedFuture {
            inner: self.inner.call(req),
            state: Some(InFlight {
                config: self.config.clone(),
                labels,
                start: Instant::now(),
                gauge: in_flight,
            }),
        }
    }
}

/// Request bookkeeping; decrements the in-flight gauge when dropped.
struct InFlight {
    config: Config,
    labels: LabelSet,
    start: Instant,
    gauge: Arc<UpDownCounter>,
}

impl InFlight {
    fn finish(mut self, status: String) {
        let elapsed = self.start.elapsed();
        let _ = self.labels.add("status", status);
        let core = &self.config.core;
        core.counter_with(&self.config.requests, &self.labels).inc();
        core.histogram_with(&self.config.duration, &self.labels)
            .observe_duration(elapsed);
    }
}

impl Drop for InFlight {
    fn drop(&mut self) {
        self.gauge.dec();
    }
}

pin_project_lite::pin_project! {
    /// Response future of [`RedService`].
    pub struct RedFuture<F> {
        #[pin]
        inner: F,
        state: Option<InFlight>,
    }
}

impl<F> fmt::Debug for RedFuture<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RedFuture").finish_non_exhaustive()
    }
}

impl<F, ResBody, E> Future for RedFuture<F>
where
    F: Future<Output = Result<http::Response<ResBody>, E>>,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let output = match this.inner.poll(cx) {
            Poll::Ready(output) => output,
            Poll::Pending => return Poll::Pending,
        };
        if let Some(state) = this.state.take() {
            let status = match &output {
                Ok(response) => response.status().as_str().to_owned(),
                Err(_) => "error".to_owned(),
            };
            state.finish(status);
        }
        Poll::Ready(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::Infallible;
    use tower::{Layer, Service, ServiceExt};

    fn runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap()
    }

    #[test]
    fn test_records_requests_durations_and_statuses() {
        let core = Arc::new(MetricsCore::new());
        let layer = RedLayer::new(Arc::clone(&core))
            .with_prefix("api_")
            .with_route_label(|uri, _| {
                uri.path()
                    .starts_with("/users/")
                    .then(|| "/users/{id}".to_owned())
            });
        assert!(format!("{layer:?}").contains("api_http_requests_total"));
        let svc = tower::service_fn(|req: http::Request<()>| async move {
            let status = if req.uri().path() == "/missing" {
                404
            } else {
                200
            };
            let mut resp = http::Response::new(());
            *resp.status_mut() = http::StatusCode::from_u16(status).unwrap();
            Ok::<_, Infallible>(resp)
        });
        let mut svc = layer.layer(svc);
        assert!(format!("{svc:?}").contains("RedService"));
        runtime().block_on(async {
            for path in ["/users/1", "/users/2", "/missing"] {
                let req = http::Request::builder().uri(path).body(()).unwrap();
                let fut = svc.ready().await.unwrap().call(req);
                assert!(format!("{fut:?}").contains("RedFuture"));
                let _ = fut.await.unwrap();
            }
            let custom = http::Request::builder()
                .method(http::Method::from_bytes(b"BREW").unwrap())
                .uri("/")
                .body(())
                .unwrap();
            let _ = svc.ready().await.unwrap().call(custom).await.unwrap();
        });
        let users = LabelSet::from([
            ("method", "GET"),
            ("route", "/users/{id}"),
            ("status", "200"),
        ]);
        assert_eq!(
            core.counter_with("api_http_requests_total", &users).get(),
            2
        );
        assert_eq!(
            core.histogram_with("api_http_request_duration_seconds", &users)
                .count(),
            2
        );
        let missing = LabelSet::from([("method", "GET"), ("status", "404")]);
        assert_eq!(
            core.counter_with("api_http_requests_total", &missing).get(),
            1
        );
        let other = LabelSet::from([("method", "other"), ("status", "200")]);
        assert_eq!(
            core.counter_with("api_http_requests_total", &other).get(),
            1
        );
        assert_eq!(core.up_down_counter("api_http_requests_in_flight").get(), 0);
    }

    #[test]
    fn test_errors_and_cancellation() {
        let core = Arc::new(MetricsCore::new());
        let failing = tower::service_fn(|_req: http::Request<()>| async {
            Err::<http::Response<()>, _>("boom")
        });
        let mut svc = RedLayer::new(Arc::clone(&core)).layer(failing);
        runtime().block_on(async {
            assert!(
                svc.ready()
                    .await
                    .unwrap()
                    .call(http::Request::new(()))
                    .await
                    .is_err()
            );
        });
        let labels = LabelSet::from([("method", "GET"), ("status", "error")]);
        assert_eq!(core.counter_with("http_requests_total", &labels).get(), 1);

        let pending = tower::service_fn(|_req: http::Request<()>| {
            std::future::pending::<Result<http::Response<()>, Infallible>>()
        });
        let mut svc = RedLayer::new(Arc::clone(&core)).layer(pending);
        let fut = svc.call(http::Request::new(()));
        assert_eq!(core.up_down_counter("http_requests_in_flight").get(), 1);
        drop(fut);
        assert_eq!(core.up_down_counter("http_requests_in_flight").get(), 0);
    }

    #[test]
    fn test_method_labels() {
        for (m, want) in [
            (http::Method::POST, "POST"),
            (http::Method::PUT, "PUT"),
            (http::Method::DELETE, "DELETE"),
            (http::Method::HEAD, "HEAD"),
            (http::Method::OPTIONS, "OPTIONS"),
            (http::Method::CONNECT, "CONNECT"),
            (http::Method::PATCH, "PATCH"),
            (http::Method::TRACE, "TRACE"),
        ] {
            assert_eq!(method_label(&m), want);
        }
    }
}
