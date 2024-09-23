use std::collections::{BTreeSet, HashMap, HashSet};

use fhevm_engine_common::types::{FhevmError, SupportedFheOperations};

#[cfg(test)]
use crate::server::coprocessor::AsyncComputationInput;

use crate::{
    server::coprocessor::{async_computation_input::Input, AsyncComputation},
    types::CoprocessorError,
};

pub fn check_valid_ciphertext_handle(inp: &[u8]) -> Result<(), CoprocessorError> {
    if inp.len() > 64 {
        return Err(CoprocessorError::CiphertextHandleLongerThan64Bytes);
    }

    if inp.len() < 1 {
        return Err(CoprocessorError::CiphertextHandleMustBeAtLeast1Byte(
            format!("0x{}", hex::encode(inp)),
        ));
    }

    Ok(())
}

/// Returns computations in order for dependency resolution and not found handles for checking in database
pub fn sort_computations_by_dependencies<'a>(
    input: &'a [AsyncComputation],
) -> Result<(Vec<&'a AsyncComputation>, BTreeSet<Vec<u8>>), CoprocessorError> {
    let mut res = Vec::with_capacity(input.len());

    let mut output_handles: HashMap<&[u8], usize> = HashMap::new();
    for (idx, comp) in input.iter().enumerate() {
        check_valid_ciphertext_handle(&comp.output_handle)?;
        if output_handles.insert(&comp.output_handle, idx).is_some() {
            return Err(CoprocessorError::DuplicateOutputHandleInBatch(format!(
                "0x{}",
                hex::encode(&comp.output_handle)
            )));
        }
    }

    let mut computation_dependencies: Vec<(usize, Vec<usize>)> = Vec::with_capacity(input.len());
    let mut handles_to_check_in_db: BTreeSet<Vec<u8>> = BTreeSet::new();
    for (idx, comp) in input.iter().enumerate() {
        let mut this_deps = Vec::with_capacity(comp.inputs.len());
        let mut this_scalar_operands = Vec::with_capacity(comp.inputs.len());
        let fhe_op: SupportedFheOperations = comp
            .operation
            .try_into()
            .map_err(|e| CoprocessorError::FhevmError(e))?;
        for (dep_idx, ih) in comp.inputs.iter().enumerate() {
            let mut is_scalar_operand = false;
            if let Some(ih_input) = &ih.input {
                match ih_input {
                    Input::InputHandle(ih_bytes) => {
                        check_valid_ciphertext_handle(&ih_bytes)?;
                        if ih_bytes == &comp.output_handle {
                            return Err(CoprocessorError::OutputHandleIsAlsoInputHandle(format!(
                                "0x{}",
                                hex::encode(ih_bytes)
                            )));
                        }

                        match output_handles.get(ih_bytes.as_slice()) {
                            Some(dep_idx) => {
                                this_deps.push(*dep_idx);
                            }
                            None => {
                                handles_to_check_in_db.insert(ih_bytes.clone());
                            }
                        }
                    }
                    Input::Scalar(sc_bytes) => {
                        check_valid_ciphertext_handle(&sc_bytes)?;
                        if dep_idx != 1 && !fhe_op.does_have_more_than_one_scalar() {
                            // TODO: remove wrapping after refactor
                            return Err(CoprocessorError::FhevmError(
                                FhevmError::FheOperationOnlySecondOperandCanBeScalar {
                                    scalar_input_index: dep_idx,
                                    only_allowed_scalar_input_index: 1,
                                },
                            ));
                        }
                        is_scalar_operand = true;
                    }
                }
                this_scalar_operands.push(is_scalar_operand);
            } else {
                return Err(CoprocessorError::ComputationInputIsUndefined {
                    computation_output_handle: format!("0x{}", hex::encode(&comp.output_handle)),
                    computation_inputs_index: idx,
                });
            }
        }
        // the further dependency is in array the later it should be processed
        this_deps.sort();
        this_deps.reverse();
        computation_dependencies.push((idx, this_deps));
    }

    // least dependencies goes to the left, most dependencies to the right
    computation_dependencies.sort_by(|(_, deps_a), (_, deps_b)| deps_a.cmp(deps_b));
    let mut simulation_completed_outputs: HashSet<&[u8]> = HashSet::new();

    loop {
        let mut progress_made_in_iteration = false;
        let mut new_computation_dependencies: Vec<(usize, Vec<usize>)> = Vec::new();

        let mut first_uncomputable_handle: &[u8] = [].as_slice();
        let mut first_uncomputable_handle_dependency: &[u8] = [].as_slice();

        for (inp_idx, deps) in computation_dependencies {
            let async_comp = &input[inp_idx];

            let mut can_compute_this = true;
            for ih in &async_comp.inputs {
                if let Some(Input::InputHandle(ih)) = &ih.input {
                    if !handles_to_check_in_db.contains(ih.as_slice())
                        && !simulation_completed_outputs.contains(ih.as_slice())
                    {
                        if first_uncomputable_handle.is_empty() {
                            first_uncomputable_handle = async_comp.output_handle.as_slice();
                            first_uncomputable_handle_dependency = ih.as_slice();
                        }
                        can_compute_this = false;
                    }
                }
            }

            if can_compute_this {
                progress_made_in_iteration = true;
                simulation_completed_outputs.insert(&async_comp.output_handle);
                res.push(async_comp);
            } else {
                // push uncomputable to new queue to try again later
                new_computation_dependencies.push((inp_idx, deps));
            }
        }

        if !progress_made_in_iteration {
            // this must be loop if we don't see progress made
            // [output: 1, deps: [0, 2, 3]]
            // [output: 0, deps: [1, 2, 3]]
            return Err(
                CoprocessorError::CiphertextComputationDependencyLoopDetected {
                    uncomputable_output_handle: format!(
                        "0x{}",
                        hex::encode(&first_uncomputable_handle)
                    ),
                    uncomputable_handle_dependency: format!(
                        "0x{}",
                        hex::encode(first_uncomputable_handle_dependency)
                    ),
                },
            );
        }

        if new_computation_dependencies.is_empty() {
            // everything computed, break loop
            break;
        }

        computation_dependencies = new_computation_dependencies;
    }

    Ok((res, handles_to_check_in_db))
}

pub fn db_url(args: &crate::cli::Args) -> String {
    if let Some(db_url) = &args.database_url {
        return db_url.clone();
    }
    std::env::var("DATABASE_URL").expect("DATABASE_URL is undefined")
}

#[test]
fn test_invalid_handle_too_short() {
    let comp = vec![AsyncComputation {
        operation: 1,
        output_handle: vec![],
        inputs: vec![
            AsyncComputationInput {
                input: Some(Input::InputHandle(vec![1])),
            },
            AsyncComputationInput {
                input: Some(Input::InputHandle(vec![2])),
            },
        ],
    }];

    match sort_computations_by_dependencies(&comp) {
        Err(CoprocessorError::CiphertextHandleMustBeAtLeast1Byte(handle)) => {
            assert_eq!(handle, "0x");
        }
        other => {
            panic!("Unexpected result: {:?}", other);
        }
    }
}

#[test]
fn test_invalid_handle_too_long() {
    let comp = vec![AsyncComputation {
        operation: 1,
        output_handle: vec![0u8; 65],
        inputs: vec![
            AsyncComputationInput {
                input: Some(Input::InputHandle(vec![1])),
            },
            AsyncComputationInput {
                input: Some(Input::InputHandle(vec![2])),
            },
        ],
    }];

    match sort_computations_by_dependencies(&comp) {
        Err(CoprocessorError::CiphertextHandleLongerThan64Bytes) => {}
        other => {
            panic!("Unexpected result: {:?}", other);
        }
    }
}

#[test]
fn test_simple_circular_dependency_detection() {
    let comp = vec![
        AsyncComputation {
            operation: 1,
            output_handle: vec![0],
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(vec![1])),
                },
                AsyncComputationInput {
                    input: Some(Input::InputHandle(vec![2])),
                },
            ],
        },
        AsyncComputation {
            operation: 1,
            output_handle: vec![1],
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(vec![0])),
                },
                AsyncComputationInput {
                    input: Some(Input::InputHandle(vec![2])),
                },
            ],
        },
    ];

    match sort_computations_by_dependencies(&comp) {
        Err(CoprocessorError::CiphertextComputationDependencyLoopDetected {
            uncomputable_output_handle,
            uncomputable_handle_dependency,
        }) => {
            assert_eq!(uncomputable_output_handle, "0x01");
            assert_eq!(uncomputable_handle_dependency, "0x00");
        }
        other => {
            panic!("Unexpected result: {:?}", other);
        }
    }
}

#[test]
fn test_multi_level_circular_dependency_detection() {
    // 0x00 depends on 0x03, 0x03 depends on 0x04, 0x04 depends on 0x00
    let comp = vec![
        AsyncComputation {
            operation: 1,
            output_handle: vec![0],
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(vec![1])),
                },
                AsyncComputationInput {
                    input: Some(Input::InputHandle(vec![3])),
                },
            ],
        },
        AsyncComputation {
            operation: 1,
            output_handle: vec![3],
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(vec![2])),
                },
                AsyncComputationInput {
                    input: Some(Input::InputHandle(vec![4])),
                },
            ],
        },
        AsyncComputation {
            operation: 1,
            output_handle: vec![4],
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::InputHandle(vec![0])),
                },
                AsyncComputationInput {
                    input: Some(Input::InputHandle(vec![2])),
                },
            ],
        },
    ];

    match sort_computations_by_dependencies(&comp) {
        Err(CoprocessorError::CiphertextComputationDependencyLoopDetected {
            uncomputable_output_handle,
            uncomputable_handle_dependency,
        }) => {
            assert_eq!(uncomputable_output_handle, "0x04");
            assert_eq!(uncomputable_handle_dependency, "0x00");
        }
        other => {
            panic!("Unexpected result: {:?}", other);
        }
    }
}
