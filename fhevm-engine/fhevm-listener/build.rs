use foundry_compilers::{Project, ProjectPathsConfig};
use std::path::Path;
fn main() {
    println!("cargo::warning=build.rs run ...");
    let paths =
        ProjectPathsConfig::hardhat(Path::new(env!("CARGO_MANIFEST_DIR")))
            .unwrap();
    let project = Project::builder()
        .paths(paths)
        .build(Default::default())
        .unwrap();
    let output = project.compile().unwrap();
    if output.has_compiler_errors() {
        eprintln!("{output}");
    }
    assert!(!output.has_compiler_errors());
    project.rerun_if_sources_changed();
}
