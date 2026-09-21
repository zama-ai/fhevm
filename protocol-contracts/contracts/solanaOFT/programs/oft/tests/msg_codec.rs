#[cfg(test)]
mod test_msg_codec {
    use anchor_lang::prelude::Pubkey;
    use oft::compose_msg_codec;
    use oft::msg_codec;

    #[test]
    fn test_msg_codec_with_compose_msg() {
        let send_to: [u8; 32] = [1; 32];
        let amount_sd: u64 = 123456789;
        let sender: Pubkey = Pubkey::new_unique();
        let compose_msg: Option<Vec<u8>> = Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
        let encoded = msg_codec::encode(send_to, amount_sd, sender, &compose_msg);
        assert_eq!(encoded.len(), 72 + compose_msg.clone().unwrap().len());
        assert_eq!(msg_codec::send_to(&encoded), send_to);
        assert_eq!(msg_codec::amount_sd(&encoded), amount_sd);
        assert_eq!(
            msg_codec::compose_msg(&encoded),
            Some([sender.to_bytes().as_ref(), compose_msg.unwrap().as_slice()].concat())
        );
    }

    #[test]
    fn test_msg_codec_without_compose_msg() {
        let send_to: [u8; 32] = [1; 32];
        let amount_sd: u64 = 123456789;
        let sender: Pubkey = Pubkey::new_unique();
        let compose_msg: Option<Vec<u8>> = None;
        let encoded = msg_codec::encode(send_to, amount_sd, sender, &compose_msg);
        assert_eq!(encoded.len(), 40);
        assert_eq!(msg_codec::send_to(&encoded), send_to);
        assert_eq!(msg_codec::amount_sd(&encoded), amount_sd);
        assert_eq!(msg_codec::compose_msg(&encoded), None);
    }

    #[test]
    fn test_compose_msg_codec() {
        let nonce: u64 = 123456789;
        let src_eid: u32 = 987654321;
        let amount_ld: u64 = 123456789;
        let compose_from: [u8; 32] = [1; 32];
        let compose_msg: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let encoded = compose_msg_codec::encode(
            nonce,
            src_eid,
            amount_ld,
            &[&compose_from[..], &compose_msg].concat(),
        );
        assert_eq!(encoded.len(), 20 + [&compose_from[..], &compose_msg].concat().len());
        assert_eq!(compose_msg_codec::nonce(&encoded), nonce);
        assert_eq!(compose_msg_codec::src_eid(&encoded), src_eid);
        assert_eq!(compose_msg_codec::amount_ld(&encoded), amount_ld);
        assert_eq!(compose_msg_codec::compose_msg(&encoded), compose_msg);
    }
}
