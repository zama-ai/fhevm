use daggy::Dag;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::sync::atomic::AtomicUsize;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Generate a DOT file and PNG image from a DAG using Graphviz.
/// The output files will be named `graph-<label>.dot` and `graph-<label>.png`.
pub fn gen_dot_from_dag<N: std::fmt::Debug, E: std::fmt::Debug>(
    dag: &Dag<N, E>,
    folder: &str,
    label: &str,
) -> Result<(), Box<dyn Error>> {
    use daggy::petgraph::dot::{Config, Dot};
    let graph = dag.graph();

    let dot = Dot::with_attr_getters(
        graph,
        &[Config::EdgeNoLabel],
        &|_, _| String::new(),
        &|_g, (ni, node)| {
            let as_str = format!("{:?}, index {}", node, ni.index());
            let sanitized = as_str.replace('"', "");
            format!("label=\"{}\"", sanitized)
        },
    );

    let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let dot_filename = format!("{}/graph-{}-{}.dot", folder, label, id);
    let png_filename = format!("{}/graph-{}-{}.png", folder, label, id);

    // Write DOT file
    let mut file = File::create(&dot_filename)?;
    write!(file, "{:?}", dot)?;

    // Run Graphviz
    let status = Command::new("dot")
        .args(["-Tpng", &dot_filename, "-o", &png_filename])
        .status()?;

    if !status.success() {
        return Err(format!("dot command failed with status {}", status).into());
    }

    tracing::debug!(
        "Graphviz output written to '{}' and '{}'",
        dot_filename,
        png_filename
    );

    Ok(())
}
