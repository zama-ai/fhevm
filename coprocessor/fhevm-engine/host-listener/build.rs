use foundry_compilers::{
    error::SolcError,
    multi::MultiCompiler,
    solc::{Solc, SolcCompiler},
    Project, ProjectPathsConfig,
};
use semver::Version;
use std::{env, fs, path::Path, process::Command, thread, time::Duration};

const SOLC_INSTALL_RETRY_ATTEMPTS: usize = 5;

fn is_etxtbsy(err: &SolcError) -> bool {
    matches!(err, SolcError::Io(io_err) if io_err.source().kind() == std::io::ErrorKind::ExecutableFileBusy || io_err.source().raw_os_error() == Some(26))
}

fn find_or_install_solc(version: &Version) -> Solc {
    for attempt in 1..=SOLC_INSTALL_RETRY_ATTEMPTS {
        match Solc::find_or_install(version) {
            Ok(solc) => return solc,
            Err(err)
                if is_etxtbsy(&err)
                    && attempt < SOLC_INSTALL_RETRY_ATTEMPTS =>
            {
                let backoff = Duration::from_millis((attempt as u64) * 200);
                println!(
                    "cargo:warning=solc install race (ETXTBSY), retrying attempt {attempt}/{SOLC_INSTALL_RETRY_ATTEMPTS} after {:?}",
                    backoff
                );
                thread::sleep(backoff);
            }
            Err(err) => {
                panic!("Failed to find or install solc {version}: {err:?}")
            }
        }
    }

    unreachable!("solc install retry loop exited unexpectedly")
}

fn build_contracts() {
    println!(
        "cargo:rerun-if-changed=../../../host-contracts/contracts/ACL.sol"
    );
    println!(
        "cargo:rerun-if-changed=../../../host-contracts/contracts/ACLEvents.sol"
    );
    println!("cargo:rerun-if-changed=../../../host-contracts/contracts/FHEVMExecutor.sol");
    // Step 1: Copy ../../contracts/.env.example to ../../contracts/.env
    let env_example = Path::new("../../../host-contracts/.env.example");
    let env_dest = Path::new("../../../host-contracts/.env");
    let artefacts = Path::new("../../../host-contracts/artifacts");
    if env_example.exists() {
        // CI build
        if !env_dest.exists() {
            fs::copy(env_example, env_dest)
                .expect("Failed to copy .env.example to .env");
            println!("Copied .env.example to .env");
        }
    } else if artefacts.exists() {
        // Docker build
        println!("Assuming artefacts are up to date.");
        return;
    } else {
        panic!("Error: .env.example not found in contracts directory");
    }

    // Change to the contracts directory for npm commands.
    let contracts_dir = Path::new("../../../host-contracts");
    if !contracts_dir.exists() {
        panic!("Error: contracts directory not found");
    }
    env::set_current_dir(contracts_dir)
        .expect("Failed to change to contracts directory");

    // Step 2: Run `npm ci --include=optional` in ../../contracts
    let npm_ci_status = Command::new("npm")
        .args(["ci", "--include=optional"])
        .status()
        .expect("Failed to run npm ci");
    if !npm_ci_status.success() {
        panic!("Error: npm ci failed");
    }
    println!("Ran npm ci successfully");

    // Step 3: Run `HARDHAT_NETWORK=hardhat npm run deploy:emptyProxies
    // && npx hardhat compile` in ../../contracts
    let npm_run_status = Command::new("npm")
        .env("HARDHAT_NETWORK", "hardhat")
        .args(["run", "deploy:emptyProxies"])
        .status()
        .expect("Failed to run npm run");
    if !npm_run_status.success() {
        panic!("Error: npm tun failed");
    }
    println!("Ran npm run successfully");

    let hardhat_compile_status = Command::new("npx")
        .args(["hardhat", "compile"])
        .status()
        .expect("Failed to run npx hardhat compile");
    if !hardhat_compile_status.success() {
        panic!("Error: npx hardhat compile failed");
    }
    println!("Ran npx hardhat compile successfully");
}

fn main() {
    println!("cargo::warning=build.rs run ...");
    build_contracts();
    // build tests contracts
    let paths =
        ProjectPathsConfig::hardhat(Path::new(env!("CARGO_MANIFEST_DIR")))
            .unwrap();
    // Use a specific version due to an issue with libc and libstdc++ in the
    // rust Docker image we use to run it.
    let solc = find_or_install_solc(&Version::new(0, 8, 28));
    let project = Project::builder()
        .paths(paths)
        .build(
            MultiCompiler::new(Some(SolcCompiler::Specific(solc)), None)
                .unwrap(),
        )
        .unwrap();
    let output = project.compile().unwrap();
    if output.has_compiler_errors() {
        eprintln!("{output}");
    }
    assert!(!output.has_compiler_errors());
}
