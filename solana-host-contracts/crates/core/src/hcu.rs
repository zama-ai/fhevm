use crate::error::{HostContractError, Result};
use crate::events::HostEvent;
use crate::types::{FheType, Handle, Operator, Pubkey};
use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::{HashMap, HashSet};

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, BorshSerialize, BorshDeserialize,
)]
pub struct HcuOperationKey {
    pub op: Operator,
    pub result_type: FheType,
    pub scalar: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct TransactionMeter {
    per_handle_depth: HashMap<Handle, u64>,
    tx_hcu: u64,
}

impl TransactionMeter {
    pub fn hcu_for_handle(&self, handle: Handle) -> u64 {
        self.per_handle_depth
            .get(&handle)
            .copied()
            .unwrap_or_default()
    }

    pub fn transaction_hcu(&self) -> u64 {
        self.tx_hcu
    }
}

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct HcuLimitState {
    global_hcu_cap_per_block: u64,
    used_block_hcu: u64,
    last_seen_slot: u64,
    max_hcu_depth_per_tx: u64,
    max_hcu_per_tx: u64,
    block_hcu_whitelist: HashSet<Pubkey>,
    pricing_overrides: Vec<(HcuOperationKey, u64)>,
}

impl HcuLimitState {
    pub fn new(
        hcu_cap_per_block: u64,
        max_hcu_depth_per_tx: u64,
        max_hcu_per_tx: u64,
    ) -> Result<Self> {
        if hcu_cap_per_block < max_hcu_per_tx {
            return Err(HostContractError::HCUPerBlockBelowMaxPerTx);
        }
        if max_hcu_per_tx < max_hcu_depth_per_tx {
            return Err(HostContractError::MaxHCUPerTxBelowDepth);
        }
        Ok(Self {
            global_hcu_cap_per_block: hcu_cap_per_block,
            used_block_hcu: 0,
            last_seen_slot: 0,
            max_hcu_depth_per_tx,
            max_hcu_per_tx,
            block_hcu_whitelist: HashSet::new(),
            pricing_overrides: Vec::new(),
        })
    }

    pub fn set_pricing(&mut self, key: HcuOperationKey, cost: u64) {
        if let Some((_, existing_cost)) = self
            .pricing_overrides
            .iter_mut()
            .find(|(existing_key, _)| *existing_key == key)
        {
            *existing_cost = cost;
            return;
        }
        self.pricing_overrides.push((key, cost));
    }

    pub fn hcu_per_block(&self) -> u64 {
        self.global_hcu_cap_per_block
    }

    pub fn max_hcu_depth_per_tx(&self) -> u64 {
        self.max_hcu_depth_per_tx
    }

    pub fn max_hcu_per_tx(&self) -> u64 {
        self.max_hcu_per_tx
    }

    pub fn is_block_hcu_whitelisted(&self, account: Pubkey) -> bool {
        self.block_hcu_whitelist.contains(&account)
    }

    pub fn charge_for_operation(
        &mut self,
        key: HcuOperationKey,
        inputs: &[Handle],
        result: Handle,
        caller: Pubkey,
        slot: u64,
        meter: &mut TransactionMeter,
    ) -> Result<()> {
        let op_hcu = self
            .operation_cost(key)
            .ok_or(HostContractError::UnsupportedOperationPricing)?;

        self.update_and_verify_hcu_block_limit(op_hcu, caller, slot)?;

        let transaction_hcu = meter.tx_hcu.saturating_add(op_hcu);
        if transaction_hcu > self.max_hcu_per_tx {
            return Err(HostContractError::HCUTransactionLimitExceeded);
        }
        meter.tx_hcu = transaction_hcu;

        let parent_depth = inputs
            .iter()
            .map(|handle| meter.hcu_for_handle(*handle))
            .max()
            .unwrap_or_default();
        let total_depth = op_hcu.saturating_add(parent_depth);
        if total_depth > self.max_hcu_depth_per_tx {
            return Err(HostContractError::HCUTransactionDepthLimitExceeded);
        }
        meter.per_handle_depth.insert(result, total_depth);
        Ok(())
    }

    pub fn set_hcu_per_block(&mut self, hcu_per_block: u64) -> Result<HostEvent> {
        if hcu_per_block < self.max_hcu_per_tx {
            return Err(HostContractError::HCUPerBlockBelowMaxPerTx);
        }
        self.global_hcu_cap_per_block = hcu_per_block;
        Ok(HostEvent::HcuPerBlockSet { hcu_per_block })
    }

    pub fn set_max_hcu_depth_per_tx(&mut self, max_hcu_depth_per_tx: u64) -> Result<HostEvent> {
        if self.max_hcu_per_tx < max_hcu_depth_per_tx {
            return Err(HostContractError::MaxHCUPerTxBelowDepth);
        }
        self.max_hcu_depth_per_tx = max_hcu_depth_per_tx;
        Ok(HostEvent::MaxHcuDepthPerTxSet {
            max_hcu_depth_per_tx,
        })
    }

    pub fn set_max_hcu_per_tx(&mut self, max_hcu_per_tx: u64) -> Result<HostEvent> {
        if self.global_hcu_cap_per_block < max_hcu_per_tx {
            return Err(HostContractError::HCUPerBlockBelowMaxPerTx);
        }
        if max_hcu_per_tx < self.max_hcu_depth_per_tx {
            return Err(HostContractError::MaxHCUPerTxBelowDepth);
        }
        self.max_hcu_per_tx = max_hcu_per_tx;
        Ok(HostEvent::MaxHcuPerTxSet { max_hcu_per_tx })
    }

    pub fn add_to_block_hcu_whitelist(&mut self, account: Pubkey) -> Result<HostEvent> {
        if !self.block_hcu_whitelist.insert(account) {
            return Err(HostContractError::AlreadyBlockHCUWhitelisted);
        }
        Ok(HostEvent::BlockHcuWhitelistAdded { account })
    }

    pub fn remove_from_block_hcu_whitelist(&mut self, account: Pubkey) -> Result<HostEvent> {
        if !self.block_hcu_whitelist.remove(&account) {
            return Err(HostContractError::NotBlockHCUWhitelisted);
        }
        Ok(HostEvent::BlockHcuWhitelistRemoved { account })
    }

    fn update_and_verify_hcu_block_limit(
        &mut self,
        op_hcu: u64,
        caller: Pubkey,
        slot: u64,
    ) -> Result<()> {
        if self.block_hcu_whitelist.contains(&caller) {
            return Ok(());
        }
        if self.last_seen_slot != slot {
            self.last_seen_slot = slot;
            self.used_block_hcu = 0;
        }
        let next_hcu = self.used_block_hcu.saturating_add(op_hcu);
        if next_hcu > self.global_hcu_cap_per_block {
            return Err(HostContractError::HCUBlockLimitExceeded);
        }
        self.used_block_hcu = next_hcu;
        Ok(())
    }

    fn operation_cost(&self, key: HcuOperationKey) -> Option<u64> {
        self.pricing_overrides
            .iter()
            .find_map(|(override_key, cost)| (*override_key == key).then_some(*cost))
            .or_else(|| default_pricing(key))
    }
}

const DEFAULT_PRICING: &[(HcuOperationKey, u64)] = &[
    (
        HcuOperationKey {
            op: Operator::FheAdd,
            result_type: FheType::Uint8,
            scalar: true,
        },
        84_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheAdd,
            result_type: FheType::Uint16,
            scalar: true,
        },
        93_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheAdd,
            result_type: FheType::Uint32,
            scalar: true,
        },
        95_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheAdd,
            result_type: FheType::Uint64,
            scalar: true,
        },
        133_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheAdd,
            result_type: FheType::Uint128,
            scalar: true,
        },
        172_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheAdd,
            result_type: FheType::Uint8,
            scalar: false,
        },
        88_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheAdd,
            result_type: FheType::Uint16,
            scalar: false,
        },
        93_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheAdd,
            result_type: FheType::Uint32,
            scalar: false,
        },
        125_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheAdd,
            result_type: FheType::Uint64,
            scalar: false,
        },
        162_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheAdd,
            result_type: FheType::Uint128,
            scalar: false,
        },
        259_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheSub,
            result_type: FheType::Uint8,
            scalar: true,
        },
        84_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheSub,
            result_type: FheType::Uint16,
            scalar: true,
        },
        93_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheSub,
            result_type: FheType::Uint32,
            scalar: true,
        },
        95_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheSub,
            result_type: FheType::Uint64,
            scalar: true,
        },
        133_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheSub,
            result_type: FheType::Uint128,
            scalar: true,
        },
        172_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheSub,
            result_type: FheType::Uint8,
            scalar: false,
        },
        91_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheSub,
            result_type: FheType::Uint16,
            scalar: false,
        },
        93_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheSub,
            result_type: FheType::Uint32,
            scalar: false,
        },
        125_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheSub,
            result_type: FheType::Uint64,
            scalar: false,
        },
        162_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheSub,
            result_type: FheType::Uint128,
            scalar: false,
        },
        260_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMul,
            result_type: FheType::Uint8,
            scalar: true,
        },
        122_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMul,
            result_type: FheType::Uint16,
            scalar: true,
        },
        193_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMul,
            result_type: FheType::Uint32,
            scalar: true,
        },
        265_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMul,
            result_type: FheType::Uint64,
            scalar: true,
        },
        365_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMul,
            result_type: FheType::Uint128,
            scalar: true,
        },
        696_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMul,
            result_type: FheType::Uint8,
            scalar: false,
        },
        150_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMul,
            result_type: FheType::Uint16,
            scalar: false,
        },
        222_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMul,
            result_type: FheType::Uint32,
            scalar: false,
        },
        328_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMul,
            result_type: FheType::Uint64,
            scalar: false,
        },
        596_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMul,
            result_type: FheType::Uint128,
            scalar: false,
        },
        1_686_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheDiv,
            result_type: FheType::Uint8,
            scalar: true,
        },
        210_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheDiv,
            result_type: FheType::Uint16,
            scalar: true,
        },
        302_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheDiv,
            result_type: FheType::Uint32,
            scalar: true,
        },
        438_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheDiv,
            result_type: FheType::Uint64,
            scalar: true,
        },
        715_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheDiv,
            result_type: FheType::Uint128,
            scalar: true,
        },
        1_225_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRem,
            result_type: FheType::Uint8,
            scalar: true,
        },
        440_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRem,
            result_type: FheType::Uint16,
            scalar: true,
        },
        580_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRem,
            result_type: FheType::Uint32,
            scalar: true,
        },
        792_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRem,
            result_type: FheType::Uint64,
            scalar: true,
        },
        1_153_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRem,
            result_type: FheType::Uint128,
            scalar: true,
        },
        1_943_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitAnd,
            result_type: FheType::Bool,
            scalar: true,
        },
        22_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitAnd,
            result_type: FheType::Uint8,
            scalar: true,
        },
        31_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitAnd,
            result_type: FheType::Uint16,
            scalar: true,
        },
        31_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitAnd,
            result_type: FheType::Uint32,
            scalar: true,
        },
        32_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitAnd,
            result_type: FheType::Uint64,
            scalar: true,
        },
        34_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitAnd,
            result_type: FheType::Uint128,
            scalar: true,
        },
        37_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitAnd,
            result_type: FheType::Uint256,
            scalar: true,
        },
        38_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitAnd,
            result_type: FheType::Bool,
            scalar: false,
        },
        25_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitAnd,
            result_type: FheType::Uint8,
            scalar: false,
        },
        31_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitAnd,
            result_type: FheType::Uint16,
            scalar: false,
        },
        31_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitAnd,
            result_type: FheType::Uint32,
            scalar: false,
        },
        32_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitAnd,
            result_type: FheType::Uint64,
            scalar: false,
        },
        34_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitAnd,
            result_type: FheType::Uint128,
            scalar: false,
        },
        37_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitAnd,
            result_type: FheType::Uint256,
            scalar: false,
        },
        38_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitOr,
            result_type: FheType::Bool,
            scalar: true,
        },
        22_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitOr,
            result_type: FheType::Uint8,
            scalar: true,
        },
        30_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitOr,
            result_type: FheType::Uint16,
            scalar: true,
        },
        30_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitOr,
            result_type: FheType::Uint32,
            scalar: true,
        },
        32_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitOr,
            result_type: FheType::Uint64,
            scalar: true,
        },
        34_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitOr,
            result_type: FheType::Uint128,
            scalar: true,
        },
        37_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitOr,
            result_type: FheType::Uint256,
            scalar: true,
        },
        38_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitOr,
            result_type: FheType::Bool,
            scalar: false,
        },
        24_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitOr,
            result_type: FheType::Uint8,
            scalar: false,
        },
        30_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitOr,
            result_type: FheType::Uint16,
            scalar: false,
        },
        31_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitOr,
            result_type: FheType::Uint32,
            scalar: false,
        },
        32_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitOr,
            result_type: FheType::Uint64,
            scalar: false,
        },
        34_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitOr,
            result_type: FheType::Uint128,
            scalar: false,
        },
        37_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitOr,
            result_type: FheType::Uint256,
            scalar: false,
        },
        38_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitXor,
            result_type: FheType::Bool,
            scalar: true,
        },
        22_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitXor,
            result_type: FheType::Uint8,
            scalar: true,
        },
        31_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitXor,
            result_type: FheType::Uint16,
            scalar: true,
        },
        31_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitXor,
            result_type: FheType::Uint32,
            scalar: true,
        },
        32_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitXor,
            result_type: FheType::Uint64,
            scalar: true,
        },
        34_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitXor,
            result_type: FheType::Uint128,
            scalar: true,
        },
        37_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitXor,
            result_type: FheType::Uint256,
            scalar: true,
        },
        39_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitXor,
            result_type: FheType::Bool,
            scalar: false,
        },
        22_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitXor,
            result_type: FheType::Uint8,
            scalar: false,
        },
        31_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitXor,
            result_type: FheType::Uint16,
            scalar: false,
        },
        31_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitXor,
            result_type: FheType::Uint32,
            scalar: false,
        },
        32_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitXor,
            result_type: FheType::Uint64,
            scalar: false,
        },
        34_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitXor,
            result_type: FheType::Uint128,
            scalar: false,
        },
        37_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheBitXor,
            result_type: FheType::Uint256,
            scalar: false,
        },
        39_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShl,
            result_type: FheType::Uint8,
            scalar: true,
        },
        32_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShl,
            result_type: FheType::Uint16,
            scalar: true,
        },
        32_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShl,
            result_type: FheType::Uint32,
            scalar: true,
        },
        32_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShl,
            result_type: FheType::Uint64,
            scalar: true,
        },
        34_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShl,
            result_type: FheType::Uint128,
            scalar: true,
        },
        37_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShl,
            result_type: FheType::Uint256,
            scalar: true,
        },
        39_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShl,
            result_type: FheType::Uint8,
            scalar: false,
        },
        92_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShl,
            result_type: FheType::Uint16,
            scalar: false,
        },
        125_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShl,
            result_type: FheType::Uint32,
            scalar: false,
        },
        162_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShl,
            result_type: FheType::Uint64,
            scalar: false,
        },
        208_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShl,
            result_type: FheType::Uint128,
            scalar: false,
        },
        272_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShl,
            result_type: FheType::Uint256,
            scalar: false,
        },
        378_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShr,
            result_type: FheType::Uint8,
            scalar: true,
        },
        32_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShr,
            result_type: FheType::Uint16,
            scalar: true,
        },
        32_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShr,
            result_type: FheType::Uint32,
            scalar: true,
        },
        32_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShr,
            result_type: FheType::Uint64,
            scalar: true,
        },
        34_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShr,
            result_type: FheType::Uint128,
            scalar: true,
        },
        37_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShr,
            result_type: FheType::Uint256,
            scalar: true,
        },
        38_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShr,
            result_type: FheType::Uint8,
            scalar: false,
        },
        91_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShr,
            result_type: FheType::Uint16,
            scalar: false,
        },
        123_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShr,
            result_type: FheType::Uint32,
            scalar: false,
        },
        163_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShr,
            result_type: FheType::Uint64,
            scalar: false,
        },
        209_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShr,
            result_type: FheType::Uint128,
            scalar: false,
        },
        272_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheShr,
            result_type: FheType::Uint256,
            scalar: false,
        },
        369_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotl,
            result_type: FheType::Uint8,
            scalar: true,
        },
        31_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotl,
            result_type: FheType::Uint16,
            scalar: true,
        },
        31_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotl,
            result_type: FheType::Uint32,
            scalar: true,
        },
        32_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotl,
            result_type: FheType::Uint64,
            scalar: true,
        },
        34_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotl,
            result_type: FheType::Uint128,
            scalar: true,
        },
        37_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotl,
            result_type: FheType::Uint256,
            scalar: true,
        },
        38_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotl,
            result_type: FheType::Uint8,
            scalar: false,
        },
        91_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotl,
            result_type: FheType::Uint16,
            scalar: false,
        },
        125_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotl,
            result_type: FheType::Uint32,
            scalar: false,
        },
        163_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotl,
            result_type: FheType::Uint64,
            scalar: false,
        },
        209_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotl,
            result_type: FheType::Uint128,
            scalar: false,
        },
        278_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotl,
            result_type: FheType::Uint256,
            scalar: false,
        },
        378_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotr,
            result_type: FheType::Uint8,
            scalar: true,
        },
        31_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotr,
            result_type: FheType::Uint16,
            scalar: true,
        },
        31_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotr,
            result_type: FheType::Uint32,
            scalar: true,
        },
        32_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotr,
            result_type: FheType::Uint64,
            scalar: true,
        },
        34_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotr,
            result_type: FheType::Uint128,
            scalar: true,
        },
        37_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotr,
            result_type: FheType::Uint256,
            scalar: true,
        },
        40_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotr,
            result_type: FheType::Uint8,
            scalar: false,
        },
        93_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotr,
            result_type: FheType::Uint16,
            scalar: false,
        },
        125_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotr,
            result_type: FheType::Uint32,
            scalar: false,
        },
        160_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotr,
            result_type: FheType::Uint64,
            scalar: false,
        },
        209_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotr,
            result_type: FheType::Uint128,
            scalar: false,
        },
        283_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRotr,
            result_type: FheType::Uint256,
            scalar: false,
        },
        375_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheEq,
            result_type: FheType::Bool,
            scalar: true,
        },
        25_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheEq,
            result_type: FheType::Uint8,
            scalar: true,
        },
        55_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheEq,
            result_type: FheType::Uint16,
            scalar: true,
        },
        55_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheEq,
            result_type: FheType::Uint32,
            scalar: true,
        },
        82_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheEq,
            result_type: FheType::Uint64,
            scalar: true,
        },
        83_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheEq,
            result_type: FheType::Uint128,
            scalar: true,
        },
        117_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheEq,
            result_type: FheType::Uint160,
            scalar: true,
        },
        117_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheEq,
            result_type: FheType::Uint256,
            scalar: true,
        },
        118_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheEq,
            result_type: FheType::Bool,
            scalar: false,
        },
        26_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheEq,
            result_type: FheType::Uint8,
            scalar: false,
        },
        55_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheEq,
            result_type: FheType::Uint16,
            scalar: false,
        },
        83_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheEq,
            result_type: FheType::Uint32,
            scalar: false,
        },
        86_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheEq,
            result_type: FheType::Uint64,
            scalar: false,
        },
        120_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheEq,
            result_type: FheType::Uint128,
            scalar: false,
        },
        122_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheEq,
            result_type: FheType::Uint160,
            scalar: false,
        },
        137_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheEq,
            result_type: FheType::Uint256,
            scalar: false,
        },
        152_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNe,
            result_type: FheType::Bool,
            scalar: true,
        },
        23_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNe,
            result_type: FheType::Uint8,
            scalar: true,
        },
        55_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNe,
            result_type: FheType::Uint16,
            scalar: true,
        },
        55_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNe,
            result_type: FheType::Uint32,
            scalar: true,
        },
        83_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNe,
            result_type: FheType::Uint64,
            scalar: true,
        },
        84_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNe,
            result_type: FheType::Uint128,
            scalar: true,
        },
        117_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNe,
            result_type: FheType::Uint160,
            scalar: true,
        },
        117_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNe,
            result_type: FheType::Uint256,
            scalar: true,
        },
        117_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNe,
            result_type: FheType::Bool,
            scalar: false,
        },
        23_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNe,
            result_type: FheType::Uint8,
            scalar: false,
        },
        55_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNe,
            result_type: FheType::Uint16,
            scalar: false,
        },
        83_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNe,
            result_type: FheType::Uint32,
            scalar: false,
        },
        85_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNe,
            result_type: FheType::Uint64,
            scalar: false,
        },
        118_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNe,
            result_type: FheType::Uint128,
            scalar: false,
        },
        122_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNe,
            result_type: FheType::Uint160,
            scalar: false,
        },
        136_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNe,
            result_type: FheType::Uint256,
            scalar: false,
        },
        150_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheGe,
            result_type: FheType::Uint8,
            scalar: true,
        },
        52_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheGe,
            result_type: FheType::Uint16,
            scalar: true,
        },
        55_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheGe,
            result_type: FheType::Uint32,
            scalar: true,
        },
        84_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheGe,
            result_type: FheType::Uint64,
            scalar: true,
        },
        116_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheGe,
            result_type: FheType::Uint128,
            scalar: true,
        },
        149_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheGe,
            result_type: FheType::Uint8,
            scalar: false,
        },
        63_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheGe,
            result_type: FheType::Uint16,
            scalar: false,
        },
        84_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheGe,
            result_type: FheType::Uint32,
            scalar: false,
        },
        118_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheGe,
            result_type: FheType::Uint64,
            scalar: false,
        },
        152_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheGe,
            result_type: FheType::Uint128,
            scalar: false,
        },
        210_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheGt,
            result_type: FheType::Uint8,
            scalar: true,
        },
        52_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheGt,
            result_type: FheType::Uint16,
            scalar: true,
        },
        55_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheGt,
            result_type: FheType::Uint32,
            scalar: true,
        },
        84_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheGt,
            result_type: FheType::Uint64,
            scalar: true,
        },
        117_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheGt,
            result_type: FheType::Uint128,
            scalar: true,
        },
        150_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheGt,
            result_type: FheType::Uint8,
            scalar: false,
        },
        59_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheGt,
            result_type: FheType::Uint16,
            scalar: false,
        },
        84_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheGt,
            result_type: FheType::Uint32,
            scalar: false,
        },
        118_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheGt,
            result_type: FheType::Uint64,
            scalar: false,
        },
        152_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheGt,
            result_type: FheType::Uint128,
            scalar: false,
        },
        218_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheLe,
            result_type: FheType::Uint8,
            scalar: true,
        },
        58_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheLe,
            result_type: FheType::Uint16,
            scalar: true,
        },
        58_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheLe,
            result_type: FheType::Uint32,
            scalar: true,
        },
        84_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheLe,
            result_type: FheType::Uint64,
            scalar: true,
        },
        119_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheLe,
            result_type: FheType::Uint128,
            scalar: true,
        },
        150_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheLe,
            result_type: FheType::Uint8,
            scalar: false,
        },
        58_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheLe,
            result_type: FheType::Uint16,
            scalar: false,
        },
        83_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheLe,
            result_type: FheType::Uint32,
            scalar: false,
        },
        117_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheLe,
            result_type: FheType::Uint64,
            scalar: false,
        },
        149_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheLe,
            result_type: FheType::Uint128,
            scalar: false,
        },
        218_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheLt,
            result_type: FheType::Uint8,
            scalar: true,
        },
        52_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheLt,
            result_type: FheType::Uint16,
            scalar: true,
        },
        58_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheLt,
            result_type: FheType::Uint32,
            scalar: true,
        },
        83_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheLt,
            result_type: FheType::Uint64,
            scalar: true,
        },
        118_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheLt,
            result_type: FheType::Uint128,
            scalar: true,
        },
        149_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheLt,
            result_type: FheType::Uint8,
            scalar: false,
        },
        59_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheLt,
            result_type: FheType::Uint16,
            scalar: false,
        },
        84_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheLt,
            result_type: FheType::Uint32,
            scalar: false,
        },
        117_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheLt,
            result_type: FheType::Uint64,
            scalar: false,
        },
        146_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheLt,
            result_type: FheType::Uint128,
            scalar: false,
        },
        215_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMin,
            result_type: FheType::Uint8,
            scalar: true,
        },
        84_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMin,
            result_type: FheType::Uint16,
            scalar: true,
        },
        88_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMin,
            result_type: FheType::Uint32,
            scalar: true,
        },
        117_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMin,
            result_type: FheType::Uint64,
            scalar: true,
        },
        150_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMin,
            result_type: FheType::Uint128,
            scalar: true,
        },
        186_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMin,
            result_type: FheType::Uint8,
            scalar: false,
        },
        119_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMin,
            result_type: FheType::Uint16,
            scalar: false,
        },
        146_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMin,
            result_type: FheType::Uint32,
            scalar: false,
        },
        182_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMin,
            result_type: FheType::Uint64,
            scalar: false,
        },
        219_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMin,
            result_type: FheType::Uint128,
            scalar: false,
        },
        289_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMax,
            result_type: FheType::Uint8,
            scalar: true,
        },
        89_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMax,
            result_type: FheType::Uint16,
            scalar: true,
        },
        89_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMax,
            result_type: FheType::Uint32,
            scalar: true,
        },
        117_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMax,
            result_type: FheType::Uint64,
            scalar: true,
        },
        149_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMax,
            result_type: FheType::Uint128,
            scalar: true,
        },
        180_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMax,
            result_type: FheType::Uint8,
            scalar: false,
        },
        121_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMax,
            result_type: FheType::Uint16,
            scalar: false,
        },
        145_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMax,
            result_type: FheType::Uint32,
            scalar: false,
        },
        180_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMax,
            result_type: FheType::Uint64,
            scalar: false,
        },
        218_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheMax,
            result_type: FheType::Uint128,
            scalar: false,
        },
        290_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNeg,
            result_type: FheType::Uint8,
            scalar: false,
        },
        79_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNeg,
            result_type: FheType::Uint16,
            scalar: false,
        },
        93_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNeg,
            result_type: FheType::Uint32,
            scalar: false,
        },
        95_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNeg,
            result_type: FheType::Uint64,
            scalar: false,
        },
        131_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNeg,
            result_type: FheType::Uint128,
            scalar: false,
        },
        168_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNeg,
            result_type: FheType::Uint256,
            scalar: false,
        },
        269_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNot,
            result_type: FheType::Bool,
            scalar: false,
        },
        2,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNot,
            result_type: FheType::Uint8,
            scalar: false,
        },
        9,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNot,
            result_type: FheType::Uint16,
            scalar: false,
        },
        16,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNot,
            result_type: FheType::Uint32,
            scalar: false,
        },
        32,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNot,
            result_type: FheType::Uint64,
            scalar: false,
        },
        63,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNot,
            result_type: FheType::Uint128,
            scalar: false,
        },
        130,
    ),
    (
        HcuOperationKey {
            op: Operator::FheNot,
            result_type: FheType::Uint256,
            scalar: false,
        },
        130,
    ),
    (
        HcuOperationKey {
            op: Operator::Cast,
            result_type: FheType::Bool,
            scalar: false,
        },
        32,
    ),
    (
        HcuOperationKey {
            op: Operator::Cast,
            result_type: FheType::Uint8,
            scalar: false,
        },
        32,
    ),
    (
        HcuOperationKey {
            op: Operator::Cast,
            result_type: FheType::Uint16,
            scalar: false,
        },
        32,
    ),
    (
        HcuOperationKey {
            op: Operator::Cast,
            result_type: FheType::Uint32,
            scalar: false,
        },
        32,
    ),
    (
        HcuOperationKey {
            op: Operator::Cast,
            result_type: FheType::Uint64,
            scalar: false,
        },
        32,
    ),
    (
        HcuOperationKey {
            op: Operator::Cast,
            result_type: FheType::Uint128,
            scalar: false,
        },
        32,
    ),
    (
        HcuOperationKey {
            op: Operator::Cast,
            result_type: FheType::Uint256,
            scalar: false,
        },
        32,
    ),
    (
        HcuOperationKey {
            op: Operator::TrivialEncrypt,
            result_type: FheType::Bool,
            scalar: false,
        },
        32,
    ),
    (
        HcuOperationKey {
            op: Operator::TrivialEncrypt,
            result_type: FheType::Uint8,
            scalar: false,
        },
        32,
    ),
    (
        HcuOperationKey {
            op: Operator::TrivialEncrypt,
            result_type: FheType::Uint16,
            scalar: false,
        },
        32,
    ),
    (
        HcuOperationKey {
            op: Operator::TrivialEncrypt,
            result_type: FheType::Uint32,
            scalar: false,
        },
        32,
    ),
    (
        HcuOperationKey {
            op: Operator::TrivialEncrypt,
            result_type: FheType::Uint64,
            scalar: false,
        },
        32,
    ),
    (
        HcuOperationKey {
            op: Operator::TrivialEncrypt,
            result_type: FheType::Uint128,
            scalar: false,
        },
        32,
    ),
    (
        HcuOperationKey {
            op: Operator::TrivialEncrypt,
            result_type: FheType::Uint160,
            scalar: false,
        },
        32,
    ),
    (
        HcuOperationKey {
            op: Operator::TrivialEncrypt,
            result_type: FheType::Uint256,
            scalar: false,
        },
        32,
    ),
    (
        HcuOperationKey {
            op: Operator::FheIfThenElse,
            result_type: FheType::Bool,
            scalar: false,
        },
        55_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheIfThenElse,
            result_type: FheType::Uint8,
            scalar: false,
        },
        55_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheIfThenElse,
            result_type: FheType::Uint16,
            scalar: false,
        },
        55_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheIfThenElse,
            result_type: FheType::Uint32,
            scalar: false,
        },
        55_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheIfThenElse,
            result_type: FheType::Uint64,
            scalar: false,
        },
        55_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheIfThenElse,
            result_type: FheType::Uint128,
            scalar: false,
        },
        57_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheIfThenElse,
            result_type: FheType::Uint160,
            scalar: false,
        },
        83_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheIfThenElse,
            result_type: FheType::Uint256,
            scalar: false,
        },
        108_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRand,
            result_type: FheType::Bool,
            scalar: false,
        },
        19_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRand,
            result_type: FheType::Uint8,
            scalar: false,
        },
        23_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRand,
            result_type: FheType::Uint16,
            scalar: false,
        },
        23_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRand,
            result_type: FheType::Uint32,
            scalar: false,
        },
        24_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRand,
            result_type: FheType::Uint64,
            scalar: false,
        },
        24_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRand,
            result_type: FheType::Uint128,
            scalar: false,
        },
        25_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRand,
            result_type: FheType::Uint256,
            scalar: false,
        },
        30_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRandBounded,
            result_type: FheType::Uint8,
            scalar: false,
        },
        23_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRandBounded,
            result_type: FheType::Uint16,
            scalar: false,
        },
        23_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRandBounded,
            result_type: FheType::Uint32,
            scalar: false,
        },
        24_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRandBounded,
            result_type: FheType::Uint64,
            scalar: false,
        },
        24_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRandBounded,
            result_type: FheType::Uint128,
            scalar: false,
        },
        25_000,
    ),
    (
        HcuOperationKey {
            op: Operator::FheRandBounded,
            result_type: FheType::Uint256,
            scalar: false,
        },
        30_000,
    ),
];

fn default_pricing(key: HcuOperationKey) -> Option<u64> {
    DEFAULT_PRICING
        .iter()
        .find_map(|(default_key, cost)| (*default_key == key).then_some(*cost))
}
