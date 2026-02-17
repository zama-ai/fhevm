use foundry_compilers::{error::SolcError, solc::Solc};
use semver::Version;
use std::{thread, time::Duration};

const SOLC_INSTALL_RETRY_ATTEMPTS: u64 = 5;
const SOLC_INSTALL_RETRY_BASE_BACKOFF_MS: u64 = 200;

fn is_etxtbsy(err: &SolcError) -> bool {
    matches!(err, SolcError::Io(io_err) if io_err.source().kind() == std::io::ErrorKind::ExecutableFileBusy || io_err.source().raw_os_error() == Some(26))
}

fn backoff_ms(attempt: u64) -> u64 {
    attempt * SOLC_INSTALL_RETRY_BASE_BACKOFF_MS
}

pub(crate) fn find_or_install_solc(version: &Version) -> Solc {
    let mut attempt = 1_u64;
    loop {
        match Solc::find_or_install(version) {
            Ok(solc) => return solc,
            Err(err) => {
                if is_etxtbsy(&err) && attempt < SOLC_INSTALL_RETRY_ATTEMPTS {
                    let backoff_ms = backoff_ms(attempt);
                    println!(
                        "cargo:warning=solc install race (ETXTBSY), retrying attempt {attempt}/{SOLC_INSTALL_RETRY_ATTEMPTS} after {backoff_ms}ms"
                    );
                    thread::sleep(Duration::from_millis(backoff_ms));
                    attempt += 1;
                } else {
                    panic!("Failed to find or install solc {version}: {err:?}");
                }
            }
        }
    }
}
