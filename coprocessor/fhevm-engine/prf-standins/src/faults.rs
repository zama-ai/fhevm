//! Faults the stand-in pad-worker injects, selected by the first byte of the label, so one
//! deployment serves every scenario: `PAD_WORKER_FAULTS="d1:delay=10s,b1:wrong_digest"`.

use alloy::primitives::B256;
use anyhow::{anyhow, bail, Context};
use std::{collections::HashMap, fmt, str::FromStr, time::Duration};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fault {
    /// Evaluates after this delay.
    Delay(Duration),
    /// Writes a pad that only this party computes, with a valid signature.
    WrongDigest,
    /// Writes the right pad with a zero signature.
    Unsigned,
    /// Writes the right pad, signed by a key that is not a registered coprocessor signer.
    ForeignSigner,
    /// Exits the process after claiming the row, on the first attempt only.
    CrashOnce,
    /// Marks the row failed (`is_error`), as for a permanent evaluation failure.
    Fail,
}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Delay(d) => write!(f, "delay={}", humantime::format_duration(*d)),
            Self::WrongDigest => f.write_str("wrong_digest"),
            Self::Unsigned => f.write_str("unsigned"),
            Self::ForeignSigner => f.write_str("foreign_signer"),
            Self::CrashOnce => f.write_str("crash_once"),
            Self::Fail => f.write_str("fail"),
        }
    }
}

impl FromStr for Fault {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        if let Some(delay) = s.strip_prefix("delay=") {
            return Ok(Self::Delay(humantime::parse_duration(delay)?));
        }
        Ok(match s {
            "wrong_digest" => Self::WrongDigest,
            "unsigned" => Self::Unsigned,
            "foreign_signer" => Self::ForeignSigner,
            "crash_once" => Self::CrashOnce,
            "fail" => Self::Fail,
            other => bail!("unknown fault {other:?}"),
        })
    }
}

/// The fault of each label first byte.
#[derive(Clone, Debug, Default)]
pub struct Faults(HashMap<u8, Fault>);

impl Faults {
    pub fn for_label(&self, label: &B256) -> Option<&Fault> {
        self.0.get(&label[0])
    }
}

impl FromStr for Faults {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut faults = HashMap::new();
        for entry in s.split(',').map(str::trim).filter(|e| !e.is_empty()) {
            let (byte, fault) = entry
                .split_once(':')
                .ok_or_else(|| anyhow!("{entry:?}: expected <label byte hex>:<fault>"))?;
            let byte = u8::from_str_radix(byte, 16).with_context(|| format!("{entry:?}"))?;
            faults.insert(byte, fault.parse().with_context(|| format!("{entry:?}"))?);
        }
        Ok(Self(faults))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_faults_by_label_byte() {
        let faults: Faults = "d1:delay=10s, b1:wrong_digest,c1:crash_once"
            .parse()
            .unwrap();
        let label = |byte: u8| {
            let mut label = B256::ZERO;
            label.0[0] = byte;
            label
        };
        assert_eq!(
            faults.for_label(&label(0xd1)),
            Some(&Fault::Delay(Duration::from_secs(10)))
        );
        assert_eq!(faults.for_label(&label(0xb1)), Some(&Fault::WrongDigest));
        assert_eq!(faults.for_label(&label(0x00)), None);
        assert!("zz:fail".parse::<Faults>().is_err());
        assert!("a1:explode".parse::<Faults>().is_err());
        assert!("".parse::<Faults>().unwrap().0.is_empty());
    }
}
