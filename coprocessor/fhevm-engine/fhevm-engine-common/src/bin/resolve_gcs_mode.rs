//! Prints `true` when this image is a green (GCS) deploy, `false` otherwise.
//!
//! Used by `initialize_db.sh` to decide whether it owns migrating `public`. A
//! green deploy must not: the upgrade-controller applies the delta at cutover,
//! in the same transaction that retires the blue stack.

use fhevm_engine_common::database::resolve_database_url_from_option;
use fhevm_engine_common::versioning::resolve_gcs_mode;

#[tokio::main]
async fn main() {
    let database_url = match resolve_database_url_from_option(None) {
        Ok(database_url) => database_url,
        Err(err) => {
            eprintln!("failed to resolve DATABASE_URL: {err}");
            std::process::exit(1);
        }
    };
    match resolve_gcs_mode(database_url.as_str()).await {
        Ok(gcs_mode) => println!("{gcs_mode}"),
        Err(err) => {
            eprintln!("failed to resolve gcs mode: {err}");
            std::process::exit(1);
        }
    }
}
