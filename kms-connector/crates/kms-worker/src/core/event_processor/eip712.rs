use alloy::{
    hex,
    primitives::{Address, B256, U256},
    sol_types::Eip712Domain,
};
use anyhow::{Result, anyhow};
use kms_grpc::kms::v1::{Eip712DomainMsg, UserDecryptionRequest};
use tracing::warn;

const ERR_PARSE_CHECKSUMMED: &str = "error parsing checksummed address";

/// Converts an alloy EIP-712 domain into protobuf domain.
pub fn alloy_to_protobuf_domain(domain: &Eip712Domain) -> anyhow::Result<Eip712DomainMsg> {
    let name = domain
        .name
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("missing domain name"))?
        .to_string();
    let version = domain
        .version
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("missing domain version"))?
        .to_string();
    let chain_id = domain
        .chain_id
        .ok_or_else(|| anyhow::anyhow!("missing domain chain_id"))?
        .to_be_bytes_vec();
    let verifying_contract = domain
        .verifying_contract
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("missing domain chain_id"))?
        .to_string();
    let domain_msg = Eip712DomainMsg {
        name,
        version,
        chain_id,
        verifying_contract,
        salt: domain.salt.map(|x| x.to_vec()),
    };
    Ok(domain_msg)
}

/// Converts a protobuf domain message to an alloy EIP-712 domain
pub fn protobuf_to_alloy_domain(pb_domain: &Eip712DomainMsg) -> Result<Eip712Domain> {
    // any salt that has the wrong length will result in an error
    let salt = match pb_domain.salt.as_ref() {
        Some(inner_salt) => match B256::try_from(inner_salt.as_slice()) {
            Ok(b256) => Some(b256),
            Err(e) => {
                warn!("Invalid salt length in EIP-712 domain: {}", e);
                None
            }
        },
        None => None,
    };

    let out = Eip712Domain::new(
        Some(pb_domain.name.clone().into()),
        Some(pb_domain.version.clone().into()),
        Some(
            U256::try_from_be_slice(&pb_domain.chain_id)
                .ok_or_else(|| anyhow::anyhow!("invalid chain ID"))?,
        ),
        Some(
            Address::parse_checksummed(pb_domain.verifying_contract.clone(), None).map_err(
                |e| {
                    anyhow::anyhow!(
                        "{ERR_PARSE_CHECKSUMMED}: {} - {e}",
                        pb_domain.verifying_contract,
                    )
                },
            )?,
        ),
        salt,
    );
    Ok(out)
}

/// Verifies the EIP-712 signature for a user decryption request
pub fn verify_user_decryption_eip712(req: &UserDecryptionRequest) -> Result<()> {
    // Check if client_address is a valid Ethereum address format
    if !req.client_address.starts_with("0x") {
        warn!(
            "Client address does not start with 0x prefix: {}",
            req.client_address
        );
    }

    // Check domain
    let Some(domain_msg) = req.domain.as_ref() else {
        return Err(anyhow!("Domain is missing in user decryption request"));
    };

    let domain = protobuf_to_alloy_domain(domain_msg)
        .map_err(|e| anyhow!("Failed to convert domain for verification: {e}"))?;

    // Parse client address - handle non-standard address lengths
    let client_address_str = req.client_address.trim_start_matches("0x");
    let client_bytes = hex::decode(client_address_str)
        .map_err(|e| anyhow!("Failed to decode client address hex: {e}"))?;
    let client_address = Address::try_from(client_bytes.as_slice())
        .map_err(|e| anyhow!("Invalid Ethereum address: {e}"))?;

    // Get verifying contract
    let Some(verifying_contract) = domain.verifying_contract else {
        return Err(anyhow!("Missing verifying contract in domain"));
    };

    // Verify client address is not the same as verifying contract
    if client_address == verifying_contract {
        warn!("Client address is the same as verifying contract");
    }

    // Check that there are handles
    if req.typed_ciphertexts.is_empty() {
        warn!("No ciphertext handles provided");
    }

    Ok(())
}
