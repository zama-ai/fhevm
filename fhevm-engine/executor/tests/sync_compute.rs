use executor::server::executor::{fhevm_executor_client::FhevmExecutorClient, SyncComputeRequest};
use utils::TestInstance;

mod utils;

#[tokio::test]
async fn compute_on_ciphertexts() -> Result<(), Box<dyn std::error::Error>> {
    let test_instance = TestInstance::new();
    let mut client = FhevmExecutorClient::connect(test_instance.server_addr).await?;
    let resp = client.sync_compute(SyncComputeRequest::default()).await?;
    Ok(())
}
