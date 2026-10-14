//! The 122-cell exploration grid.

use zama_host::MAX_FHE_EXECUTION_STEPS;

use super::harness::ShapeBuilder;
use super::shapes::*;

/// Every shape on the exploration frontier, admitted or not: persist kind x output count x
/// allow-list width, plus the attestation ladder.
pub(crate) fn frontier_shapes() -> Vec<(String, ShapeBuilder)> {
    let mut shapes: Vec<(String, ShapeBuilder)> = Vec::new();
    for (kind, kind_name) in [
        (PersistKind::Create, "create"),
        (PersistKind::Update, "update"),
    ] {
        for allows in [1, 2, 4, 6, 8] {
            for outputs in [4, 8, 12, 16, 20, 24, 28, MAX_FHE_EXECUTION_STEPS] {
                shapes.push((
                    format!("{kind_name} x{outputs:2} allows={allows}"),
                    Box::new(persist_shape(
                        kind,
                        MAX_FHE_EXECUTION_STEPS,
                        outputs,
                        allows,
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
        "mixed ops (add/sum/is_in, one Store output)".to_string(),
        Box::new(mixed_ops_shape()),
    ));
    shapes.push((
        "attestation reused across the chain".to_string(),
        Box::new(reused_attestation_shape()),
    ));
    shapes
}
