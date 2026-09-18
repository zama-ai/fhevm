# Specimen program keypairs (well-known dev-only test keys)

Throwaway, publicly-known program keypairs for the two e2e specimen programs, `encrypted_counter`
and `dep_chain`: their public keys are the program ids (pinned in each `declare_id!`), and the
private keys are only the upgrade authority on a fresh local `solana-test-validator`. They hold no
funds and are never deployed to a public cluster. The side-stack setup
(`test-suite/fhevm/src/solana/validator.ts` `seedProgramKeypairs`) copies them into `target/deploy/`
so `solana program deploy` produces the specimens at the ids their generated clients expect.

The four deployed programs (`zama_host`, `confidential_token`, `demo_vault`, `confidential_batcher`)
have no keypair here. They have one id on every cluster (`solana/environments/preview-env.json`),
their private keys live in the preview environment's secret store, and the test validator loads
their build at genesis with the deployer wallet as upgrade authority (`genesisDeployedPrograms`).

To rotate a specimen: `solana-keygen new -o <name>-keypair.json`, update its `declare_id!`, rebuild,
and run `solana/scripts/sync-zama-host-idl.sh` then the SDK's `npm run codegen:solana`.
