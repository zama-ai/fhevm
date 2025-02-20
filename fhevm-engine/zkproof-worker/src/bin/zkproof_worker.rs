use tracing::error;

#[tokio::main]
async fn main() {
    let conf = zkproof_worker::verifier::Config::default();

    println!("Starting zkProof worker...");
    if let Err(err) = zkproof_worker::verifier::execute_verify_proofs_loop(&conf).await {
        error!("Worker failed: {:?}", err);
    }
}
