use clap::Parser;

#[tokio::main]
async fn main() {
    let args = fhevm_listener::cmd::Args::parse();
    fhevm_listener::cmd::main(args).await;
}
