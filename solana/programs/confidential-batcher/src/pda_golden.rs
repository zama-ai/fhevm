use crate::{pda_vectors::PdaVectors, state::*};

#[test]
fn pda_golden() {
    let vectors = PdaVectors::load();
    let batch = batch_address(
        vectors.key("batcher"),
        vectors.fixture["inputs"]["batchIndex"]
            .as_str()
            .unwrap()
            .parse::<u64>()
            .unwrap(),
    );
    let pdas = [
        ("batch", batch),
        ("batchAuthority", batch_authority_address(batch.0)),
        (
            "joinRecord",
            join_record_address(batch.0, vectors.key("user")),
        ),
        (
            "batchJoinUnderlying",
            batch_join_underlying_address(batch.0),
        ),
        (
            "batchPayoutUnderlying",
            batch_payout_underlying_address(batch.0),
        ),
    ];
    vectors.check("confidentialBatcher", crate::ID, &pdas);
}
