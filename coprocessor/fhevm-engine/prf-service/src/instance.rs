// TODO: read the instances from the PRF module.
/// The bits of one PRF block of a known instance. `None` for an instance this coprocessor cannot serve.
pub fn block_bits(prf: u16) -> Option<u32> {
    match prf {
        0 => Some(2),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serves_only_known_instances() {
        assert_eq!(block_bits(0), Some(2));
        assert_eq!(block_bits(1), None);
    }
}
