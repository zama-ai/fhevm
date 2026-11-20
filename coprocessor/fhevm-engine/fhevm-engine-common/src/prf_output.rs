//! The S3 keys of the PRF outputs. The pad-worker writes the objects, and the consensus-detector
//! reads the objects of the other coprocessors.

use alloy::primitives::B256;

/// A form of a PRF output. Each form has its own S3 object.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PrfOutputForm {
    /// Before noise squashing: `Unmask` uses it.
    Ct64,
    /// After noise squashing: the response body.
    Ct128,
}

impl PrfOutputForm {
    pub fn name(self) -> &'static str {
        match self {
            PrfOutputForm::Ct64 => "ct64",
            PrfOutputForm::Ct128 => "ct128",
        }
    }
}

/// The S3 key of a form of a PRF output, `pads/{form}/{prf}/{label}/{padLength}`. The form comes
/// first, as in `ct128/` and `ct64/`, so a lifecycle rule can select it by prefix. The label is
/// lowercase hex, without `0x`.
pub fn prf_output_s3_key(prf_id: u16, label: B256, pad_length: u16, form: PrfOutputForm) -> String {
    format!(
        "pads/{}/{prf_id}/{}/{pad_length}",
        form.name(),
        hex::encode(label)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s3_key_of_each_form() {
        let label = B256::repeat_byte(0xab);
        let hex_label = "ab".repeat(32);
        assert_eq!(
            prf_output_s3_key(7, label, 64, PrfOutputForm::Ct64),
            format!("pads/ct64/7/{hex_label}/64")
        );
        assert_eq!(
            prf_output_s3_key(7, label, 64, PrfOutputForm::Ct128),
            format!("pads/ct128/7/{hex_label}/64")
        );
    }
}
