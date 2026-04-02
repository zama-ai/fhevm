use fhevm_engine_common::{
    database::{resolve_database_url_from_option, resolve_runtime_database_url},
};

#[tokio::main]
async fn main() {
    let database_url = match resolve_database_url_from_option(None) {
        Ok(database_url) => database_url,
        Err(err) => {
            eprintln!("failed to resolve DATABASE_URL: {err}");
            std::process::exit(1);
        }
    };
    match resolve_runtime_database_url(&database_url).await {
        Ok(resolved_url) => {
            println!("{resolved_url}");
        }
        Err(err) => {
            eprintln!("failed to resolve DATABASE_URL: {err}");
            std::process::exit(1);
        }
    }
}
