use crate::{pda_vectors::PdaVectors, state::*};

#[test]
fn pda_golden() {
    let vectors = PdaVectors::load();
    let chain = chain_address(vectors.key("owner"));
    let pdas = [
        ("chain", chain),
        ("chainAuthority", chain_authority_address(chain.0)),
    ];
    vectors.check("depChain", crate::ID, &pdas);
}
