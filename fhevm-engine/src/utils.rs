use std::collections::{BTreeSet, HashMap, HashSet};

use lazy_static::lazy_static;
use crate::{server::coprocessor::AsyncComputation, types::CoprocessorError};

pub fn check_if_handle_is_zero(inp: &str) -> bool {
    lazy_static! {
        static ref TARGET_HANDLE_REGEX: regex::Regex = regex::Regex::new("^0x[0]+$").unwrap();
    }

    TARGET_HANDLE_REGEX.is_match(inp)
}

// handle must be serializable to bytes for scalar operations
pub fn check_valid_ciphertext_handle(inp: &str) -> Result<(), CoprocessorError> {
    lazy_static! {
        static ref VALID_HANDLE_REGEX: regex::Regex = regex::Regex::new("^0x[0-9a-f]+$").unwrap();
    }

    // 66 including 0x in front
    if inp.len() > 66 {
        return Err(CoprocessorError::CiphertextHandleLongerThan64Bytes);
    }

    // at least one hex nibble
    if inp.len() < 4 {
        return Err(CoprocessorError::CiphertextHandleMustBeAtLeast4Bytes(inp.to_string()));
    }

    if inp.len() % 2 != 0 {
        return Err(CoprocessorError::CiphertextHandleMustHaveEvenAmountOfHexNibblets(inp.to_string()));
    }

    if !VALID_HANDLE_REGEX.is_match(inp) {
        return Err(CoprocessorError::InvalidHandle(inp.to_string()));
    }

    Ok(())
}

/// Returns computations in order for dependency resolution and not found handles for checking in database
pub fn sort_computations_by_dependencies<'a>(input: &'a [AsyncComputation]) -> Result<(Vec<&'a AsyncComputation>, BTreeSet<String>), CoprocessorError> {
    let mut res = Vec::with_capacity(input.len());

    let mut output_handles: HashMap<&str, usize> = HashMap::new();
    for (idx, comp) in input.iter().enumerate() {
        check_valid_ciphertext_handle(comp.output_handle.as_str())?;
        if output_handles.insert(comp.output_handle.as_str(), idx).is_some() {
            return Err(CoprocessorError::DuplicateOutputHandleInBatch(comp.output_handle.clone()));
        }
    }

    let mut computation_dependencies: Vec<(usize, Vec<usize>)> = Vec::with_capacity(input.len());
    let mut handles_to_check_in_db: BTreeSet<String> = BTreeSet::new();
    for (idx, comp) in input.iter().enumerate() {
        let mut this_deps = Vec::with_capacity(comp.input_handles.len());
        for (dep_idx, ih) in comp.input_handles.iter().enumerate() {
            check_valid_ciphertext_handle(ih.as_str())?;
            if ih == &comp.output_handle {
                return Err(CoprocessorError::OutputHandleIsAlsoInputHandle(ih.clone()));
            }

            let is_scalar_operand = comp.is_scalar && dep_idx == 1;

            if !is_scalar_operand {
                match output_handles.get(ih.as_str()) {
                    Some(dep_idx) => {
                        this_deps.push(*dep_idx);
                    }
                    None => {
                        handles_to_check_in_db.insert(ih.clone());
                    }
                }
            }
        }
        // the further dependency is in array the later it should be processed
        this_deps.sort();
        this_deps.reverse();
        computation_dependencies.push((idx, this_deps));
    }


    // least dependencies goes to the left, most dependencies to the right
    computation_dependencies.sort_by(|(_, deps_a), (_, deps_b)| deps_a.cmp(deps_b));

    let mut simulation_completed_outputs: HashSet<&str> = HashSet::new();
    for (inp_idx, _) in computation_dependencies {
        let async_comp = &input[inp_idx];

        for (dep_idx, ih) in async_comp.input_handles.iter().enumerate() {

            let is_scalar_operand = async_comp.is_scalar && dep_idx == 1;

            if !is_scalar_operand && !handles_to_check_in_db.contains(ih) && !simulation_completed_outputs.contains(ih.as_str()) {
                // this must be loop if we don't see that handle is completed here, for example
                // [output: 1, deps: [0, 2, 3]]
                // [output: 0, deps: [1, 2, 3]]
                return Err(CoprocessorError::CiphertextComputationDependencyLoopDetected {
                    uncomputable_output_handle: async_comp.output_handle.clone(),
                    uncomputable_handle_dependencies: async_comp.input_handles.clone(),
                });
            }
        }

        simulation_completed_outputs.insert(async_comp.output_handle.as_str());

        res.push(async_comp);
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
    let comp = vec![
        AsyncComputation {
            operation: 1,
            is_scalar: false,
            output_handle: "0x".to_string(),
            input_handles: vec![
                "0x01".to_string(),
                "0x02".to_string(),
            ]
        },
    ];

    match sort_computations_by_dependencies(&comp) {
        Err(CoprocessorError::CiphertextHandleMustBeAtLeast4Bytes(handle)) => {
            assert_eq!(handle, "0x");
        },
        other => {
            panic!("Unexpected result: {:?}", other);
        }
    }
}

#[test]
fn test_invalid_handle_uneven_hex_nibblets() {
    let comp = vec![
        AsyncComputation {
            operation: 1,
            is_scalar: false,
            output_handle: "0x000".to_string(),
            input_handles: vec![
                "0x01".to_string(),
                "0x02".to_string(),
            ]
        },
    ];

    match sort_computations_by_dependencies(&comp) {
        Err(CoprocessorError::CiphertextHandleMustHaveEvenAmountOfHexNibblets(h)) => {
            assert_eq!(h, "0x000");
        },
        other => {
            panic!("Unexpected result: {:?}", other);
        }
    }
}

#[test]
fn test_invalid_handle_too_long() {
    let comp = vec![
        AsyncComputation {
            operation: 1,
            is_scalar: false,
            output_handle: "0x00000000000000000000000000000000000000000000000000000000000000000".to_string(),
            input_handles: vec![
                "0x01".to_string(),
                "0x02".to_string(),
            ]
        },
    ];

    match sort_computations_by_dependencies(&comp) {
        Err(CoprocessorError::CiphertextHandleLongerThan64Bytes) => {},
        other => {
            panic!("Unexpected result: {:?}", other);
        }
    }
}

#[test]
fn test_invalid_handle_bad_symbol() {
    let comp = vec![
        AsyncComputation {
            operation: 1,
            is_scalar: false,
            output_handle: "0x@@".to_string(),
            input_handles: vec![
                "0x01".to_string(),
                "0x02".to_string(),
            ]
        },
    ];

    match sort_computations_by_dependencies(&comp) {
        Err(CoprocessorError::InvalidHandle(handle)) => {
            assert_eq!(handle, "0x@@");
        },
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
            is_scalar: false,
            output_handle: "0x00".to_string(),
            input_handles: vec![
                "0x01".to_string(),
                "0x02".to_string(),
            ]
        },
        AsyncComputation {
            operation: 1,
            is_scalar: false,
            output_handle: "0x01".to_string(),
            input_handles: vec![
                "0x00".to_string(),
                "0x02".to_string(),
            ]
        },
    ];

    match sort_computations_by_dependencies(&comp) {
        Err(CoprocessorError::CiphertextComputationDependencyLoopDetected { uncomputable_output_handle, uncomputable_handle_dependencies }) => {
            assert_eq!(uncomputable_output_handle, "0x01");
            assert_eq!(uncomputable_handle_dependencies, ["0x00", "0x02"]);
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
            is_scalar: false,
            output_handle: "0x00".to_string(),
            input_handles: vec![
                "0x01".to_string(),
                "0x03".to_string(),
            ]
        },
        AsyncComputation {
            operation: 1,
            is_scalar: false,
            output_handle: "0x03".to_string(),
            input_handles: vec![
                "0x02".to_string(),
                "0x04".to_string(),
            ]
        },
        AsyncComputation {
            operation: 1,
            is_scalar: false,
            output_handle: "0x04".to_string(),
            input_handles: vec![
                "0x00".to_string(),
                "0x02".to_string(),
            ]
        },
    ];

    match sort_computations_by_dependencies(&comp) {
        Err(CoprocessorError::CiphertextComputationDependencyLoopDetected { uncomputable_output_handle, uncomputable_handle_dependencies }) => {
            assert_eq!(uncomputable_output_handle, "0x04");
            assert_eq!(uncomputable_handle_dependencies, ["0x00", "0x02"]);
        }
        other => {
            panic!("Unexpected result: {:?}", other);
        }
    }
}