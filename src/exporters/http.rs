//! Dependency-free HTTP helpers (feature `http`): a `/metrics` endpoint and
//! a Prometheus Pushgateway client, both plain HTTP/1.1 over `std::net`.
//!
//! These are for services that do not already run an HTTP stack. If yours
//! does (axum, actix, hyper, …), serve
//! [`prometheus::render_core`] from a route
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
use std::io::{self, Read, Write};
use std::net::{
    IpAddr, Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr, TcpListener, TcpStream, ToSocketAddrs,
};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

/// Largest request head (request line plus headers) the server accepts.
/// Longer heads are answered `431`.
pub const MAX_REQUEST_HEAD: usize = 8 * 1024;
/// Time a client has, from the moment its connection is accepted, to send
/// the complete request head. Slower clients are answered `408`. Scrapers
/// send their request at once, so this is short.
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(2);
/// Time the server spends writing one response before giving up on a
/// client that does not read it.
pub const RESPONSE_TIMEOUT: Duration = Duration::from_secs(10);
/// Connections served at the same time. When all are taken, a new
/// connection replaces the oldest one that has not finished sending its
/// request; only when every slot is writing a response is it answered
/// `503`.
pub const MAX_CONNECTIONS: usize = 16;

/// Limits of one server; the public constants are the defaults.
#[derive(Clone, Copy)]
struct Limits {
    request_timeout: Duration,
    response_timeout: Duration,
    max_connections: usize,
}

const DEFAULT_LIMITS: Limits = Limits {
    request_timeout: REQUEST_TIMEOUT,
    response_timeout: RESPONSE_TIMEOUT,
    max_connections: MAX_CONNECTIONS,
};

/// A blocking HTTP server for `GET /metrics` and `GET /health`.
///
/// `/metrics` answers with the OpenMetrics format when the `Accept` header
/// asks for `application/openmetrics-text`, and with the Prometheus format
/// otherwise; it includes the `process_*` families when the core has them
/// enabled. `/health` answers `ok`. Other paths get 404, other methods 405.
///
/// One thread accepts connections and hands each to a short-lived worker
/// thread, at most [`MAX_CONNECTIONS`] at a time, so a slow or idle client
/// cannot delay other scrapes. Each client gets [`REQUEST_TIMEOUT`] from
/// accept to send its request head (at most [`MAX_REQUEST_HEAD`] bytes) and
/// [`RESPONSE_TIMEOUT`] to read the response; both are total deadlines, not
/// per-read timeouts, so a client dripping bytes is cut off too. When every
/// slot is taken, a new connection evicts the oldest connection still
/// waiting for its request, so idle connections cannot lock scrapers out.
/// A client that opens connections faster than the server reads requests
/// (more than [`MAX_CONNECTIONS`] while one scrape is in flight) can still
/// push others out; keep the endpoint on a network you control.
pub struct MetricsServer {
    local_addr: SocketAddr,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl MetricsServer {
    /// Bind `addr` and serve `core` on background threads.
    ///
    /// # Errors
    ///
    /// Returns the bind error, or the error from configuring the listener or
    /// spawning the accept thread.
    pub fn spawn(addr: impl ToSocketAddrs, core: Arc<MetricsCore>) -> io::Result<Self> {
        Self::spawn_with(addr, core, DEFAULT_LIMITS)
    }

    fn spawn_with(
        addr: impl ToSocketAddrs,
        core: Arc<MetricsCore>,
        limits: Limits,
    ) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        let local_addr = listener.local_addr()?;
        let stop = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&stop);
        let thread = thread::Builder::new()
            .name("metrics-lib-http".into())
            .spawn(move || accept_loop(&listener, &core, &flag, limits))?;
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

    /// Stop accepting connections and wait for the accept thread to exit.
    /// Requests already being served finish on their own threads within
    /// their deadlines.
    pub fn shutdown(mut self) {
        self.stop_and_join();
    }

    fn stop_and_join(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(thread) = self.thread.take() {
            // Wake the blocking `accept` with a connection of our own. If
            // that fails the listener is already broken and the thread has
            // left its loop.
            let _ = TcpStream::connect_timeout(&wake_addr(self.local_addr), Duration::from_secs(1));
            // A panic in the server thread has nothing left to clean up.
            let _ = thread.join();
        }
    }
}

/// Where to connect to reach a listener bound to `addr` (an unspecified
/// address is reachable through loopback).
fn wake_addr(addr: SocketAddr) -> SocketAddr {
    let ip = match addr.ip() {
        IpAddr::V4(v4) if v4.is_unspecified() => IpAddr::V4(Ipv4Addr::LOCALHOST),
        IpAddr::V6(v6) if v6.is_unspecified() => IpAddr::V6(Ipv6Addr::LOCALHOST),
        ip => ip,
    };
    SocketAddr::new(ip, addr.port())
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

/// Decrements a refusal count when a refusing thread ends.
struct Slot(Arc<AtomicUsize>);

impl Drop for Slot {
    fn drop(&mut self) {
        let _ = self.0.fetch_sub(1, Ordering::AcqRel);
    }
}

/// One served connection: the worker's handle on the shared table.
struct Conn {
    id: u64,
    /// Still reading the request head (may be evicted).
    reading: bool,
    /// A second handle on the socket, used to shut down an evicted client.
    stream: TcpStream,
}

/// Connections being served, oldest first.
#[derive(Default)]
struct Conns {
    next_id: u64,
    list: Vec<Conn>,
}

/// Shared table plus the worker's own id; removes the entry when dropped.
struct ConnGuard {
    conns: Arc<Mutex<Conns>>,
    id: u64,
}

impl ConnGuard {
    /// Mark the request head as read: from now on the connection is not
    /// evicted.
    fn serving(&self) {
        let mut conns = self.conns.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(c) = conns.list.iter_mut().find(|c| c.id == self.id) {
            c.reading = false;
        }
    }
}

impl Drop for ConnGuard {
    fn drop(&mut self) {
        let mut conns = self.conns.lock().unwrap_or_else(PoisonError::into_inner);
        conns.list.retain(|c| c.id != self.id);
    }
}

/// Register `stream`, evicting the oldest connection still reading its
/// request when the table is full. `None` when every slot is serving.
fn admit(conns: &Arc<Mutex<Conns>>, stream: &TcpStream, max: usize) -> Option<ConnGuard> {
    let mut table = conns.lock().unwrap_or_else(PoisonError::into_inner);
    if table.list.len() >= max {
        let oldest_idle = table.list.iter().position(|c| c.reading)?;
        let evicted = table.list.remove(oldest_idle);
        // Its worker sees the read fail and exits; nothing to report to a
        // client that never sent a request.
        let _ = evicted.stream.shutdown(Shutdown::Both);
    }
    let handle = stream.try_clone().ok()?;
    let id = table.next_id;
    table.next_id += 1;
    table.list.push(Conn {
        id,
        reading: true,
        stream: handle,
    });
    Some(ConnGuard {
        conns: Arc::clone(conns),
        id,
    })
}

fn accept_loop(listener: &TcpListener, core: &Arc<MetricsCore>, stop: &AtomicBool, limits: Limits) {
    let conns = Arc::new(Mutex::new(Conns::default()));
    let refusing = Arc::new(AtomicUsize::new(0));
    loop {
        let accepted = listener.accept();
        if stop.load(Ordering::Acquire) {
            return;
        }
        let Ok((stream, _)) = accepted else {
            // Accept errors (for example a reset during the handshake or a
            // full descriptor table) only affect that connection; back off
            // briefly so a persistent error cannot spin the thread.
            thread::sleep(Duration::from_millis(10));
            continue;
        };
        let accepted_at = Instant::now();
        let Some(guard) = admit(&conns, &stream, limits.max_connections) else {
            // Every slot is writing a response. Answer 503 off the accept
            // thread, with its own bound; past it the connection is closed.
            if refusing.fetch_add(1, Ordering::AcqRel) < limits.max_connections {
                let slot = Slot(Arc::clone(&refusing));
                let _ = thread::Builder::new()
                    .name("metrics-lib-http-busy".into())
                    .spawn(move || {
                        let _slot = slot;
                        refuse(stream, "503 Service Unavailable");
                    });
            } else {
                let _ = refusing.fetch_sub(1, Ordering::AcqRel);
            }
            continue;
        };
        let core = Arc::clone(core);
        let spawned = thread::Builder::new()
            .name("metrics-lib-http-conn".into())
            .spawn(move || {
                // A failed exchange only affects that client.
                let _ = serve_connection(stream, &core, accepted_at, limits, &guard);
            });
        if spawned.is_err() {
            // The closure (and with it the stream and the guard) was dropped.
            thread::sleep(Duration::from_millis(10));
        }
    }
}

/// Answer `status` with an empty body and close.
fn refuse(mut stream: TcpStream, status: &str) {
    let response = format!("HTTP/1.1 {status}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
    if write_all_by(&mut stream, response.as_bytes(), Instant::now() + LINGER).is_ok() {
        close_gracefully(&mut stream);
    }
}

/// How long a closing connection is drained.
const LINGER: Duration = Duration::from_millis(250);

/// Half-close, then discard what the client still sends (at most 64 KiB or
/// [`LINGER`]). Closing a socket with unread input resets the connection,
/// which can destroy the response before the client has read it.
fn close_gracefully(stream: &mut TcpStream) {
    let _ = stream.shutdown(Shutdown::Write);
    let deadline = Instant::now() + LINGER;
    let mut buf = [0u8; 4096];
    let mut drained = 0;
    while drained < 64 * 1024 {
        let Some(left) = remaining(deadline) else {
            break;
        };
        if stream.set_read_timeout(Some(left)).is_err() {
            break;
        }
        match stream.read(&mut buf) {
            Ok(0) | Err(_) => break,
            Ok(n) => drained += n,
        }
    }
}

struct Request {
    method: String,
    path: String,
    accepts_openmetrics: bool,
}

/// Why a request head could not be read.
enum HeadError {
    TooLarge,
    TimedOut,
    Io,
}

/// Time left until `deadline`, or `None` when it has passed.
fn remaining(deadline: Instant) -> Option<Duration> {
    deadline
        .checked_duration_since(Instant::now())
        .filter(|d| !d.is_zero())
}

/// Read until the blank line that ends the head, within `deadline`.
fn read_head(stream: &mut TcpStream, deadline: Instant) -> Result<Vec<u8>, HeadError> {
    let mut head = Vec::with_capacity(512);
    let mut buf = [0u8; 1024];
    loop {
        let Some(left) = remaining(deadline) else {
            return Err(HeadError::TimedOut);
        };
        stream
            .set_read_timeout(Some(left))
            .map_err(|_| HeadError::Io)?;
        let n = match stream.read(&mut buf) {
            Ok(0) => return Err(HeadError::Io),
            Ok(n) => n,
            Err(e)
                if matches!(
                    e.kind(),
                    io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
                ) =>
            {
                return Err(HeadError::TimedOut);
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(_) => return Err(HeadError::Io),
        };
        let scan_from = head.len().saturating_sub(3);
        head.extend_from_slice(&buf[..n]);
        if let Some(end) = find_head_end(&head[scan_from..]) {
            head.truncate(scan_from + end);
            return Ok(head);
        }
        if head.len() > MAX_REQUEST_HEAD {
            return Err(HeadError::TooLarge);
        }
    }
}

/// Offset just past the first `\r\n\r\n` or `\n\n` in `bytes`.
fn find_head_end(bytes: &[u8]) -> Option<usize> {
    (0..bytes.len()).find_map(|i| {
        if bytes[i..].starts_with(b"\r\n\r\n") {
            Some(i + 4)
        } else if bytes[i..].starts_with(b"\n\n") {
            Some(i + 2)
        } else {
            None
        }
    })
}

fn parse_head(head: &[u8]) -> Request {
    let text = String::from_utf8_lossy(head);
    let mut lines = text.lines();
    let mut parts = lines.next().unwrap_or_default().split_whitespace();
    let method = parts.next().unwrap_or_default().to_owned();
    let target = parts.next().unwrap_or_default();
    let path = target.split('?').next().unwrap_or_default().to_owned();
    let accepts_openmetrics = lines.any(|line| {
        line.split_once(':').is_some_and(|(name, value)| {
            name.trim().eq_ignore_ascii_case("accept")
                && value.contains("application/openmetrics-text")
        })
    });
    Request {
        method,
        path,
        accepts_openmetrics,
    }
}

/// Write all of `bytes` before `deadline`.
fn write_all_by(stream: &mut TcpStream, mut bytes: &[u8], deadline: Instant) -> io::Result<()> {
    while !bytes.is_empty() {
        let left = remaining(deadline)
            .ok_or_else(|| io::Error::new(io::ErrorKind::TimedOut, "response deadline passed"))?;
        stream.set_write_timeout(Some(left))?;
        match stream.write(bytes) {
            Ok(0) => return Err(io::Error::new(io::ErrorKind::WriteZero, "client closed")),
            Ok(n) => bytes = &bytes[n..],
            Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn serve_connection(
    mut stream: TcpStream,
    core: &MetricsCore,
    accepted_at: Instant,
    limits: Limits,
    guard: &ConnGuard,
) -> io::Result<()> {
    stream.set_nonblocking(false)?;
    let head = match read_head(&mut stream, accepted_at + limits.request_timeout) {
        Ok(head) => head,
        Err(HeadError::TooLarge) => {
            refuse(stream, "431 Request Header Fields Too Large");
            return Ok(());
        }
        Err(HeadError::TimedOut) => {
            refuse(stream, "408 Request Timeout");
            return Ok(());
        }
        Err(HeadError::Io) => return Ok(()),
    };
    guard.serving();
    let request = parse_head(&head);
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
    let mut response = String::with_capacity(160 + body.len());
    let _ = write!(
        response,
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n",
        body.len()
    );
    if status.starts_with("405") {
        response.push_str("Allow: GET, HEAD\r\n");
    }
    response.push_str("\r\n");
    if request.method != "HEAD" {
        response.push_str(&body);
    }
    write_all_by(
        &mut stream,
        response.as_bytes(),
        Instant::now() + limits.response_timeout,
    )?;
    close_gracefully(&mut stream);
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

    /// Set the timeout of one request: connecting (to every resolved
    /// address in turn), sending and reading the answer must all finish
    /// within it. Name resolution is not covered.
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
    /// Returns an `InvalidInput` error for an invalid grouping-label key,
    /// `TimedOut` when the request did not finish within the timeout, the
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
        let deadline = Instant::now() + self.timeout;
        // Name resolution uses the system resolver and is not bounded by the
        // timeout.
        let addrs: Vec<SocketAddr> = (
            self.host.trim_start_matches('[').trim_end_matches(']'),
            self.port,
        )
            .to_socket_addrs()?
            .collect();
        let mut stream = connect_any(&addrs, deadline)?;
        let request = format!(
            "{method} {path} HTTP/1.1\r\nHost: {}:{}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            self.host,
            self.port,
            prometheus::CONTENT_TYPE,
            body.len()
        );
        write_all_by(&mut stream, request.as_bytes(), deadline)?;
        write_all_by(&mut stream, body.as_bytes(), deadline)?;
        let response = read_response_by(&mut stream, deadline, 64 * 1024)?;
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

fn timed_out() -> io::Error {
    io::Error::new(io::ErrorKind::TimedOut, "pushgateway request timed out")
}

/// Connect to the first of `addrs` that accepts before `deadline`, trying
/// them in order, each with an equal share of the time left.
fn connect_any(addrs: &[SocketAddr], deadline: Instant) -> io::Result<TcpStream> {
    let mut last = io::Error::new(
        io::ErrorKind::NotFound,
        "pushgateway host resolved to nothing",
    );
    for (i, addr) in addrs.iter().enumerate() {
        let Some(left) = remaining(deadline) else {
            return Err(timed_out());
        };
        // Share what is left fairly among the addresses not tried yet, so
        // one unreachable address (an IPv6 `localhost` with an IPv4-only
        // gateway) cannot use up the whole budget.
        let untried = u32::try_from(addrs.len() - i).unwrap_or(u32::MAX);
        match TcpStream::connect_timeout(addr, left / untried) {
            Ok(stream) => return Ok(stream),
            Err(e) => last = e,
        }
    }
    if last.kind() == io::ErrorKind::WouldBlock {
        return Err(timed_out());
    }
    Err(last)
}

/// Read until the peer closes, at most `limit` bytes, before `deadline`.
fn read_response_by(
    stream: &mut TcpStream,
    deadline: Instant,
    limit: usize,
) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();
    let mut buf = [0u8; 4096];
    while out.len() < limit {
        let left = remaining(deadline).ok_or_else(timed_out)?;
        stream.set_read_timeout(Some(left))?;
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => out.extend_from_slice(&buf[..n.min(limit - out.len())]),
            Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e)
                if matches!(
                    e.kind(),
                    io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
                ) =>
            {
                return Err(timed_out());
            }
            Err(e) => return Err(e),
        }
    }
    Ok(out)
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
    use std::io::{BufRead, BufReader};
    use std::net::TcpListener;

    fn fast_limits() -> Limits {
        Limits {
            request_timeout: Duration::from_millis(300),
            response_timeout: Duration::from_millis(300),
            max_connections: 4,
        }
    }

    #[test]
    fn test_idle_and_dripping_clients_do_not_delay_scrapes_regression_ml2_002() {
        let core = Arc::new(MetricsCore::new());
        let server = MetricsServer::spawn("127.0.0.1:0", Arc::clone(&core)).unwrap();
        let addr = server.local_addr();
        // An idle connection and a client dripping its head byte by byte.
        let _idle = TcpStream::connect(addr).unwrap();
        let mut drip = TcpStream::connect(addr).unwrap();
        drip.write_all(b"GET /met").unwrap();
        let start = Instant::now();
        let response = get(addr, "GET /health HTTP/1.1\r\n\r\n");
        assert!(response.ends_with("ok\n"), "{response}");
        assert!(
            start.elapsed() < Duration::from_secs(2),
            "{:?}",
            start.elapsed()
        );
        server.shutdown();
    }

    #[test]
    fn test_slow_clients_get_408_at_the_total_deadline() {
        let core = Arc::new(MetricsCore::new());
        let server = MetricsServer::spawn_with("127.0.0.1:0", core, fast_limits()).unwrap();
        let mut drip = TcpStream::connect(server.local_addr()).unwrap();
        drip.set_read_timeout(Some(Duration::from_millis(20)))
            .unwrap();
        let start = Instant::now();
        // Send a byte more often than any per-read timeout would allow,
        // until the server answers; stop sending then, so closing the
        // connection cannot reset it before the answer is read.
        let mut response = Vec::new();
        let mut buf = [0u8; 256];
        let mut sending = true;
        while start.elapsed() < Duration::from_secs(5) {
            if sending && drip.write_all(b"a").is_err() {
                sending = false;
            }
            match drip.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    response.extend_from_slice(&buf[..n]);
                    sending = false;
                }
                Err(_) => {}
            }
        }
        let response = String::from_utf8_lossy(&response);
        assert!(response.starts_with("HTTP/1.1 408"), "{response:?}");
        // The deadline is 300 ms from accept; per-read timeouts would never
        // have fired while bytes kept arriving.
        assert!(start.elapsed() < Duration::from_secs(3));
        server.shutdown();
    }

    #[test]
    fn test_oversized_heads_get_431() {
        let core = Arc::new(MetricsCore::new());
        let server = MetricsServer::spawn("127.0.0.1:0", core).unwrap();
        let mut stream = TcpStream::connect(server.local_addr()).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let junk = vec![b'a'; MAX_REQUEST_HEAD + 1024];
        let _ = stream.write_all(b"GET /metrics HTTP/1.1\r\nX: ");
        let _ = stream.write_all(&junk);
        let mut response = Vec::new();
        let _ = stream.read_to_end(&mut response);
        let text = String::from_utf8_lossy(&response);
        assert!(text.starts_with("HTTP/1.1 431"), "{text:?}");
        server.shutdown();
    }

    #[test]
    fn test_idle_connections_cannot_lock_scrapers_out_regression_f1() {
        let core = Arc::new(MetricsCore::new());
        let limits = Limits {
            request_timeout: Duration::from_secs(5),
            ..fast_limits()
        };
        let server = MetricsServer::spawn_with("127.0.0.1:0", core, limits).unwrap();
        let addr = server.local_addr();
        // The re-audit's scenario: as many idle connections as the server
        // has slots, each re-opened as soon as the server drops it.
        let stop = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&stop);
        let attacker = thread::spawn(move || {
            let open = || {
                let s = TcpStream::connect(addr).unwrap();
                s.set_nonblocking(true).unwrap();
                s
            };
            let mut held: Vec<TcpStream> = (0..limits.max_connections).map(|_| open()).collect();
            let mut buf = [0u8; 64];
            while !flag.load(Ordering::Acquire) {
                for s in &mut held {
                    // Closed (evicted or timed out): open a fresh one.
                    let open_still = matches!(
                        s.read(&mut buf),
                        Err(e) if e.kind() == io::ErrorKind::WouldBlock
                    );
                    if !open_still {
                        *s = open();
                    }
                }
                thread::sleep(Duration::from_millis(10));
            }
        });
        // Let the idle connections fill every slot.
        thread::sleep(Duration::from_millis(200));
        for _ in 0..20 {
            let response = get(addr, "GET /health HTTP/1.1\r\n\r\n");
            assert!(response.ends_with("ok\n"), "{response}");
            thread::sleep(Duration::from_millis(20));
        }
        stop.store(true, Ordering::Release);
        attacker.join().unwrap();
        server.shutdown();
    }

    #[test]
    fn test_admission_evicts_the_oldest_idle_connection_and_refuses_when_all_serve() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let pair = || {
            let client = TcpStream::connect(addr).unwrap();
            let (server_side, _) = listener.accept().unwrap();
            (client, server_side)
        };
        let conns = Arc::new(Mutex::new(Conns::default()));
        let (mut first_client, first) = pair();
        let (_c2, second) = pair();
        let g1 = admit(&conns, &first, 2).unwrap();
        let g2 = admit(&conns, &second, 2).unwrap();
        // Full: the next connection evicts the oldest one still reading.
        let (_c3, third) = pair();
        let g3 = admit(&conns, &third, 2).unwrap();
        let mut buf = [0u8; 1];
        first_client
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        assert_eq!(first_client.read(&mut buf).unwrap_or(0), 0, "evicted");
        assert_eq!(conns.lock().unwrap().list.len(), 2);
        // Once every slot is serving a response, nothing can be evicted.
        g2.serving();
        g3.serving();
        let (_c4, fourth) = pair();
        assert!(admit(&conns, &fourth, 2).is_none());
        // Guards remove their entries; the evicted one is already gone.
        drop((g1, g2, g3));
        assert_eq!(conns.lock().unwrap().list.len(), 0);
    }

    #[test]
    fn test_scrape_latency_has_no_accept_polling() {
        let core = Arc::new(MetricsCore::new());
        let server = MetricsServer::spawn("127.0.0.1:0", core).unwrap();
        let addr = server.local_addr();
        let _ = get(addr, "GET /health HTTP/1.1\r\n\r\n");
        let fastest = (0..5)
            .map(|_| {
                let start = Instant::now();
                let _ = get(addr, "GET /health HTTP/1.1\r\n\r\n");
                start.elapsed()
            })
            .min()
            .unwrap();
        // An earlier accept loop polled every 50 ms.
        assert!(fastest < Duration::from_millis(40), "{fastest:?}");
        server.shutdown();
    }

    #[test]
    fn test_shutdown_of_a_server_bound_to_all_interfaces() {
        let server = MetricsServer::spawn("0.0.0.0:0", Arc::new(MetricsCore::new())).unwrap();
        assert!(server.local_addr().ip().is_unspecified());
        assert_eq!(
            wake_addr(server.local_addr()).ip(),
            IpAddr::V4(Ipv4Addr::LOCALHOST)
        );
        let start = Instant::now();
        server.shutdown();
        assert!(start.elapsed() < Duration::from_secs(2));
        let v6 = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 9);
        assert_eq!(wake_addr(v6).ip(), IpAddr::V6(Ipv6Addr::LOCALHOST));
    }

    #[test]
    fn test_head_parsing_helpers() {
        assert_eq!(find_head_end(b"GET / HTTP/1.1\r\n\r\nrest"), Some(18));
        assert_eq!(find_head_end(b"GET /\n\n"), Some(7));
        assert_eq!(find_head_end(b"GET /\r\n"), None);
        let r =
            parse_head(b"GET /metrics?x=1 HTTP/1.1\r\nACCEPT: application/openmetrics-text\r\n");
        assert_eq!((r.method.as_str(), r.path.as_str()), ("GET", "/metrics"));
        assert!(r.accepts_openmetrics);
        let past = Instant::now().checked_sub(Duration::from_secs(1));
        assert!(past.is_none_or(|p| remaining(p).is_none()));
    }

    #[test]
    fn test_pushgateway_enforces_a_total_deadline_regression_ml2_006() {
        // A gateway that reads the request, then drips its answer.
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = [0u8; 4096];
            let _ = stream.read(&mut buf);
            for _ in 0..40 {
                if stream.write_all(b"H").is_err() {
                    break;
                }
                thread::sleep(Duration::from_millis(50));
            }
        });
        let gw = Pushgateway::new(&format!("http://{addr}"))
            .unwrap()
            .with_timeout(Duration::from_millis(300));
        let start = Instant::now();
        let err = gw.delete("job", &LabelSet::EMPTY).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::TimedOut, "{err}");
        assert!(
            start.elapsed() < Duration::from_millis(1500),
            "{:?}",
            start.elapsed()
        );
        handle.join().unwrap();
    }

    #[test]
    fn test_pushgateway_tries_every_resolved_address() {
        // The first address refuses connections; the second accepts.
        let closed = TcpListener::bind("127.0.0.1:0").unwrap();
        let refused = closed.local_addr().unwrap();
        drop(closed);
        let open = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = open.local_addr().unwrap();
        let stream = connect_any(&[refused, addr], Instant::now() + Duration::from_secs(5));
        assert_eq!(stream.unwrap().peer_addr().unwrap(), addr);
        assert_eq!(
            connect_any(&[], Instant::now() + Duration::from_secs(1))
                .unwrap_err()
                .kind(),
            io::ErrorKind::NotFound
        );
    }

    #[test]
    fn test_pushgateway_unreachable_first_address_leaves_time_for_the_next_f5() {
        // 192.0.2.0/24 (TEST-NET-1) is never routed: a connect there either
        // fails at once or hangs until its timeout. With the budget split,
        // the second address still gets half of it.
        let blackhole = SocketAddr::from(([192, 0, 2, 1], 9));
        let open = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = open.local_addr().unwrap();
        let start = Instant::now();
        let stream = connect_any(
            &[blackhole, addr],
            Instant::now() + Duration::from_millis(800),
        );
        assert_eq!(stream.unwrap().peer_addr().unwrap(), addr);
        assert!(start.elapsed() < Duration::from_millis(800));
    }

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
