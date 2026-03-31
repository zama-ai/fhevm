use anyhow::Result;
use borsh::to_vec;
use solana_host_contracts_core::{FheType, Handle, HostEvent, Operator, Pubkey};
use solana_host_listener::events::{decode_host_event_logs, HOST_EVENT_PREFIX};

#[test]
fn decodes_multiple_host_events_from_logs() -> Result<()> {
    let first = HostEvent::Operation {
        caller: Pubkey::from([1u8; 32]),
        op: Operator::TrivialEncrypt,
        operands: vec![[7u8; 32]],
        scalar_flag: None,
        result_type: FheType::Uint8,
        result: Handle::from([9u8; 32]),
    };
    let second = HostEvent::AllowedForDecryption {
        caller: Pubkey::from([2u8; 32]),
        handles: vec![Handle::from([3u8; 32]), Handle::from([4u8; 32])],
    };

    let logs = vec![
        "Program log: irrelevant".to_owned(),
        format!("{HOST_EVENT_PREFIX}{}", hex::encode(to_vec(&first)?)),
        format!("{HOST_EVENT_PREFIX}{}", hex::encode(to_vec(&second)?)),
    ];

    let decoded = decode_host_event_logs(logs.iter().map(String::as_str))?;
    assert_eq!(decoded.len(), 2);
    assert_eq!(decoded[0].log_index, 1);
    assert_eq!(decoded[0].event, first);
    assert_eq!(decoded[1].log_index, 2);
    assert_eq!(decoded[1].event, second);

    Ok(())
}
