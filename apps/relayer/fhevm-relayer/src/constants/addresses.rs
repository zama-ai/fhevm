#[derive(Debug)]
pub struct ContractAddresses;

impl ContractAddresses {
    pub const DECRYPTION_ORACLE: &'static str = "0x67aa98a03CC4559E1e98e7b4Ed071C35c40b588d";

    pub const TFHE_EXECUTOR: &'static str = "0x4e142887e3Dc6e414a9b260a1034D20C9B4Eb11F";

    pub fn validate_address(address: &str) -> bool {
        address.starts_with("0x") && address.len() == 42
    }
}
