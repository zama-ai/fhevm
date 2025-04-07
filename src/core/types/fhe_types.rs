use alloy::{
    hex,
    primitives::{Address, Bytes, U256},
};
use kms_grpc::kms::v1::{FheType, TypedPlaintext};
use tracing::{error, info};

/// Get string representation of FHE type
pub fn fhe_type_to_string(fhe_type: i32) -> &'static str {
    match fhe_type {
        t if t == FheType::Ebool as i32 => "EBOOL",
        t if t == FheType::Euint4 as i32 => "EUINT4",
        t if t == FheType::Euint8 as i32 => "EUINT8",
        t if t == FheType::Euint16 as i32 => "EUINT16",
        t if t == FheType::Euint32 as i32 => "EUINT32",
        t if t == FheType::Euint64 as i32 => "EUINT64",
        t if t == FheType::Euint128 as i32 => "EUINT128",
        t if t == FheType::Euint160 as i32 => "EUINT160",
        t if t == FheType::Euint256 as i32 => "EUINT256",
        t if t == FheType::Euint512 as i32 => "EUINT512",
        t if t == FheType::Euint1024 as i32 => "EUINT1024",
        t if t == FheType::Euint2048 as i32 => "EUINT2048",
        _ => "UNKNOWN",
    }
}

/// Extract FHE type from handle bytes
pub fn extract_fhe_type_from_handle(bytes: &[u8]) -> i32 {
    // Format: keccak256(keccak256(bundleCiphertext)+index)[0:29] + index + type + version
    // - Last byte (31): Version (currently 0)
    // - Second-to-last byte (30): FHE Type
    // - Third-to-last byte (29): Handle index
    // - Rest (0-28): Hash data
    if bytes.len() >= 32 {
        let type_byte = bytes[30]; // FHE type is at index 30

        if type_byte >= 12 {
            error!("Unknown FHE type byte: {}, must be less than 12", type_byte);
            return FheType::Ebool as i32;
        }

        match type_byte {
            0 => FheType::Ebool as i32,
            1 => FheType::Euint4 as i32,
            2 => FheType::Euint8 as i32,
            3 => FheType::Euint16 as i32,
            4 => FheType::Euint32 as i32,
            5 => FheType::Euint64 as i32,
            6 => FheType::Euint128 as i32,
            7 => FheType::Euint160 as i32,
            8 => FheType::Euint256 as i32,
            9 => FheType::Euint512 as i32,
            10 => FheType::Euint1024 as i32,
            11 => FheType::Euint2048 as i32,
            _ => unreachable!(), // We checked type_byte < 12 above
        }
    } else {
        error!("Handle too short: {} bytes, expected 32 bytes", bytes.len());
        FheType::Ebool as i32
    }
}

/// Extract FHE type and log result details
pub fn log_and_extract_result<T>(
    _result: &T,
    fhe_type: i32,
    request_id: U256,
    user_addr: Option<Address>,
) where
    T: AsRef<[u8]>,
{
    let fhe_type_str = fhe_type_to_string(fhe_type);

    match user_addr {
        Some(addr) => info!(
            "Reencrypted result type: {} for request {} (user: 0x{})",
            fhe_type_str,
            request_id,
            hex::encode(addr)
        ),
        None => info!(
            "Decrypted result type: {} for request {}",
            fhe_type_str, request_id
        ),
    }
}

/// Convert a U256 request ID to a valid hex format that KMS Core expects
/// The KMS Core expects a hex string that decodes to exactly 32 bytes
pub fn format_request_id(request_id: U256) -> String {
    // Convert U256 to big-endian bytes
    let bytes = request_id.to_be_bytes::<32>();
    // Encode as hex string
    hex::encode(bytes)
}

/// ABI encode multiple plaintexts into a single Bytes object
/// This follows the Solidity ABI encoding for dynamic arrays, matching the KMS Core implementation
pub fn abi_encode_plaintexts(plaintexts: &[TypedPlaintext]) -> Bytes {
    use alloy_dyn_abi::DynSolValue;

    // This is a hack to get the offsets right for Byte types.
    // Every offset needs to be shifted by 32 bytes (256 bits), so we prepend a U256 and delete it at the and, after encoding.
    let mut data = vec![DynSolValue::Uint(U256::from(0), 256)];

    // This is another hack to handle Euint512, Euint1024 and Euint2048 Bytes properly (alloy adds another all-zero 256 bytes to the beginning of the encoded bytes)
    let mut offset_mul = 1;

    for ptxt in plaintexts.iter() {
        info!("Encoding Plaintext with FheType: {:#?}", ptxt.fhe_type());
        let res = match ptxt.fhe_type() {
            FheType::Ebool => {
                let val = if ptxt.as_bool() { 1_u8 } else { 0 };
                DynSolValue::Uint(U256::from(val), 256)
            }
            FheType::Euint4 => DynSolValue::Uint(U256::from(ptxt.as_u4()), 256),
            FheType::Euint8 => DynSolValue::Uint(U256::from(ptxt.as_u8()), 256),
            FheType::Euint16 => DynSolValue::Uint(U256::from(ptxt.as_u16()), 256),
            FheType::Euint32 => DynSolValue::Uint(U256::from(ptxt.as_u32()), 256),
            FheType::Euint64 => DynSolValue::Uint(U256::from(ptxt.as_u64()), 256),
            FheType::Euint128 => DynSolValue::Uint(U256::from(ptxt.as_u128()), 256),
            FheType::Euint160 => {
                let mut cake = vec![0u8; 32];
                ptxt.as_u160().copy_to_be_byte_slice(cake.as_mut_slice());
                DynSolValue::Uint(U256::from_be_slice(&cake), 256)
            }
            FheType::Euint256 => {
                let mut cake = vec![0u8; 32];
                ptxt.as_u256().copy_to_be_byte_slice(cake.as_mut_slice());
                DynSolValue::Uint(U256::from_be_slice(&cake), 256)
            }
            FheType::Euint512 => {
                // if we have at least 1 Euint larger than 256 bits, we need to throw away 256 more bytes at the beginning of the encoding below, thus set offset_mul to 2
                offset_mul = 2;
                let mut cake = vec![0u8; 64];
                ptxt.as_u512().copy_to_be_byte_slice(cake.as_mut_slice());
                DynSolValue::Bytes(cake)
            }
            FheType::Euint1024 => {
                // if we have at least 1 Euint larger than 256 bits, we need to throw away 256 more bytes at the beginning of the encoding below, thus set offset_mul to 2
                offset_mul = 2;
                let mut cake = vec![0u8; 128];
                ptxt.as_u1024().copy_to_be_byte_slice(cake.as_mut_slice());
                DynSolValue::Bytes(cake)
            }
            FheType::Euint2048 => {
                // if we have at least 1 Euint larger than 256 bits, we need to throw away 256 more bytes at the beginning of the encoding below, thus set offset_mul to 2
                offset_mul = 2;
                let mut cake = vec![0u8; 256];
                ptxt.as_u2048().copy_to_be_byte_slice(cake.as_mut_slice());
                DynSolValue::Bytes(cake)
            }
        };
        data.push(res);
    }

    // wrap data in a Tuple, so we can encode it with position information
    let encoded = DynSolValue::Tuple(data).abi_encode();

    // strip off the extra U256 at the beginning, and possibly also 256 bytes more zero bytes, when we encode one or more Euint2048s
    let encoded_bytes: Vec<u8> = encoded[offset_mul * 32..].to_vec();

    let hexbytes = hex::encode(encoded_bytes.clone());
    info!("Encoded plaintext ABI {:?}", hexbytes);

    Bytes::from(encoded_bytes)
}
