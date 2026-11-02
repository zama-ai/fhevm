//! One pad decryption end to end: a user signs the RFC-016 permit for two fresh witnesses, posts
//! `POST /v4/pad-decrypt` to the relayer, then checks the answer as the SDK will: every transport signature and
//! share signature is a distinct KMS signer of the context, the shares of each label agree on their session and
//! their bytes, and the pad they carry is the cleartext PRF stream of its label.

use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

use alloy::primitives::{Address, B128, B256, Bytes, Signature, U256, keccak256};
use alloy::signers::SignerSync;
use alloy::signers::local::PrivateKeySigner;
use alloy::sol_types::SolStruct;
use kms_connector_api::PadDecryptionResponse;
use relayer_http::endpoint::flows::pad_decrypt::{PadDecryptPayload, PadEntry, PadWitness, label};
use relayer_http::endpoint::flows::user_decrypt::RequestValidity;
use serde_json::{Value, json};
use url::Url;

use crate::chain::{
    PadDecryptResponseVerification, PadShareVerification, Rpc, UserDecryptRequestVerification,
    cleartextDecryptPadPayloadCall, domain, eip712DomainCall, getKmsSignersForContextCall,
};

/// Everything the flow needs; see `main.rs` for the flags.
pub struct FlowConfig {
    pub relayer: Url,
    pub rpc: Rpc,
    pub padded_inputter: Address,
    pub kms_verifier: Address,
    pub protocol_config: Address,
    pub user: PrivateKeySigner,
    pub extra_data: Bytes,
    /// Distinct consistent shares needed per label (`2t + 1`).
    pub threshold: usize,
}

/// The PRF instance of the cleartext stack (`LocalHostBootstrap.PAD_PRF_INSTANCE`).
const PRF_ID: u16 = 1;
/// The pad lengths requested, one pad each: within one block, and across two.
const PAD_LENGTHS: [u16; 2] = [64, 300];
/// The consumer named by every witness and the only allowed contract (any address but the user's).
const CONSUMER: Address = Address::repeat_byte(0xc0);

/// Runs the flow; `Err` names the first failed check.
pub async fn run(config: &FlowConfig) -> Result<(), String> {
    let (chain_id, block_time) = config
        .rpc
        .chain_and_time()
        .await
        .map_err(|e| e.to_string())?;
    let reported = config
        .rpc
        .call(config.kms_verifier, &eip712DomainCall {})
        .await
        .map_err(|e| format!("KMSVerifier.eip712Domain: {e}"))?;
    let context = B256::from_slice(config.extra_data.get(1..33).ok_or("extraData is not v2")?);
    let signers = config
        .rpc
        .call(
            config.protocol_config,
            &getKmsSignersForContextCall {
                contextId: context.into(),
            },
        )
        .await
        .map_err(|e| format!("ProtocolConfig.getKmsSignersForContext: {e}"))?;

    let user = config.user.address();
    let seed = keccak256(format!("{user}{block_time}{:?}", Instant::now()));
    let public_key = Bytes::from(seed.to_vec());
    let payload = PadDecryptPayload {
        user_address: user,
        allowed_contracts: vec![CONSUMER],
        request_validity: RequestValidity {
            start_timestamp: block_time,
            duration_seconds: 3600,
        },
        public_key,
        extra_data: config.extra_data.clone(),
        witness_version: 0,
        prf_id: PRF_ID,
        chain_id,
        pads: Vec::new(),
    };
    let pads: Vec<PadEntry> = PAD_LENGTHS
        .iter()
        .enumerate()
        .map(|(i, &pad_length)| {
            let witness = PadWitness {
                submitter: user,
                consumer: CONSUMER,
                expiration_timestamp: block_time + 7 * 24 * 3600,
                decryptors: vec![],
                nonce: B128::from_slice(&keccak256([seed.as_slice(), &[i as u8]].concat())[..16]),
            };
            PadEntry {
                label: label(&payload, &witness),
                witness,
                pad_length,
            }
        })
        .collect();

    let permit = UserDecryptRequestVerification {
        userAddress: user,
        publicKey: payload.public_key.clone(),
        allowedContracts: payload.allowed_contracts.clone(),
        startTimestamp: U256::from(block_time),
        durationSeconds: U256::from(3600u64),
        extraData: payload.extra_data.clone(),
    };
    let digest = permit.eip712_signing_hash(&domain(&reported, U256::from(chain_id)));
    let signature = config
        .user
        .sign_hash_sync(&digest)
        .map_err(|e| e.to_string())?;
    let body = wire(&payload, &pads, &Bytes::from(signature.as_bytes().to_vec()));

    println!(
        "user {user}, chain {chain_id}, {} pads ({PAD_LENGTHS:?} bits), {} KMS signers",
        pads.len(),
        signers.len()
    );
    let started = Instant::now();
    let reply = reqwest::Client::new()
        .post(
            config
                .relayer
                .join("v4/pad-decrypt")
                .map_err(|e| e.to_string())?,
        )
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("relayer unreachable: {e}"))?;
    let status = reply.status();
    let reply: Value = reply.json().await.map_err(|e| e.to_string())?;
    println!(
        "POST /v4/pad-decrypt -> {status} in {:?}",
        started.elapsed()
    );
    if !status.is_success() {
        return Err(format!("relayer answered {status}: {reply}"));
    }
    let responses: Vec<PadDecryptionResponse> =
        serde_json::from_value(reply["result"]["responses"].clone())
            .map_err(|e| format!("result.responses: {e}"))?;

    let gateway = domain(&reported, reported.chainId);
    let mut nodes = BTreeSet::new();
    let mut by_label: BTreeMap<B256, Vec<(Address, crate::chain::PadShare)>> = BTreeMap::new();
    for response in &responses {
        let node = check_response(config, response, &gateway, &signers, &payload).await?;
        if !nodes.insert(node.0) {
            return Err(format!("two responses from {}", node.0));
        }
        for share in node.1 {
            by_label
                .entry(share.label)
                .or_default()
                .push((node.0, share));
        }
    }
    for pad in &pads {
        let shares = by_label
            .get(&pad.label)
            .ok_or_else(|| format!("no share for label {}", pad.label))?;
        check_label(pad, shares, config.threshold)?;
        println!(
            "label {}: {} shares, one session, pad {} bits ok",
            pad.label,
            shares.len(),
            pad.pad_length
        );
    }
    println!(
        "pad decryption ok: {} responses from {} distinct KMS signers",
        responses.len(),
        nodes.len()
    );
    Ok(())
}

/// One node's response: its transport signer, then its decrypted shares, each signed by the same signer for the
/// request's context, one per requested label.
async fn check_response(
    config: &FlowConfig,
    response: &PadDecryptionResponse,
    gateway: &alloy::sol_types::Eip712Domain,
    signers: &[Address],
    payload: &PadDecryptPayload,
) -> Result<(Address, Vec<crate::chain::PadShare>), String> {
    let transport = PadDecryptResponseVerification {
        requestId: response.decryption_id,
        encryptedPayload: response.encrypted_payload.clone(),
    }
    .eip712_signing_hash(gateway);
    let node = recover(&response.signature, &transport)?;
    if !signers.contains(&node) {
        return Err(format!("transport signer {node} is not a KMS signer"));
    }
    let decrypted = config
        .rpc
        .call(
            config.padded_inputter,
            &cleartextDecryptPadPayloadCall {
                encryptedPayload: response.encrypted_payload.clone(),
                publicKey: payload.public_key.clone(),
            },
        )
        .await
        .map_err(|e| format!("cleartextDecryptPadPayload: {e}"))?;
    let context = B256::from_slice(&payload.extra_data[1..33]);
    for share in &decrypted.shares {
        let digest = PadShareVerification {
            label: share.label,
            rawShares: share.rawShares.clone(),
            partyId: share.partyId,
            degree: share.degree,
            contextId: share.contextId,
            sessionId: share.sessionId,
            extraData: share.extraData.clone(),
        }
        .eip712_signing_hash(gateway);
        if recover(&share.signature, &digest)? != node {
            return Err(format!("share of {} not signed by {node}", share.label));
        }
        if share.contextId != context {
            return Err(format!(
                "share of {} signed for context {}",
                share.label, share.contextId
            ));
        }
    }
    Ok((node, decrypted.shares))
}

/// The shares of one label: enough of them, one session, one encoding, and the cleartext PRF stream of the label.
fn check_label(
    pad: &PadEntry,
    shares: &[(Address, crate::chain::PadShare)],
    threshold: usize,
) -> Result<(), String> {
    if shares.len() < threshold {
        return Err(format!(
            "label {}: {} shares, {threshold} needed",
            pad.label,
            shares.len()
        ));
    }
    let first = &shares[0].1;
    if shares
        .iter()
        .any(|(_, s)| s.sessionId != first.sessionId || s.rawShares != first.rawShares)
    {
        return Err(format!("label {}: shares disagree", pad.label));
    }
    let expected = [
        &[0u8][..],
        &cleartext_pad(PRF_ID, pad.label, pad.pad_length),
    ]
    .concat();
    if first.rawShares.as_ref() != expected.as_slice() {
        return Err(format!("label {}: not the cleartext PRF stream", pad.label));
    }
    Ok(())
}

/// `LibCleartextPad.cleartextPrfBlocks`: ⌈padLength/256⌉ blocks `keccak256("FHE_cpad" ‖ prfId ‖ label ‖ i_le32)`.
fn cleartext_pad(prf_id: u16, label: B256, pad_length: u16) -> Vec<u8> {
    let blocks = u32::from(pad_length).div_ceil(256);
    (0..blocks)
        .flat_map(|i| {
            keccak256(
                [
                    b"FHE_cpad".as_slice(),
                    &prf_id.to_be_bytes(),
                    label.as_slice(),
                    &i.to_le_bytes(),
                ]
                .concat(),
            )
        })
        .collect()
}

/// The signer of `digest`.
fn recover(signature: &Bytes, digest: &B256) -> Result<Address, String> {
    Signature::try_from(signature.as_ref())
        .and_then(|s| s.recover_address_from_prehash(digest))
        .map_err(|e| format!("bad signature: {e}"))
}

/// The SDK's wire body of `POST /v4/pad-decrypt`.
fn wire(payload: &PadDecryptPayload, pads: &[PadEntry], signature: &Bytes) -> Value {
    json!({
        "attestationType": "eip712-unified-user-decrypt-v1",
        "payload": {
            "witnessVersion": payload.witness_version,
            "prfId": payload.prf_id,
            "chainId": payload.chain_id.to_string(),
            "pads": pads.iter().map(|p| json!({
                "label": p.label,
                "witness": {
                    "submitter": p.witness.submitter,
                    "consumer": p.witness.consumer,
                    "expirationTimestamp": p.witness.expiration_timestamp.to_string(),
                    "decryptors": p.witness.decryptors,
                    "nonce": p.witness.nonce,
                },
                "padLength": p.pad_length,
            })).collect::<Vec<_>>(),
            "userAddress": payload.user_address,
            "publicKey": payload.public_key,
            "allowedContracts": payload.allowed_contracts,
            "requestValidity": {
                "startTimestamp": payload.request_validity.start_timestamp.to_string(),
                "durationSeconds": payload.request_validity.duration_seconds.to_string(),
            },
            "extraData": payload.extra_data,
        },
        "signature": signature,
    })
}

#[cfg(test)]
mod tests {
    use relayer_http::config::HttpConfig;
    use relayer_http::endpoint::flows::pad_decrypt::{PadDecryptRequest, validate};

    use super::*;

    fn payload() -> PadDecryptPayload {
        PadDecryptPayload {
            user_address: Address::repeat_byte(0x55),
            allowed_contracts: vec![CONSUMER],
            request_validity: RequestValidity {
                start_timestamp: 1_800_000_000,
                duration_seconds: 3600,
            },
            public_key: Bytes::from(vec![0x20; 32]),
            extra_data: crate::chain::default_extra_data(),
            witness_version: 0,
            prf_id: PRF_ID,
            chain_id: 31337,
            pads: vec![],
        }
    }

    #[test]
    fn wire_body_is_what_the_relayer_accepts() {
        let payload = payload();
        let witness = PadWitness {
            submitter: payload.user_address,
            consumer: CONSUMER,
            expiration_timestamp: 1_800_604_800,
            decryptors: vec![],
            nonce: B128::repeat_byte(0x77),
        };
        let pad = PadEntry {
            label: label(&payload, &witness),
            witness,
            pad_length: 300,
        };
        let body = wire(&payload, &[pad], &Bytes::from(vec![0x66; 65]));
        let request: PadDecryptRequest = serde_json::from_value(body).unwrap();
        let http: HttpConfig = serde_json::from_value(serde_json::json!({
            "endpoint": "127.0.0.1:0",
            "supported_chain_ids": [31337],
        }))
        .unwrap();
        assert_eq!(validate(&request, &http, 1_800_000_100), Ok(()));
    }

    #[test]
    fn cleartext_pad_has_whole_blocks() {
        let label = B256::repeat_byte(0x01);
        assert_eq!(cleartext_pad(1, label, 64).len(), 32);
        assert_eq!(cleartext_pad(1, label, 256).len(), 32);
        assert_eq!(cleartext_pad(1, label, 300).len(), 64);
        let pad = cleartext_pad(1, label, 300);
        assert_eq!(&pad[..32], cleartext_pad(1, label, 64).as_slice());
        assert_ne!(&pad[..32], &pad[32..]);
    }
}
