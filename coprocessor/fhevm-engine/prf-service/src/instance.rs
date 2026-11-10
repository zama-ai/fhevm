// TODO: read the instances from the PRF module.
/// Whether this coprocessor serves the instance `prf`.
pub fn is_known(prf: u16) -> bool {
    prf == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serves_only_known_instances() {
        assert!(is_known(0));
        assert!(!is_known(1));
    }
}
