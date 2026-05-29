//! IDL-generated ZamaHost Anchor CPI event decoders shared by the LiteSVM harness
//! and `host-listener`.

include!(concat!(env!("OUT_DIR"), "/zama_host_events.rs"));

impl FheBinaryOpCode {
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::Add => 0,
            Self::Sub => 1,
            Self::Ge  => 14
        }
    }
}

impl FheTernaryOpCode {
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::IfThenElse => 25
        }
    }
}
