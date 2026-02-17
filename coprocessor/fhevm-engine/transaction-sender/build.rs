use std::path::Path;
use std::{thread, time::Duration};

use foundry_compilers::{
    error::SolcError,
    multi::MultiCompiler,
    solc::{Solc, SolcCompiler},
    Project, ProjectPathsConfig,
};
use semver::Version;

const SOLC_INSTALL_RETRY_ATTEMPTS: usize = 5;

fn is_etxtbsy(err: &SolcError) -> bool {
    matches!(err, SolcError::Io(io_err) if io_err.source().kind() == std::io::ErrorKind::ExecutableFileBusy || io_err.source().raw_os_error() == Some(26))
}

fn find_or_install_solc(version: &Version) -> Solc {
    for attempt in 1..=SOLC_INSTALL_RETRY_ATTEMPTS {
        match Solc::find_or_install(version) {
            Ok(solc) => return solc,
            Err(err) if is_etxtbsy(&err) && attempt < SOLC_INSTALL_RETRY_ATTEMPTS => {
                let backoff = Duration::from_millis((attempt as u64) * 200);
                println!(
                    "cargo:warning=solc install race (ETXTBSY), retrying attempt {attempt}/{SOLC_INSTALL_RETRY_ATTEMPTS} after {:?}",
                    backoff
                );
                thread::sleep(backoff);
            }
            Err(err) => panic!("Failed to find or install solc {version}: {err:?}"),
        }
    }

    unreachable!("solc install retry loop exited unexpectedly")
}

fn main() {
    let paths = ProjectPathsConfig::hardhat(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    // Use a specific version due to an issue with libc and libstdc++ in the rust Docker image we use to run it.
    let solc = find_or_install_solc(&Version::new(0, 8, 28));
    let project = Project::builder()
        .paths(paths)
        .build(MultiCompiler::new(Some(SolcCompiler::Specific(solc)), None).unwrap())
        .unwrap();

    let output = project.compile().unwrap();
    if output.has_compiler_errors() {
        panic!("Solidity compilation failed: {:#?}", output);
    }
    assert!(!output.has_compiler_errors());

    project.rerun_if_sources_changed();
}
