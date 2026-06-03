//! Helpers for inspecting native Solana Ed25519 verify instruction data.

const SIGNATURE_OFFSETS_SERIALIZED_SIZE: usize = 14;
const SIGNATURE_OFFSETS_START: usize = 2;
const PUBKEY_SERIALIZED_SIZE: usize = 32;
const SIGNATURE_SERIALIZED_SIZE: usize = 64;

/// Returns true when one signature tuple in `data` verifies `expected_message`
/// against `expected_pubkey` within the same Ed25519 instruction.
pub fn instruction_contains_message(
    data: &[u8],
    expected_pubkey: &[u8],
    expected_message: &[u8],
) -> bool {
    if data.len() < SIGNATURE_OFFSETS_START || data[1] != 0 {
        return false;
    }
    let signature_count = data[0] as usize;
    if signature_count == 0 {
        return false;
    }

    let expected_offsets_end = SIGNATURE_OFFSETS_START
        .saturating_add(signature_count.saturating_mul(SIGNATURE_OFFSETS_SERIALIZED_SIZE));
    if data.len() < expected_offsets_end {
        return false;
    }

    for signature_index in 0..signature_count {
        let start = SIGNATURE_OFFSETS_START
            .saturating_add(signature_index.saturating_mul(SIGNATURE_OFFSETS_SERIALIZED_SIZE));
        let fields = &data[start..start + SIGNATURE_OFFSETS_SERIALIZED_SIZE];
        let signature_offset = read_u16_le(fields, 0) as usize;
        let signature_instruction_index = read_u16_le(fields, 2);
        let public_key_offset = read_u16_le(fields, 4) as usize;
        let public_key_instruction_index = read_u16_le(fields, 6);
        let message_data_offset = read_u16_le(fields, 8) as usize;
        let message_data_size = read_u16_le(fields, 10) as usize;
        let message_instruction_index = read_u16_le(fields, 12);

        if signature_instruction_index != u16::MAX
            || public_key_instruction_index != u16::MAX
            || message_instruction_index != u16::MAX
        {
            continue;
        }
        let Some(signature_end) = signature_offset.checked_add(SIGNATURE_SERIALIZED_SIZE) else {
            continue;
        };
        let Some(public_key_end) = public_key_offset.checked_add(PUBKEY_SERIALIZED_SIZE) else {
            continue;
        };
        let Some(message_end) = message_data_offset.checked_add(message_data_size) else {
            continue;
        };
        if signature_end > data.len() || public_key_end > data.len() || message_end > data.len() {
            continue;
        }
        if &data[public_key_offset..public_key_end] != expected_pubkey {
            continue;
        }
        if &data[message_data_offset..message_end] == expected_message {
            return true;
        }
    }
    false
}

fn read_u16_le(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([data[offset], data[offset + 1]])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_u16_le(data: &mut [u8], offset: usize, value: u16) {
        data[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
    }

    fn instruction_data(tuples: &[(&[u8; PUBKEY_SERIALIZED_SIZE], &[u8])]) -> Vec<u8> {
        let signature_count =
            u8::try_from(tuples.len()).expect("signature tuple count must fit in u8");
        let offsets_end = SIGNATURE_OFFSETS_START
            + usize::from(signature_count).saturating_mul(SIGNATURE_OFFSETS_SERIALIZED_SIZE);
        let mut data = vec![0; offsets_end];
        data[0] = signature_count;
        data[1] = 0;

        for (signature_index, (public_key, message)) in tuples.iter().enumerate() {
            let signature_offset = data.len();
            data.extend_from_slice(&[0; SIGNATURE_SERIALIZED_SIZE]);
            let public_key_offset = data.len();
            data.extend_from_slice(public_key.as_slice());
            let message_data_offset = data.len();
            data.extend_from_slice(message);

            let fields_offset =
                SIGNATURE_OFFSETS_START + signature_index * SIGNATURE_OFFSETS_SERIALIZED_SIZE;
            write_u16_le(
                &mut data,
                fields_offset,
                u16::try_from(signature_offset).expect("signature offset must fit in u16"),
            );
            write_u16_le(&mut data, fields_offset + 2, u16::MAX);
            write_u16_le(
                &mut data,
                fields_offset + 4,
                u16::try_from(public_key_offset).expect("public key offset must fit in u16"),
            );
            write_u16_le(&mut data, fields_offset + 6, u16::MAX);
            write_u16_le(
                &mut data,
                fields_offset + 8,
                u16::try_from(message_data_offset).expect("message offset must fit in u16"),
            );
            write_u16_le(
                &mut data,
                fields_offset + 10,
                u16::try_from(message.len()).expect("message length must fit in u16"),
            );
            write_u16_le(&mut data, fields_offset + 12, u16::MAX);
        }

        data
    }

    #[test]
    fn accepts_matching_tuple() {
        let public_key = [7; PUBKEY_SERIALIZED_SIZE];
        let message = b"verified-message";
        let data = instruction_data(&[(&public_key, message)]);

        assert!(instruction_contains_message(&data, &public_key, message));
    }

    #[test]
    fn rejects_wrong_public_key() {
        let public_key = [7; PUBKEY_SERIALIZED_SIZE];
        let other_public_key = [8; PUBKEY_SERIALIZED_SIZE];
        let message = b"verified-message";
        let data = instruction_data(&[(&public_key, message)]);

        assert!(!instruction_contains_message(
            &data,
            &other_public_key,
            message
        ));
    }

    #[test]
    fn rejects_cross_instruction_offsets() {
        let public_key = [7; PUBKEY_SERIALIZED_SIZE];
        let message = b"verified-message";
        let mut data = instruction_data(&[(&public_key, message)]);
        write_u16_le(&mut data, SIGNATURE_OFFSETS_START + 12, 0);

        assert!(!instruction_contains_message(&data, &public_key, message));
    }

    #[test]
    fn finds_later_matching_tuple() {
        let first_public_key = [7; PUBKEY_SERIALIZED_SIZE];
        let second_public_key = [8; PUBKEY_SERIALIZED_SIZE];
        let data = instruction_data(&[
            (&first_public_key, b"wrong-message"),
            (&second_public_key, b"right-message"),
        ]);

        assert!(instruction_contains_message(
            &data,
            &second_public_key,
            b"right-message"
        ));
    }

    #[test]
    fn rejects_truncated_instruction_data() {
        let public_key = [7; PUBKEY_SERIALIZED_SIZE];
        let message = b"verified-message";
        let mut data = instruction_data(&[(&public_key, message)]);
        data.truncate(data.len() - 1);

        assert!(!instruction_contains_message(&data, &public_key, message));
    }
}
