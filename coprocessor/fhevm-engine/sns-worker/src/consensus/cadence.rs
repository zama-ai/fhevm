const DEFAULT_MANIFEST_PUBLICATION_CADENCE: i64 = 30;

#[derive(Clone, Copy)]
struct ChainManifestCadence {
    host_chain_id: i64,
    blocks_per_manifest: i64,
}

// Fixed block ranges target roughly one manifest per minute. Unknown chains use
// the conservative one-second-block assumption and publish every 30 blocks.
const MANIFEST_PUBLICATION_CADENCE_BY_CHAIN: &[ChainManifestCadence] = &[
    ChainManifestCadence {
        host_chain_id: 1,
        blocks_per_manifest: 5,
    },
    ChainManifestCadence {
        host_chain_id: 11_155_111,
        blocks_per_manifest: 5,
    },
    ChainManifestCadence {
        host_chain_id: 137,
        blocks_per_manifest: 30,
    },
    ChainManifestCadence {
        host_chain_id: 80_002,
        blocks_per_manifest: 30,
    },
    ChainManifestCadence {
        host_chain_id: 8_453,
        blocks_per_manifest: 30,
    },
    ChainManifestCadence {
        host_chain_id: 84_532,
        blocks_per_manifest: 30,
    },
];

pub(crate) fn manifest_publication_cadence(host_chain_id: i64) -> i64 {
    MANIFEST_PUBLICATION_CADENCE_BY_CHAIN
        .iter()
        .find_map(|configured| {
            (configured.host_chain_id == host_chain_id).then_some(configured.blocks_per_manifest)
        })
        .unwrap_or(DEFAULT_MANIFEST_PUBLICATION_CADENCE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_fixed_chain_cadence_with_thirty_block_default() {
        assert_eq!(manifest_publication_cadence(1), 5);
        assert_eq!(manifest_publication_cadence(11_155_111), 5);
        assert_eq!(manifest_publication_cadence(137), 30);
        assert_eq!(manifest_publication_cadence(8_453), 30);
        assert_eq!(manifest_publication_cadence(9_999_999), 30);
    }
}
