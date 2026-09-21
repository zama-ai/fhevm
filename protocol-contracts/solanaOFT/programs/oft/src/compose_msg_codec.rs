const NONCE_OFFSET: usize = 0;
const SRC_EID_OFFSET: usize = 8;
const AMOUNT_LD_OFFSET: usize = 12;
const COMPOSE_FROM_OFFSET: usize = 20;
const COMPOSE_MSG_OFFSET: usize = 52;

pub fn encode(
    nonce: u64,
    src_eid: u32,
    amount_ld: u64,
    compose_msg: &Vec<u8>, // [composeFrom][composeMsg]
) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(20 + compose_msg.len()); // 8 + 4 + 8
    encoded.extend_from_slice(&nonce.to_be_bytes());
    encoded.extend_from_slice(&src_eid.to_be_bytes());
    encoded.extend_from_slice(&amount_ld.to_be_bytes());
    encoded.extend_from_slice(&compose_msg);
    encoded
}

pub fn nonce(message: &[u8]) -> u64 {
    let mut nonce_bytes = [0; 8];
    nonce_bytes.copy_from_slice(&message[NONCE_OFFSET..SRC_EID_OFFSET]);
    u64::from_be_bytes(nonce_bytes)
}

pub fn src_eid(message: &[u8]) -> u32 {
    let mut src_eid_bytes = [0; 4];
    src_eid_bytes.copy_from_slice(&message[SRC_EID_OFFSET..AMOUNT_LD_OFFSET]);
    u32::from_be_bytes(src_eid_bytes)
}

pub fn amount_ld(message: &[u8]) -> u64 {
    let mut amount_ld_bytes = [0; 8];
    amount_ld_bytes.copy_from_slice(&message[AMOUNT_LD_OFFSET..COMPOSE_FROM_OFFSET]);
    u64::from_be_bytes(amount_ld_bytes)
}

pub fn compose_from(message: &[u8]) -> [u8; 32] {
    let mut compose_from = [0; 32];
    compose_from.copy_from_slice(&message[COMPOSE_FROM_OFFSET..COMPOSE_MSG_OFFSET]);
    compose_from
}

pub fn compose_msg(message: &[u8]) -> Vec<u8> {
    if message.len() > COMPOSE_MSG_OFFSET {
        message[COMPOSE_MSG_OFFSET..].to_vec()
    } else {
        Vec::new()
    }
}
