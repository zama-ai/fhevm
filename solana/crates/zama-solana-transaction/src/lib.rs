//! Provider-independent resolution of Solana compiled transaction instructions.
//!
//! RPC and streaming adapters parse their own wire formats, then pass raw keys
//! and compiled instructions here. This crate owns the Solana transaction
//! semantics shared by both paths without depending on a Solana SDK version.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompiledInstruction {
    pub program_id_index: usize,
    pub account_indices: Vec<usize>,
    pub data: Vec<u8>,
    /// Required for inner instructions. Ignored for top-level instructions,
    /// whose stack height is always one.
    pub stack_height: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InnerInstructionGroup {
    pub top_level_index: usize,
    pub instructions: Vec<CompiledInstruction>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedInstruction {
    pub program_id: [u8; 32],
    pub accounts: Vec<[u8; 32]>,
    pub data: Vec<u8>,
    pub top_level_index: usize,
    pub stack_height: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeError {
    ProgramIndexOutOfRange {
        index: usize,
    },
    AccountIndexOutOfRange {
        index: usize,
    },
    OrphanInnerGroup {
        top_level_index: usize,
    },
    DuplicateInnerGroup {
        top_level_index: usize,
    },
    MissingStackHeight {
        top_level_index: usize,
        position: usize,
    },
    InvalidStackHeight {
        top_level_index: usize,
        position: usize,
        height: u32,
    },
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProgramIndexOutOfRange { index } => {
                write!(f, "program id index {index} is out of range")
            }
            Self::AccountIndexOutOfRange { index } => {
                write!(f, "account index {index} is out of range")
            }
            Self::OrphanInnerGroup { top_level_index } => write!(
                f,
                "inner-instruction group index {top_level_index} is out of range"
            ),
            Self::DuplicateInnerGroup { top_level_index } => write!(
                f,
                "duplicate inner-instruction group index {top_level_index}"
            ),
            Self::MissingStackHeight {
                top_level_index,
                position,
            } => write!(
                f,
                "inner instruction {position} in group {top_level_index} has no stack height"
            ),
            Self::InvalidStackHeight {
                top_level_index,
                position,
                height,
            } => write!(
                f,
                "impossible stack height {height} at inner instruction {position} in group {top_level_index}"
            ),
        }
    }
}

impl Error for DecodeError {}

/// Resolves a successful transaction's instructions in execution order.
///
/// Account indexes address static keys followed by writable ALT keys followed
/// by readonly ALT keys. Each top-level instruction is immediately followed by
/// the inner instructions it invoked, in the order recorded by Solana.
pub fn resolve_transaction(
    static_keys: &[[u8; 32]],
    loaded_writable_keys: &[[u8; 32]],
    loaded_readonly_keys: &[[u8; 32]],
    top_level: Vec<CompiledInstruction>,
    inner_groups: Vec<InnerInstructionGroup>,
) -> Result<Vec<ResolvedInstruction>, DecodeError> {
    let account_keys: Vec<[u8; 32]> = static_keys
        .iter()
        .chain(loaded_writable_keys)
        .chain(loaded_readonly_keys)
        .copied()
        .collect();

    let mut inner_by_index = HashMap::with_capacity(inner_groups.len());
    for group in inner_groups {
        if group.top_level_index >= top_level.len() {
            return Err(DecodeError::OrphanInnerGroup {
                top_level_index: group.top_level_index,
            });
        }
        if inner_by_index
            .insert(group.top_level_index, group.instructions)
            .is_some()
        {
            return Err(DecodeError::DuplicateInnerGroup {
                top_level_index: group.top_level_index,
            });
        }
        validate_inner_stack(
            inner_by_index
                .get(&group.top_level_index)
                .expect("inner group was just inserted"),
            group.top_level_index,
        )?;
    }

    let instruction_count = top_level.len() + inner_by_index.values().map(Vec::len).sum::<usize>();
    let mut resolved = Vec::with_capacity(instruction_count);
    for (top_level_index, instruction) in top_level.into_iter().enumerate() {
        resolved.push(resolve_instruction(
            &account_keys,
            instruction,
            top_level_index,
            1,
        )?);
        if let Some(inner) = inner_by_index.remove(&top_level_index) {
            for instruction in inner {
                let stack_height = instruction
                    .stack_height
                    .expect("inner stack heights were validated");
                resolved.push(resolve_instruction(
                    &account_keys,
                    instruction,
                    top_level_index,
                    stack_height,
                )?);
            }
        }
    }
    Ok(resolved)
}

fn validate_inner_stack(
    instructions: &[CompiledInstruction],
    top_level_index: usize,
) -> Result<(), DecodeError> {
    let mut previous_height = 1u32;
    for (position, instruction) in instructions.iter().enumerate() {
        let height = instruction
            .stack_height
            .ok_or(DecodeError::MissingStackHeight {
                top_level_index,
                position,
            })?;
        if height < 2
            || (position == 0 && height != 2)
            || height > previous_height.saturating_add(1)
        {
            return Err(DecodeError::InvalidStackHeight {
                top_level_index,
                position,
                height,
            });
        }
        previous_height = height;
    }
    Ok(())
}

fn resolve_instruction(
    account_keys: &[[u8; 32]],
    instruction: CompiledInstruction,
    top_level_index: usize,
    stack_height: u32,
) -> Result<ResolvedInstruction, DecodeError> {
    let program_id = account_keys
        .get(instruction.program_id_index)
        .copied()
        .ok_or(DecodeError::ProgramIndexOutOfRange {
            index: instruction.program_id_index,
        })?;
    let accounts = instruction
        .account_indices
        .iter()
        .map(|index| {
            account_keys
                .get(*index)
                .copied()
                .ok_or(DecodeError::AccountIndexOutOfRange { index: *index })
        })
        .collect::<Result<_, _>>()?;

    Ok(ResolvedInstruction {
        program_id,
        accounts,
        data: instruction.data,
        top_level_index,
        stack_height,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    fn instruction(
        program_id_index: usize,
        account_indices: &[usize],
        stack_height: Option<u32>,
    ) -> CompiledInstruction {
        CompiledInstruction {
            program_id_index,
            account_indices: account_indices.to_vec(),
            data: vec![program_id_index as u8],
            stack_height,
        }
    }

    #[test]
    fn assembles_alt_keys_and_preserves_execution_order() {
        let resolved = resolve_transaction(
            &[key(1), key(2)],
            &[key(3)],
            &[key(4)],
            vec![instruction(3, &[2], None), instruction(1, &[0], None)],
            vec![InnerInstructionGroup {
                top_level_index: 0,
                instructions: vec![instruction(2, &[3], Some(2)), instruction(1, &[0], Some(3))],
            }],
        )
        .unwrap();

        assert_eq!(
            resolved
                .iter()
                .map(|instruction| (
                    instruction.program_id[0],
                    instruction.accounts[0][0],
                    instruction.top_level_index,
                    instruction.stack_height,
                ))
                .collect::<Vec<_>>(),
            vec![(4, 3, 0, 1), (3, 4, 0, 2), (2, 1, 0, 3), (2, 1, 1, 1)]
        );
    }

    #[test]
    fn rejects_orphan_and_duplicate_inner_groups() {
        let top_level = [instruction(0, &[], None)];
        let orphan = [InnerInstructionGroup {
            top_level_index: 1,
            instructions: vec![],
        }];
        assert_eq!(
            resolve_transaction(&[key(1)], &[], &[], top_level.to_vec(), orphan.to_vec(),),
            Err(DecodeError::OrphanInnerGroup { top_level_index: 1 })
        );

        let duplicate = [
            InnerInstructionGroup {
                top_level_index: 0,
                instructions: vec![],
            },
            InnerInstructionGroup {
                top_level_index: 0,
                instructions: vec![],
            },
        ];
        assert_eq!(
            resolve_transaction(&[key(1)], &[], &[], top_level.to_vec(), duplicate.to_vec(),),
            Err(DecodeError::DuplicateInnerGroup { top_level_index: 0 })
        );
    }

    #[test]
    fn rejects_invalid_indexes_and_stack_traces() {
        assert_eq!(
            resolve_transaction(&[key(1)], &[], &[], vec![instruction(1, &[], None)], vec![],),
            Err(DecodeError::ProgramIndexOutOfRange { index: 1 })
        );
        assert_eq!(
            resolve_transaction(
                &[key(1)],
                &[],
                &[],
                vec![instruction(0, &[1], None)],
                vec![],
            ),
            Err(DecodeError::AccountIndexOutOfRange { index: 1 })
        );

        let top_level = [instruction(0, &[], None)];
        for (stack_height, expected) in [
            (
                None,
                DecodeError::MissingStackHeight {
                    top_level_index: 0,
                    position: 0,
                },
            ),
            (
                Some(3),
                DecodeError::InvalidStackHeight {
                    top_level_index: 0,
                    position: 0,
                    height: 3,
                },
            ),
        ] {
            let groups = [InnerInstructionGroup {
                top_level_index: 0,
                instructions: vec![instruction(0, &[], stack_height)],
            }];
            assert_eq!(
                resolve_transaction(&[key(1)], &[], &[], top_level.to_vec(), groups.to_vec(),),
                Err(expected)
            );
        }
    }
}
