use foundry_compilers::{
    multi::MultiCompiler,
    solc::{Solc, SolcCompiler},
    Project, ProjectPathsConfig,
};
use semver::Version;
use std::path::Path;
fn main() {
    println!("cargo::warning=build.rs run ...");
    let paths =
        ProjectPathsConfig::hardhat(Path::new(env!("CARGO_MANIFEST_DIR")))
            .unwrap();
    // Use a specific version due to an issue with libc and libstdc++ in the rust Docker image we use to run it.
    let solc = Solc::find_or_install(&Version::new(0, 8, 28)).unwrap();
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
    project.rerun_if_sources_changed();
}
