use std::{env, path::Path, thread::sleep, time::Duration};

use foundry_compilers::{
    multi::MultiCompiler,
    solc::{Solc, SolcCompiler},
    Project, ProjectPathsConfig,
};
use semver::Version;

fn retry_on_text_file_busy<T, E, F>(label: &str, mut op: F) -> T
where
    E: std::fmt::Display,
    F: FnMut() -> Result<T, E>,
{
    const MAX_ATTEMPTS: u64 = 5;
    for attempt in 1..=MAX_ATTEMPTS {
        match op() {
            Ok(value) => return value,
            Err(err) => {
                if err.to_string().contains("Text file busy") && attempt < MAX_ATTEMPTS {
                    println!(
                        "cargo:warning={label} failed with 'Text file busy' (attempt {attempt}/{MAX_ATTEMPTS}), retrying..."
                    );
                    sleep(Duration::from_millis(250 * attempt));
                    continue;
                }
                panic!("{label} failed: {err}");
            }
        }
    }
    unreachable!()
}

fn main() {
    let paths = ProjectPathsConfig::hardhat(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    // Use a specific version due to an issue with libc and libstdc++ in the rust Docker image we use to run it.
    let solc = retry_on_text_file_busy("Solc::find_or_install", || {
        Solc::find_or_install(&Version::new(0, 8, 28))
    });
    let project = Project::builder()
        .paths(paths)
        .build(MultiCompiler::new(Some(SolcCompiler::Specific(solc)), None).unwrap())
        .unwrap();

    let output = retry_on_text_file_busy("project.compile", || project.compile());
    if output.has_compiler_errors() {
        panic!("Solidity compilation error: {}", output);
    }

    project.rerun_if_sources_changed();
}
