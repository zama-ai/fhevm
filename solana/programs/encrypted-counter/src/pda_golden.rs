use crate::{pda_vectors::PdaVectors, state::*};

#[test]
fn pda_golden() {
    let vectors = PdaVectors::load();
    let counter = counter_address(vectors.key("owner"));
    let pdas = [
        ("counter", counter),
        ("counterAuthority", counter_authority_address(counter.0)),
    ];
    vectors.check("encryptedCounter", crate::ID, &pdas);
}
