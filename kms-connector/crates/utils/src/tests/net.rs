use tokio::{net::TcpListener, task::JoinHandle};

/// Starts a socket that accepts connections and never answers them: a request sent to it can only
/// end on a deadline of the caller's own.
///
/// Returns the URL of the socket, and the handle of its accept loop to abort.
pub async fn black_hole_server() -> (String, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let accept_loop = tokio::spawn(async move {
        // The accepted streams are kept alive, and left unanswered, until the caller aborts us.
        let mut accepted = Vec::new();
        while let Ok((stream, _)) = listener.accept().await {
            accepted.push(stream);
        }
    });
    (url, accept_loop)
}
