use std::error::Error;

use executor::{
    fhevm_executor_server::{FhevmExecutor, FhevmExecutorServer},
    SyncComputeRequest, SyncComputeResponse,
};
use tonic::{transport::Server, Request, Response};

mod common {
    tonic::include_proto!("fhevm.common");
}

pub mod executor {
    tonic::include_proto!("fhevm.executor");
}

pub fn start(args: &crate::cli::Args) -> Result<(), Box<dyn Error>> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(args.tokio_threads)
        .max_blocking_threads(args.fhe_compute_threads)
        .enable_all()
        .build()?;

    let executor = FhevmExecutorService::default();
    let addr = args.server_addr.parse().expect("server address");

    runtime.block_on(async {
        Server::builder()
            .add_service(FhevmExecutorServer::new(executor))
            .serve(addr)
            .await?;
        Ok::<(), Box<dyn Error>>(())
    })?;
    Ok(())
}

#[derive(Default)]
pub struct FhevmExecutorService {}

#[tonic::async_trait]
impl FhevmExecutor for FhevmExecutorService {
    async fn sync_compute(
        &self,
        req: Request<SyncComputeRequest>,
    ) -> Result<Response<SyncComputeResponse>, tonic::Status> {
        Ok(Response::new(SyncComputeResponse::default()))
    }
}
