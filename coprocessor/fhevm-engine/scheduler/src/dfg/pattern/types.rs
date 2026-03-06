use std::fmt;

/// Decoded description of a pattern encoding.
pub struct PatternDescription {
    pub nodes: Vec<PatternNode>,
}

pub struct PatternNode {
    /// Raw opcode value (SupportedFheOperations repr).
    pub opcode: i32,
    /// Human-readable opcode name via `FheOperation::as_str_name()`.
    pub opcode_name: &'static str,
    pub is_allowed: bool,
    pub inputs: Vec<PatternInput>,
}

pub enum PatternInput {
    /// Internal reference to another node at the given topo position.
    Internal(u8),
    /// External input (DB handle, other group, scalar).
    External,
}

impl fmt::Display for PatternDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, node) in self.nodes.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", node.opcode_name)?;
            if node.is_allowed {
                write!(f, "[a]")?;
            }
        }
        Ok(())
    }
}
