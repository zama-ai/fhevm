use foundry_compilers::{error::SolcError, solc::Solc};
use semver::Version;
use std::{thread, time::Duration};

const SOLC_INSTALL_RETRY_ATTEMPTS: usize = 5;

fn is_etxtbsy(err: &SolcError) -> bool {
    matches!(err, SolcError::Io(io_err) if io_err.source().kind() == std::io::ErrorKind::ExecutableFileBusy || io_err.source().raw_os_error() == Some(26))
}

pub(crate) fn find_or_install_solc(version: &Version) -> Solc {
    for attempt in 1..=SOLC_INSTALL_RETRY_ATTEMPTS {
        match Solc::find_or_install(version) {
            Ok(solc) => return solc,
            Err(err) => {
                if is_etxtbsy(&err) && attempt < SOLC_INSTALL_RETRY_ATTEMPTS {
                    let backoff = Duration::from_millis((attempt as u64) * 200);
                    println!(
                        "cargo:warning=solc install race (ETXTBSY), retrying attempt {attempt}/{SOLC_INSTALL_RETRY_ATTEMPTS} after {:?}",
                        backoff
                    );
                    thread::sleep(backoff);
                } else {
                    panic!("Failed to find or install solc {version}: {err:?}");
                }
            }
        }
    }

    unreachable!("solc install retry loop exited unexpectedly")
}
