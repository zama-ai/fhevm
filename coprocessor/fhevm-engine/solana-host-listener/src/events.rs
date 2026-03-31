use anyhow::{anyhow, Context, Result};
use borsh::BorshDeserialize;
use solana_host_contracts_core::HostEvent;

pub const HOST_EVENT_PREFIX: &str = "HOST_EVENT:";
const SOLANA_PROGRAM_LOG_PREFIX: &str = "Program log: ";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecodedHostEvent {
    pub log_index: usize,
    pub event: HostEvent,
}

pub fn decode_host_event_line(line: &str) -> Result<Option<HostEvent>> {
    let trimmed = line.trim();
    let normalized = trimmed
        .strip_prefix(SOLANA_PROGRAM_LOG_PREFIX)
        .unwrap_or(trimmed);
    let payload = match normalized.strip_prefix(HOST_EVENT_PREFIX) {
        Some(payload) => payload,
        None => return Ok(None),
    };

    let bytes = hex::decode(payload).context("failed to decode HOST_EVENT hex payload")?;
    let event = HostEvent::try_from_slice(&bytes)
        .map_err(|err| anyhow!("failed to decode HostEvent: {err}"))?;
    Ok(Some(event))
}

pub fn decode_host_event_logs<'a, I>(lines: I) -> Result<Vec<DecodedHostEvent>>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut decoded = Vec::new();

    for (log_index, line) in lines.into_iter().enumerate() {
        if let Some(event) = decode_host_event_line(line)? {
            decoded.push(DecodedHostEvent { log_index, event });
        }
    }

    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::{
        decode_host_event_line, decode_host_event_logs, HOST_EVENT_PREFIX,
        SOLANA_PROGRAM_LOG_PREFIX,
    };
    use borsh::to_vec;
    use solana_host_contracts_core::{FheType, Handle, HostEvent, Operator, Pubkey};

    fn sample_event() -> HostEvent {
        HostEvent::Operation {
            caller: Pubkey::from([1u8; 32]),
            op: Operator::FheAdd,
            operands: vec![[2u8; 32], [3u8; 32]],
            scalar_flag: Some(0),
            result_type: FheType::Uint8,
            result: Handle::from([4u8; 32]),
        }
    }

    #[test]
    fn decodes_host_event_line() {
        let encoded = hex::encode(to_vec(&sample_event()).unwrap());
        let line = format!("{HOST_EVENT_PREFIX}{encoded}");

        assert_eq!(decode_host_event_line(&line).unwrap(), Some(sample_event()));
    }

    #[test]
    fn ignores_non_host_event_logs() {
        assert_eq!(decode_host_event_line("Program log: hello").unwrap(), None);
    }

    #[test]
    fn decodes_program_log_host_event_line() {
        let encoded = hex::encode(to_vec(&sample_event()).unwrap());
        let line = format!("{SOLANA_PROGRAM_LOG_PREFIX}{HOST_EVENT_PREFIX}{encoded}");

        assert_eq!(decode_host_event_line(&line).unwrap(), Some(sample_event()));
    }

    #[test]
    fn preserves_log_order() {
        let encoded = hex::encode(to_vec(&sample_event()).unwrap());
        let logs = vec![
            "Program log: hello".to_owned(),
            format!("{HOST_EVENT_PREFIX}{encoded}"),
        ];

        let decoded = decode_host_event_logs(logs.iter().map(String::as_str)).unwrap();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].log_index, 1);
        assert_eq!(decoded[0].event, sample_event());
    }
}
