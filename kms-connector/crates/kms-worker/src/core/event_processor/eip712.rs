use alloy::sol_types::Eip712Domain;
use kms_grpc::kms::v1::Eip712DomainMsg;

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
