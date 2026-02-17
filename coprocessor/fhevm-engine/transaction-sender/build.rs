use std::path::Path;

use foundry_compilers::{multi::MultiCompiler, solc::SolcCompiler, Project, ProjectPathsConfig};
use semver::Version;
use solc_build_utils::find_or_install_solc_locked;

fn main() {
    let paths = ProjectPathsConfig::hardhat(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    // Use a specific version due to an issue with libc and libstdc++ in the rust Docker image we use to run it.
    let solc = find_or_install_solc_locked(&Version::new(0, 8, 28));
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
