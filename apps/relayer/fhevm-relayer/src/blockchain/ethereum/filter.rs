use alloy::primitives::Address;
use alloy::rpc::types::{BlockNumberOrTag, Filter};

pub struct ContractAndTopicsFilter {
    contract_addresses: Vec<Address>,
    _topics: Vec<String>,
}

impl ContractAndTopicsFilter {
    pub fn new(contract_addresses: Vec<Address>, _topics: Vec<String>) -> Self {
        Self {
            contract_addresses,
            _topics,
        }
    }
    pub fn to_eth_subscription_filter(&self, block_number_or_tag: BlockNumberOrTag) -> Filter {
        Filter::new()
            .from_block(block_number_or_tag)
            .address(self.contract_addresses.clone())
    }
}
