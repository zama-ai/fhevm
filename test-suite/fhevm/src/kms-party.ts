/**
 * Single source of truth for per-party KMS naming, port, and quorum conventions.
 *
 * The one rule everything below encodes: party 1 keeps the bare single-node names
 * (`kms-core`, `kms-connector`, ...) so the centralized templates, existing container
 * references, and discovery keep working unchanged; parties 2..N get a party suffix.
 * Derive every party-scoped name from here — never spell the convention inline.
 */

/** 1-indexed party ids for an N-party cluster: [1, 2, ..., N]. */
export const kmsPartyIds = (parties: number) => Array.from({ length: parties }, (_, i) => i + 1);

/**
 * On-chain decryption/keygen consensus threshold for an n = 3t+1 cluster.
 * Reconstruction needs 2t+1 matching responses (KMS core-client `num_reconstruct`;
 * see zama-ai/kms core-client/src/lib.rs + ci/kube-testing kms values). For
 * n=4,t=1 this is 3.
 */
export const reconstructionThreshold = (threshold: number) => 2 * threshold + 1;

/** gRPC service port for party i: 50100, 50200, ... */
export const kmsServicePort = (party: number) => 50000 + party * 100;

/** MPC core-to-core port for party i: 50001, 50002, ... */
export const kmsMpcPort = (party: number) => 50000 + party;

/** Core service/container name: `kms-core`, then `kms-core-{i}`. Cores reach each
 * other (and the connectors reach them) by these names over the docker network. */
export const kmsCoreName = (party: number) => (party === 1 ? "kms-core" : `kms-core-${party}`);

/** Connector container-name prefix: `kms-connector`, then `kms-connector-{i}`.
 * Services append `-db-migration`, `-gw-listener`, `-kms-worker`, `-tx-sender`. */
export const kmsConnectorPrefix = (party: number) =>
  party === 1 ? "kms-connector" : `kms-connector-${party}`;

/** Connector env-file name (for `envPath`): `kms-connector`, then `kms-connector.{i}` —
 * dot separator, following the harness's per-instance env-file convention. */
export const kmsConnectorEnvName = (party: number) =>
  party === 1 ? "kms-connector" : `kms-connector.${party}`;

/** Connector postgres database name: `kms-connector`, then `kms-connector-{i}`. */
export const kmsConnectorDbName = (party: number) =>
  party === 1 ? "kms-connector" : `kms-connector-${party}`;

/** Party i's S3 vault prefixes (threshold only; the centralized core uses `PUB`/`PRIV`).
 * kms-gen-keys writes each party's signing key under these, the cores mount them as
 * their vaults, and signer discovery reads each party's VerfAddress from its public one. */
export const kmsPublicPrefix = (party: number) => `PUB-p${party}`;
export const kmsPrivatePrefix = (party: number) => `PRIV-p${party}`;
export const kmsBackupPrefix = (party: number) => `BACKUP-p${party}`;
