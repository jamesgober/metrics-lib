//! Dependency-free HTTP helpers (feature `http`): a `/metrics` endpoint and
//! a Prometheus Pushgateway client, both plain HTTP/1.1 over `std::net`.
//!
//! These are for services that do not already run an HTTP stack. If yours
//! does (axum, actix, hyper, …), serve
//! [`prometheus::render_core`](super::prometheus::render_core) from a route
//! instead; see `examples/axum_registry_integration.rs`.
//!
//! Neither helper speaks TLS. Put the endpoint behind your ingress or bind it
//! to a private interface, and reach the Pushgateway over a trusted network.
//!
//! # Serving `/metrics`
//!
//! ```no_run
//! use metrics_lib::exporters::http::MetricsServer;
//! use metrics_lib::MetricsCore;
//! use std::sync::Arc;
//!
//! let core = Arc::new(MetricsCore::new());
//! let server = MetricsServer::spawn("127.0.0.1:9898", Arc::clone(&core))?;
//! // ... GET http://127.0.0.1:9898/metrics ...
//! server.shutdown();
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! # Pushing to a Pushgateway
//!
//! ```no_run
//! use metrics_lib::exporters::http::Pushgateway;
//! use metrics_lib::{LabelSet, MetricsCore};
//!
//! let core = MetricsCore::new();
//! let gateway = Pushgateway::new("http://pushgateway:9091")?;
//! gateway.push("nightly_backup", &LabelSet::from([("instance", "db-1")]), core.registry())?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use super::{openmetrics, prometheus};
use crate::{LabelSet, MetricsCore, MetricsError, Registry, validate_label_key};
use std::fmt::Write as _;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Largest request head (request line plus headers) the server reads.
const MAX_REQUEST_HEAD: usize = 8 * 1024;
/// How long the server waits for a client to send its request.
const READ_TIMEOUT: Duration = Duration::from_secs(5);
/// How often the accept loop re-checks for shutdown.
const ACCEPT_POLL: Duration = Duration::from_millis(50);

/// A blocking HTTP server for `GET /metrics` and `GET /health`, running on
/// its own thread.
///
/// `/metrics` answers with the OpenMetrics format when the `Accept` header
/// asks for `application/openmetrics-text`, and with the Prometheus format
/// otherwise; it includes the `process_*` families when the core has them
/// enabled. `/health` answers `ok`. Other paths get 404, other methods 405.
/// Requests are served one at a time, which is ample for scrapers.
pub struct MetricsServer {
    local_addr: SocketAddr,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl MetricsServer {
    /// Bind `addr` and serve `core` on a background thread.
    ///
    /// # Errors
    ///
    /// Returns the bind error, or the error from configuring the listener or
    /// spawning the thread.
    pub fn spawn(addr: impl ToSocketAddrs, core: Arc<MetricsCore>) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;
        let local_addr = listener.local_addr()?;
        let stop = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&stop);
        let thread = thread::Builder::new()
            .name("metrics-lib-http".into())
            .spawn(move || accept_loop(&listener, &core, &flag))?;
        Ok(Self {
            local_addr,
            stop,
            thread: Some(thread),
        })
    }

    /// The bound address (useful after binding port 0).
    #[must_use]
    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }

    /// Stop accepting connections and wait for the server thread to exit.
    pub fn shutdown(mut self) {
        self.stop_and_join();
    }

    fn stop_and_join(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(thread) = self.thread.take() {
            // A panic in the server thread has nothing left to clean up.
            let _ = thread.join();
        }
    }
}

impl Drop for MetricsServer {
    fn drop(&mut self) {
        self.stop_and_join();
    }
}

impl std::fmt::Debug for MetricsServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetricsServer")
            .field("local_addr", &self.local_addr)
            .finish_non_exhaustive()
    }
}

fn accept_loop(listener: &TcpListener, core: &MetricsCore, stop: &AtomicBool) {
    while !stop.load(Ordering::Acquire) {
        match listener.accept() {
            Ok((stream, _)) => {
                // A failed exchange only affects that client; keep serving.
                let _ = serve_connection(stream, core);
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => thread::sleep(ACCEPT_POLL),
            Err(_) => thread::sleep(ACCEPT_POLL),
        }
    }
}

struct Request {
    method: String,
    path: String,
    accepts_openmetrics: bool,
}

fn read_request(stream: &TcpStream) -> io::Result<Request> {
    let mut reader = BufReader::new(stream.take(MAX_REQUEST_HEAD as u64));
    let mut line = String::new();
    let _ = reader.read_line(&mut line)?;
    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap_or_default().to_owned();
    let target = parts.next().unwrap_or_default();
    let path = target.split('?').next().unwrap_or_default().to_owned();
    let mut accepts_openmetrics = false;
    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 || line == "\r\n" || line == "\n" {
            break;
        }
        if let Some((name, value)) = line.split_once(':') {
            if name.trim().eq_ignore_ascii_case("accept")
                && value.contains("application/openmetrics-text")
            {
                accepts_openmetrics = true;
            }
        }
    }
    Ok(Request {
        method,
        path,
        accepts_openmetrics,
    })
}

fn serve_connection(mut stream: TcpStream, core: &MetricsCore) -> io::Result<()> {
    stream.set_nonblocking(false)?;
    stream.set_read_timeout(Some(READ_TIMEOUT))?;
    stream.set_write_timeout(Some(READ_TIMEOUT))?;
    let request = read_request(&stream)?;
    let (status, content_type, body) = match (request.method.as_str(), request.path.as_str()) {
        ("GET" | "HEAD", "/metrics") => {
            if request.accepts_openmetrics {
                (
                    "200 OK",
                    openmetrics::CONTENT_TYPE,
                    openmetrics::render_core(core),
                )
            } else {
                (
                    "200 OK",
                    prometheus::CONTENT_TYPE,
                    prometheus::render_core(core),
                )
            }
        }
        ("GET" | "HEAD", "/health") => ("200 OK", "text/plain; charset=utf-8", "ok\n".to_owned()),
        ("GET" | "HEAD", _) => (
            "404 Not Found",
            "text/plain; charset=utf-8",
            "not found\n".to_owned(),
        ),
        _ => (
            "405 Method Not Allowed",
            "text/plain; charset=utf-8",
            "method not allowed\n".to_owned(),
        ),
    };
    let mut head = String::with_capacity(160);
    let _ = write!(
        head,
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n",
        body.len()
    );
    if status.starts_with("405") {
        head.push_str("Allow: GET, HEAD\r\n");
    }
    head.push_str("\r\n");
    stream.write_all(head.as_bytes())?;
    if request.method != "HEAD" {
        stream.write_all(body.as_bytes())?;
    }
    stream.flush()?;
    // The client may already be gone; nothing to do about it.
    let _ = stream.shutdown(Shutdown::Both);
    Ok(())
}

/// Client for the Prometheus Pushgateway HTTP API.
///
/// Grouping-key values that are not plain URL path segments are sent in the
/// gateway's `@base64` form, so any value (including `/` and the empty
/// string) is safe.
#[derive(Debug, Clone)]
pub struct Pushgateway {
    host: String,
    port: u16,
    base_path: String,
    timeout: Duration,
}

/// Default request timeout of [`Pushgateway`].
pub const DEFAULT_PUSH_TIMEOUT: Duration = Duration::from_secs(10);

impl Pushgateway {
    /// Client for the gateway at `url` (`http://host[:port][/path]`; port 80
    /// when omitted).
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::Config`] when `url` is not an `http://` URL
    /// with a host (`https://` is not supported).
    pub fn new(url: &str) -> crate::Result<Self> {
        let rest = url.strip_prefix("http://").ok_or_else(|| {
            MetricsError::Config(
                "pushgateway URL must start with http:// (TLS is not supported)".into(),
            )
        })?;
        let (authority, path) = match rest.find('/') {
            Some(i) => (&rest[..i], rest[i..].trim_end_matches('/')),
            None => (rest, ""),
        };
        // `[v6addr]:port`, `[v6addr]`, `host:port` or `host`.
        let (host, port_text) = if authority.starts_with('[') {
            match authority.find(']') {
                Some(end) => {
                    let (h, rest) = authority.split_at(end + 1);
                    (h, rest.strip_prefix(':'))
                }
                None => (authority, None),
            }
        } else {
            match authority.rsplit_once(':') {
                Some((h, p)) => (h, Some(p)),
                None => (authority, None),
            }
        };
        let port = match port_text {
            Some(p) => p
                .parse::<u16>()
                .map_err(|_| MetricsError::Config("pushgateway URL has an invalid port".into()))?,
            None => 80,
        };
        if host.is_empty() {
            return Err(MetricsError::Config("pushgateway URL has no host".into()));
        }
        Ok(Self {
            host: host.to_owned(),
            port,
            base_path: path.to_owned(),
            timeout: DEFAULT_PUSH_TIMEOUT,
        })
    }

    /// Set the connect/read/write timeout.
    #[must_use]
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Replace every metric of the group `(job, grouping)` with the registry
    /// (HTTP `PUT`).
    ///
    /// # Errors
    ///
    /// Returns an `InvalidInput` error for an invalid grouping-label key, the
    /// I/O error of the exchange, or an `Other` error carrying the status
    /// line and the start of the body when the gateway does not answer 2xx.
    pub fn push(&self, job: &str, grouping: &LabelSet, registry: &Registry) -> io::Result<()> {
        self.send("PUT", job, grouping, &prometheus::render(registry))
    }

    /// Add or replace metrics of the group with the registry, keeping the
    /// group's other metrics (HTTP `POST`).
    ///
    /// # Errors
    ///
    /// As [`Pushgateway::push`].
    pub fn push_add(&self, job: &str, grouping: &LabelSet, registry: &Registry) -> io::Result<()> {
        self.send("POST", job, grouping, &prometheus::render(registry))
    }

    /// Delete the group (HTTP `DELETE`).
    ///
    /// # Errors
    ///
    /// As [`Pushgateway::push`].
    pub fn delete(&self, job: &str, grouping: &LabelSet) -> io::Result<()> {
        self.send("DELETE", job, grouping, "")
    }

    /// Path for a group: `/metrics/job/<job>{/<label>/<value>}`.
    fn group_path(&self, job: &str, grouping: &LabelSet) -> io::Result<String> {
        let mut path = format!("{}/metrics/job", self.base_path);
        push_segment(&mut path, job);
        for (key, value) in grouping.iter() {
            validate_label_key(key)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;
            if key == "job" {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "the grouping key must not contain `job`",
                ));
            }
            path.push('/');
            path.push_str(key);
            push_segment(&mut path, value);
        }
        Ok(path)
    }

    fn send(&self, method: &str, job: &str, grouping: &LabelSet, body: &str) -> io::Result<()> {
        let path = self.group_path(job, grouping)?;
        let addr = (
            self.host.trim_start_matches('[').trim_end_matches(']'),
            self.port,
        )
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    "pushgateway host resolved to nothing",
                )
            })?;
        let mut stream = TcpStream::connect_timeout(&addr, self.timeout)?;
        stream.set_read_timeout(Some(self.timeout))?;
        stream.set_write_timeout(Some(self.timeout))?;
        let request = format!(
            "{method} {path} HTTP/1.1\r\nHost: {}:{}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            self.host,
            self.port,
            prometheus::CONTENT_TYPE,
            body.len()
        );
        stream.write_all(request.as_bytes())?;
        stream.write_all(body.as_bytes())?;
        stream.flush()?;
        let mut response = Vec::new();
        let _ = stream.take(64 * 1024).read_to_end(&mut response)?;
        let text = String::from_utf8_lossy(&response);
        let status_line = text.lines().next().unwrap_or_default();
        let code = status_line.split_whitespace().nth(1).unwrap_or_default();
        if code.starts_with('2') {
            Ok(())
        } else {
            let body_start = text.split("\r\n\r\n").nth(1).unwrap_or_default();
            let snippet: String = body_start.chars().take(256).collect();
            Err(io::Error::other(format!(
                "pushgateway answered {status_line:?}: {snippet}"
            )))
        }
    }
}

/// Append `/<value>` or `@base64/<encoded>` for a grouping-key value.
fn push_segment(path: &mut String, value: &str) {
    let plain = !value.is_empty()
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~'));
    if plain {
        path.push('/');
        path.push_str(value);
    } else {
        path.push_str("@base64/");
        if value.is_empty() {
            // The gateway's spelling of an empty value.
            path.push('=');
        } else {
            base64url(path, value.as_bytes());
        }
    }
}

/// RFC 4648 URL-safe base64 with padding.
fn base64url(out: &mut String, data: &[u8]) {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    for chunk in data.chunks(3) {
        let b = [
            chunk[0],
            chunk.get(1).copied().unwrap_or(0),
            chunk.get(2).copied().unwrap_or(0),
        ];
        let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
        let sextet = |shift: u32| char::from(ALPHABET[((n >> shift) & 63) as usize]);
        out.push(sextet(18));
        out.push(sextet(12));
        out.push(if chunk.len() > 1 { sextet(6) } else { '=' });
        out.push(if chunk.len() > 2 { sextet(0) } else { '=' });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;

    fn get(addr: SocketAddr, request: &str) -> String {
        let mut stream = TcpStream::connect(addr).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        stream.write_all(request.as_bytes()).unwrap();
        let mut response = String::new();
        let _ = stream.read_to_string(&mut response).unwrap();
        response
    }

    #[cfg(feature = "count")]
    #[test]
    fn test_metrics_server_serves_both_formats_and_errors() {
        let core = Arc::new(MetricsCore::new());
        core.counter("served").inc();
        let server = MetricsServer::spawn("127.0.0.1:0", Arc::clone(&core)).unwrap();
        let addr = server.local_addr();
        assert!(format!("{server:?}").contains("MetricsServer"));

        let prom = get(addr, "GET /metrics HTTP/1.1\r\nHost: x\r\n\r\n");
        assert!(prom.starts_with("HTTP/1.1 200 OK"), "{prom}");
        assert!(prom.contains(prometheus::CONTENT_TYPE));
        assert!(prom.contains("served 1"));

        let om = get(
            addr,
            "GET /metrics?x=1 HTTP/1.1\r\nAccept: application/openmetrics-text; version=1.0.0\r\n\r\n",
        );
        assert!(om.contains(openmetrics::CONTENT_TYPE));
        assert!(om.contains("served_total 1"));
        assert!(om.ends_with("# EOF\n"));

        let head = get(addr, "HEAD /metrics HTTP/1.1\r\n\r\n");
        assert!(head.starts_with("HTTP/1.1 200 OK"));
        assert!(!head.contains("served"));

        assert!(get(addr, "GET /health HTTP/1.1\r\n\r\n").ends_with("ok\n"));
        assert!(get(addr, "GET /nope HTTP/1.1\r\n\r\n").starts_with("HTTP/1.1 404"));
        let post = get(addr, "POST /metrics HTTP/1.1\r\n\r\n");
        assert!(post.starts_with("HTTP/1.1 405"));
        assert!(post.contains("Allow: GET, HEAD"));
        server.shutdown();
    }

    #[test]
    fn test_pushgateway_url_parsing() {
        let g = Pushgateway::new("http://gateway:9091").unwrap();
        assert_eq!(
            (g.host.as_str(), g.port, g.base_path.as_str()),
            ("gateway", 9091, "")
        );
        let g = Pushgateway::new("http://gateway/prefix/").unwrap();
        assert_eq!((g.port, g.base_path.as_str()), (80, "/prefix"));
        let g = Pushgateway::new("http://[::1]:9091").unwrap();
        assert_eq!((g.host.as_str(), g.port), ("[::1]", 9091));
        let g = Pushgateway::new("http://[::1]").unwrap();
        assert_eq!((g.host.as_str(), g.port), ("[::1]", 80));
        for bad in ["https://x", "gateway:9091", "http://", "http://x:notaport"] {
            assert!(Pushgateway::new(bad).is_err(), "{bad}");
        }
        assert_eq!(
            Pushgateway::new("http://x")
                .unwrap()
                .with_timeout(Duration::from_secs(1))
                .timeout,
            Duration::from_secs(1)
        );
    }

    #[test]
    fn test_group_path_encoding() {
        let g = Pushgateway::new("http://gw:9091").unwrap();
        let path = g
            .group_path(
                "backup",
                &LabelSet::from([("instance", "db-1"), ("path", "/var/lib"), ("empty", "")]),
            )
            .unwrap();
        assert_eq!(
            path,
            "/metrics/job/backup/empty@base64/=/instance/db-1/path@base64/L3Zhci9saWI="
        );
        assert!(
            g.group_path("j", &LabelSet::from([("bad key", "v")]))
                .is_err()
        );
        assert!(g.group_path("j", &LabelSet::from([("job", "v")])).is_err());
        assert_eq!(
            g.group_path("a/b", &LabelSet::EMPTY).unwrap(),
            "/metrics/job@base64/YS9i"
        );
    }

    #[test]
    fn test_base64url_vectors() {
        for (input, want) in [
            ("", ""),
            ("f", "Zg=="),
            ("fo", "Zm8="),
            ("foo", "Zm9v"),
            ("foob", "Zm9vYg=="),
            ("\u{fb}\u{ff}", "w7vDvw=="),
        ] {
            let mut out = String::new();
            base64url(&mut out, input.as_bytes());
            assert_eq!(out, want, "{input:?}");
        }
    }

    fn fake_gateway(status: &'static str) -> (SocketAddr, JoinHandle<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut head = String::new();
            let mut content_length = 0usize;
            loop {
                let mut line = String::new();
                let _ = reader.read_line(&mut line).unwrap();
                if let Some(v) = line.strip_prefix("Content-Length: ") {
                    content_length = v.trim().parse().unwrap();
                }
                head.push_str(&line);
                if line == "\r\n" {
                    break;
                }
            }
            let mut body = vec![0u8; content_length];
            reader.read_exact(&mut body).unwrap();
            let response = format!("HTTP/1.1 {status}\r\nContent-Length: 4\r\n\r\nnope");
            stream.write_all(response.as_bytes()).unwrap();
            format!("{head}{}", String::from_utf8(body).unwrap())
        });
        (addr, handle)
    }

    #[cfg(feature = "count")]
    #[test]
    fn test_pushgateway_push_sends_exposition() {
        let (addr, handle) = fake_gateway("200 OK");
        let gw = Pushgateway::new(&format!("http://{addr}")).unwrap();
        let r = Registry::new();
        r.counter("pushed").add(4);
        gw.push("job1", &LabelSet::from([("instance", "a")]), &r)
            .unwrap();
        let request = handle.join().unwrap();
        assert!(
            request.starts_with("PUT /metrics/job/job1/instance/a HTTP/1.1\r\n"),
            "{request}"
        );
        assert!(request.contains("pushed 4"));
    }

    #[test]
    fn test_pushgateway_error_status_is_reported() {
        let (addr, handle) = fake_gateway("400 Bad Request");
        let gw = Pushgateway::new(&format!("http://{addr}")).unwrap();
        let err = gw
            .push_add("job1", &LabelSet::EMPTY, &Registry::new())
            .unwrap_err();
        assert!(err.to_string().contains("400"), "{err}");
        assert!(handle.join().unwrap().starts_with("POST "));
        let (addr, handle) = fake_gateway("202 Accepted");
        let gw = Pushgateway::new(&format!("http://{addr}")).unwrap();
        gw.delete("job1", &LabelSet::EMPTY).unwrap();
        assert!(handle.join().unwrap().starts_with("DELETE "));
    }
}
