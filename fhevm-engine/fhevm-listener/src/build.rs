use std::path::Path;
use foundry_compilers::{Project, ProjectPathsConfig};
fn main() {
    let paths = ProjectPathsConfig::hardhat(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    let project = Project::builder()
        .paths(paths)
        .build(Default::default())
        .unwrap();
    let output = project.compile().unwrap();
    assert_eq!(output.has_compiler_errors(), false);
    project.rerun_if_sources_changed();
}