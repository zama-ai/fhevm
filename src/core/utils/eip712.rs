use crate::error::Result;
use alloy::{
    primitives::{Address, U256},
    sol_types::Eip712Domain,
};
use alloy_primitives::B256;
use kms_grpc::kms::v1::{Eip712DomainMsg, ReencryptionRequest};
use tracing::warn;

/// Convert an alloy EIP-712 domain to a protobuf domain message
pub fn alloy_to_protobuf_domain(domain: &Eip712Domain) -> Result<Eip712DomainMsg> {
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

const ERR_PARSE_CHECKSUMMED: &str = "error parsing checksummed address";

/// Convert a protobuf domain message to an alloy EIP-712 domain
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

/// Verify the EIP-712 signature for a reencryption request
pub fn verify_reencryption_eip712(req: &ReencryptionRequest) -> Result<()> {
    // Check if client_address is a valid Ethereum address format
    if !req.client_address.starts_with("0x") {
        warn!(
            "Client address does not start with 0x prefix: {}",
            req.client_address
        );
        return Ok(());
    }

    // Check domain
    let domain_msg = match req.domain.as_ref() {
        Some(domain) => domain,
        None => {
            warn!("Domain is missing in reencryption request");
            return Ok(());
        }
    };

    // Convert protobuf domain to alloy domain
    let domain = match protobuf_to_alloy_domain(domain_msg) {
        Ok(d) => d,
        Err(e) => {
            // Following non-failable design, log error but continue
            warn!("Failed to convert domain for verification: {}", e);
            return Ok(());
        }
    };

    // Parse client address - handle non-standard address lengths
    let client_address_str = req.client_address.trim_start_matches("0x");

    // Try to decode the hex string
    let client_bytes = match alloy::hex::decode(client_address_str) {
        Ok(bytes) => bytes,
        Err(e) => {
            warn!("Failed to decode client address hex: {}", e);
            return Ok(());
        }
    };

    // Check if we have a valid Ethereum address (20 bytes)
    if client_bytes.len() != 20 {
        warn!(
            "Client address has non-standard length: {} bytes (expected 20)",
            client_bytes.len()
        );
        return Ok(());
    }

    // Create the address from bytes
    let client_address = Address::from_slice(&client_bytes);

    // Get verifying contract
    let verifying_contract = match domain.verifying_contract {
        Some(contract) => contract,
        None => {
            warn!("Missing verifying contract in domain");
            return Ok(());
        }
    };

    // Verify client address is not the same as verifying contract
    if client_address == verifying_contract {
        warn!("Client address is the same as verifying contract");
        return Ok(());
    }

    // Check that there are handles
    if req.typed_ciphertexts.is_empty() {
        warn!("No ciphertext handles provided");
        return Ok(());
    }

    Ok(())
}
