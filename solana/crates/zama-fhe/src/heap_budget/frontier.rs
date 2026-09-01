//! The 122-cell exploration grid and its printer.

use zama_host::MAX_FHE_EXECUTION_STEPS;

use super::harness::{try_measure, ShapeBuilder};
use super::shapes::*;

/// Every shape on the exploration frontier, admitted or not: persist kind x output count x
/// subject width, plus the attestation ladder.
pub(crate) fn frontier_shapes() -> Vec<(String, ShapeBuilder)> {
    let mut shapes: Vec<(String, ShapeBuilder)> = Vec::new();
    for (kind, kind_name) in [
        (PersistKind::Create, "create"),
        (PersistKind::Update, "update"),
    ] {
        for subjects in [1, 2, 4, 6, 8] {
            for outputs in [4, 8, 12, 16, 20, 24, 28, MAX_FHE_EXECUTION_STEPS] {
                shapes.push((
                    format!("{kind_name} x{outputs:2} subjects={subjects}"),
                    Box::new(persist_shape(
                        kind,
                        MAX_FHE_EXECUTION_STEPS,
                        outputs,
                        subjects,
                    )),
                ));
            }
        }
    }
    for count in 1..=MAX_FHE_EXECUTION_STEPS {
        shapes.push((
            format!("attestations x{count:2}"),
            Box::new(attestation_shape(count)),
        ));
    }
    // The reduction ops carry the only caller-sized operand tables, so the frontier
    // exercises both, thin-and-deep and wide-and-shallow.
    for (kind, kind_name) in [(ReductionKind::Sum, "sum"), (ReductionKind::IsIn, "is_in")] {
        for (steps, operands) in [(1, 60), (2, 60), (8, 8), (MAX_FHE_EXECUTION_STEPS, 8)] {
            shapes.push((
                format!("{kind_name} x{steps:2} operands={operands}"),
                Box::new(reduction_shape(kind, steps, operands)),
            ));
        }
    }
    shapes.push((
        "mixed ops (add/sum/is_in, 1 create)".to_string(),
        Box::new(mixed_ops_shape()),
    ));
    shapes.push((
        "attestation reused across the chain".to_string(),
        Box::new(reused_attestation_shape()),
    ));
    shapes
}

/// The full app-side frontier, persist kind x output count x subject width, printed with the
/// typed rejection where the builder refuses the shape. This is the exploration companion to
/// the host-side boundary sweeps in `runtime-tests/tests/fhe_execute_boundary.rs`.
#[test]
#[ignore = "frontier grid, run explicitly with --nocapture"]
fn print_build_frontier_grid() {
    for (name, build) in frontier_shapes() {
        match try_measure(name.clone(), build) {
            Ok(shape) => println!(
                "{name:36} build={:6} packet={:6} invoke={:6} total={:6} fits",
                shape.build_bytes,
                shape.packet_bytes,
                shape.cost.invoke_heap_bytes,
                shape.total(),
            ),
            Err(error) => println!("{name:36} rejected: {error:?}"),
        }
    }
}
