//! A recording stub endpoint over plain HTTP/1.1.

use bytes::Bytes;
use http::{HeaderMap, Method, Request, Response, StatusCode, header::HOST};
use http_body_util::{BodyExt, Full};
use hyper::{body::Incoming, server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use std::{
    convert::Infallible,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::net::TcpListener;

/// A request the stub read in full.
#[derive(Clone, Debug)]
pub struct Recorded {
    pub method: Method,
    pub target: String,
    pub headers: HeaderMap,
    pub body: Bytes,
}

/// The response the stub gives every request.
#[derive(Clone, Debug)]
pub struct Canned {
    pub status: StatusCode,
    pub headers: Vec<(&'static str, &'static str)>,
    pub body: Bytes,
}

impl Default for Canned {
    fn default() -> Self {
        Self {
            status: StatusCode::OK,
            headers: Vec::new(),
            body: Bytes::new(),
        }
    }
}

/// What a stub received, without the background health checks.
#[derive(Default)]
struct Log {
    /// The target of each request whose head arrived.
    heads: Vec<String>,
    /// The requests read in full.
    requests: Vec<Recorded>,
}

#[derive(Default)]
struct Behaviour {
    canned: Canned,
    hold: Duration,
}

/// A stub endpoint recording what it receives. It answers with its [`Canned`] response after its
/// hold time: `200` with an empty body at once, until told otherwise. Health checks always get
/// `200` at once.
pub struct Stub {
    pub addr: SocketAddr,
    log: Arc<Mutex<Log>>,
    behaviour: Arc<Mutex<Behaviour>>,
}

impl Stub {
    pub async fn start() -> Self {
        let log = Arc::new(Mutex::new(Log::default()));
        let behaviour = Arc::new(Mutex::new(Behaviour::default()));
        let (recorder, answers) = (log.clone(), behaviour.clone());
        let addr = serve(move |request| {
            let (recorder, answers) = (recorder.clone(), answers.clone());
            async move {
                let (parts, body) = request.into_parts();
                // Pingora's health checks are the only requests with an empty `Host`: they are
                // answered `200` and never recorded, whatever the probed path.
                if parts.headers.get(HOST).is_some_and(|host| host.is_empty()) {
                    return Response::new(Full::default());
                }
                let target = parts.uri.to_string();
                recorder.lock().unwrap().heads.push(target.clone());
                // A request cut short is not recorded.
                if let Ok(body) = body.collect().await {
                    recorder.lock().unwrap().requests.push(Recorded {
                        method: parts.method,
                        target,
                        headers: parts.headers,
                        body: body.to_bytes(),
                    });
                }
                let (canned, hold) = {
                    let answers = answers.lock().unwrap();
                    (answers.canned.clone(), answers.hold)
                };
                tokio::time::sleep(hold).await;
                let mut response = Response::new(Full::new(canned.body));
                *response.status_mut() = canned.status;
                for (name, value) in canned.headers {
                    response.headers_mut().insert(name, value.parse().unwrap());
                }
                response
            }
        })
        .await;
        Self {
            addr,
            log,
            behaviour,
        }
    }

    /// The targets of the requests whose head reached the stub, complete or not.
    pub fn heads(&self) -> Vec<String> {
        self.log.lock().unwrap().heads.clone()
    }

    /// The requests the stub read in full.
    pub fn requests(&self) -> Vec<Recorded> {
        self.log.lock().unwrap().requests.clone()
    }

    pub fn hold(&self, hold: Duration) {
        self.behaviour.lock().unwrap().hold = hold;
    }

    pub fn answer(&self, canned: Canned) {
        self.behaviour.lock().unwrap().canned = canned;
    }
}

/// Serves `handler` over plain HTTP/1.1 on a free local port.
pub(super) async fn serve<F, Fut>(handler: F) -> SocketAddr
where
    F: Fn(Request<Incoming>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Response<Full<Bytes>>> + Send,
{
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let handler = handler.clone();
            let service = service_fn(move |request| {
                let handler = handler.clone();
                async move { Ok::<_, Infallible>(handler(request).await) }
            });
            tokio::spawn(http1::Builder::new().serve_connection(TokioIo::new(stream), service));
        }
    });
    addr
}
