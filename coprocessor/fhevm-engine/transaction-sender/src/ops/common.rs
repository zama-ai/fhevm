use anyhow::{anyhow, Result};
use std::convert::TryInto;

pub(crate) fn try_into_array<const SIZE: usize>(vec: Vec<u8>) -> Result<[u8; SIZE]> {
    if vec.len() != SIZE {
        return Err(anyhow!(
            "invalid len, expected {} but got {}",
            SIZE,
            vec.len()
        ));
    }

    vec.try_into()
        .map_err(|_| anyhow!("Failed to convert Vec to array"))
}
