/**
 * Generates the threshold-mode KMS cluster: a `core-threshold` compose override
 * (gen-keys + N cores + kms-init) wired to the checked-in
 * `templates/config/kms-core-threshold.toml`.
 *
 * Used only when a scenario's `kms` block is `mode: threshold`. The centralized
 * path is untouched (single `kms-core` from `core-docker-compose.yml`).
 *
 * Config strategy (mirrors how the centralized core is configured — a checked-in
 * template plus `KMS_CORE__*` env overrides — instead of rendering a TOML blob in TS):
 *   - the static tuning + structure lives in the checked-in template;
 *   - the only generated part is the `[[threshold.peers]]` roster, injected at the
 *     template's marker because it depends on the party count (it is identical for
 *     every party, so the rendered file is shared by the whole cluster);
 *   - per-party values (my_id, listen ports, vault prefixes) are supplied as
 *     `KMS_CORE__*` env overrides in `thresholdCoreEnv` — the same env layering the
 *     centralized core relies on. The template's per-party placeholders are invalid
 *     on purpose (my_id = 0), so a dropped override fails loudly rather than silently
 *     misconfiguring the cluster.
 *
 * Design notes (kept deliberately close to how zama-ai/kms's own CI stands up a
 * threshold-mode cluster — see ci/kube-testing + core/service/config/compose_1.toml):
 *   - Core-to-core MPC runs WITHOUT mTLS (the `[threshold.tls]` block is omitted;
 *     the field is optional). This matches the kind-CI test posture and removes all
 *     cert generation / distribution / subject-matching. Peers reach each other by
 *     docker service name.
 */
import path from "node:path";

import type { ComposeDoc } from "./compose";
import {
  kmsBackupPrefix,
  kmsCoreName,
  kmsMpcPort,
  kmsPartyIds,
  kmsPrivatePrefix,
  kmsPublicPrefix,
  kmsServicePort,
} from "../kms-party";
import type { ResolvedKmsTopology } from "../types";
import { GENERATED_CONFIG_DIR } from "../layout";

/** Knobs the generator needs that come from the surrounding stack (S3, image tag). */
export type KmsRenderOptions = {
  coreImage: string; // e.g. ghcr.io/zama-ai/kms/core-service:${CORE_VERSION}
  s3Endpoint: string; // e.g. http://minio:9000
  s3Bucket: string; // e.g. kms-public
  s3Region: string; // e.g. eu-west-1
  s3AccessKey: string; // minio access key (shared with the rest of the stack)
  s3SecretKey: string; // minio secret key
};

/** Render options from the resolved core image version + fhevm minio defaults
 * (the static test credentials from templates/env/.env.minio). */
export const kmsRenderOptionsFor = (coreVersion: string): KmsRenderOptions => ({
  coreImage: `ghcr.io/zama-ai/kms/core-service:${coreVersion}`,
  s3Endpoint: "http://minio:9000",
  s3Bucket: "kms-public",
  s3Region: "eu-west-1",
  s3AccessKey: "fhevm-access-key",
  s3SecretKey: "fhevm-access-secret-key",
});

/** The committee threshold config filename (mounted into the `committeeSize` committee cores). */
export const KMS_THRESHOLD_CONFIG_NAME = "kms-core-threshold.toml";
/** The spare threshold config filename: same template with NO peer roster (peers=None), so spare
 *  cores skip the `3t+1` startup validation and boot idle, joining a committee dynamically when a
 *  context names them. Mounted into cores with party id > committeeSize. */
export const KMS_THRESHOLD_SPARE_CONFIG_NAME = "kms-core-threshold-spare.toml";
/** Marker in the checked-in template where the per-cluster peer roster is injected. */
export const THRESHOLD_PEERS_MARKER = "# __THRESHOLD_PEERS__";

/** The `[[threshold.peers]]` roster for the committee (the first `committeeSize` parties). The MPC
 *  group is the committee, so the roster is committee-sized (3t+1), not cluster-sized.
 *  `mpc_identity` must equal the on-chain KMS_NODE_MPC_IDENTITY (kmsCoreName); otherwise the core
 *  derives it as "hostname:port" here but as the on-chain value in a dynamically-added context, so a
 *  reshare across the two contexts can't match a node's identity across them. */
export const renderThresholdPeers = (topology: ResolvedKmsTopology): string =>
  kmsPartyIds(topology.committeeSize)
    .map(
      (peer) => `[[threshold.peers]]
party_id = ${peer}
address = "${kmsCoreName(peer)}"
port = ${kmsMpcPort(peer)}
mpc_identity = "${kmsCoreName(peer)}"`,
    )
    .join("\n\n");

/** Injects the committee peer roster into the checked-in template; the rest of the config is static. */
export const renderThresholdCoreConfig = (templateText: string, topology: ResolvedKmsTopology): string => {
  if (!templateText.includes(THRESHOLD_PEERS_MARKER)) {
    throw new Error(`threshold core config template is missing the ${THRESHOLD_PEERS_MARKER} marker`);
  }
  return templateText.replace(THRESHOLD_PEERS_MARKER, renderThresholdPeers(topology));
};

/** Spare-core config: drops the peer roster entirely (peers=None) so the core skips `3t+1`
 *  validation and boots idle. */
export const renderThresholdSpareConfig = (templateText: string): string => {
  if (!templateText.includes(THRESHOLD_PEERS_MARKER)) {
    throw new Error(`threshold core config template is missing the ${THRESHOLD_PEERS_MARKER} marker`);
  }
  return templateText.replace(THRESHOLD_PEERS_MARKER, "");
};

/**
 * Per-party `KMS_CORE__*` overrides for the shared template's placeholders. The `__`
 * separator nests into the TOML tables (e.g. KMS_CORE__THRESHOLD__MY_ID -> [threshold].my_id),
 * the same layering the centralized core uses for its vault config.
 */
export const thresholdCoreEnv = (
  partyId: number,
  topology: ResolvedKmsTopology,
  opts: KmsRenderOptions,
): Record<string, string> => ({
  KMS_CORE__SERVICE__LISTEN_PORT: String(kmsServicePort(partyId)),
  KMS_CORE__THRESHOLD__LISTEN_PORT: String(kmsMpcPort(partyId)),
  KMS_CORE__THRESHOLD__MY_ID: String(partyId),
  KMS_CORE__THRESHOLD__THRESHOLD: String(topology.threshold),
  KMS_CORE__AWS__REGION: opts.s3Region,
  KMS_CORE__AWS__S3_ENDPOINT: opts.s3Endpoint,
  KMS_CORE__PUBLIC_VAULT__STORAGE__S3__BUCKET: opts.s3Bucket,
  KMS_CORE__PUBLIC_VAULT__STORAGE__S3__PREFIX: kmsPublicPrefix(partyId),
  KMS_CORE__PRIVATE_VAULT__STORAGE__S3__BUCKET: opts.s3Bucket,
  KMS_CORE__PRIVATE_VAULT__STORAGE__S3__PREFIX: kmsPrivatePrefix(partyId),
  KMS_CORE__BACKUP_VAULT__STORAGE__FILE__PREFIX: kmsBackupPrefix(partyId),
  KMS_CORE__TELEMETRY__TRACING_SERVICE_NAME: `kms-threshold-${partyId}`,
  // The core's AWS SDK reads the minio creds straight from the environment — no
  // need to shell out and `cat` them from the shared secrets volume at startup.
  AWS_ACCESS_KEY_ID: opts.s3AccessKey,
  AWS_SECRET_ACCESS_KEY: opts.s3SecretKey,
});

/** The `kms-gen-keys` TOML config for one threshold party on the newer (config-file) core CLI. The
 * `[threshold]` section (vs an absent one, which would mean centralized) selects threshold
 * signing-key + CA-cert generation for `my_id`; the empty `[keygen]` section is required for the
 * file to parse. `tls_subject` becomes the cert CN, which the KMS context wiring surfaces as each
 * node's caCert / mpcIdentity. Mirrors zama-ai/kms's own reference threshold compose. */
const genKeysThresholdConfig = (party: number, opts: KmsRenderOptions) =>
  [
    "mock_enclave = true",
    "",
    "[keygen]",
    "",
    "[threshold]",
    `my_id = ${party}`,
    `tls_subject = "${kmsCoreName(party)}"`,
    "tls_wildcard = true",
    "",
    "[aws]",
    `region = "${opts.s3Region}"`,
    `s3_endpoint = "${opts.s3Endpoint}"`,
    "",
    "[public_vault.storage.s3]",
    `bucket = "${opts.s3Bucket}"`,
    `prefix = "${kmsPublicPrefix(party)}"`,
    "",
    "[private_vault.storage.s3]",
    `bucket = "${opts.s3Bucket}"`,
    `prefix = "${kmsPrivatePrefix(party)}"`,
  ].join("\n");

/** Shell for the signing-key setup container: one signing key + self-signed CA cert per party into
 * S3 (unrolled per party in TS, prefixes from kms-party.ts, no `$$` compose escaping). The FHE key
 * shares and CRS are NOT pre-generated here; they come from the on-chain DKG (keygen/crsgen). CN =
 * the core name, which the KMS context wiring surfaces as each node's caCert / mpcIdentity.
 *
 * The kms-gen-keys CLI changed across core images: newer cores take a single TOML `--config-file`
 * (storage/AWS/party settings live in the file; threshold vs centralized is the config shape),
 * while older ones took `--public-storage`/`--aws-region`/… flags plus a `threshold` subcommand
 * (and an even older `--cmd signing-keys` selector / `--num-parties`). Probe `--help` once and emit
 * the matching form so a pinned old or new CORE_VERSION both boot. AWS creds come from the env. */
const genKeysCommand = (topology: ResolvedKmsTopology, opts: KmsRenderOptions) => {
  const parties = kmsPartyIds(topology.parties);
  const viaConfigFile = (party: number) =>
    [
      `cat > /tmp/kms-gen-keys-${party}.toml <<'TOML'`,
      genKeysThresholdConfig(party, opts),
      "TOML",
      `kms-gen-keys --config-file /tmp/kms-gen-keys-${party}.toml`,
    ].join("\n");
  const viaFlags = (party: number) => `kms-gen-keys --aws-region ${opts.s3Region} \\
  --public-storage s3 --public-s3-bucket ${opts.s3Bucket} --public-s3-prefix ${kmsPublicPrefix(party)} \\
  --aws-s3-endpoint ${opts.s3Endpoint} \\
  --private-storage s3 --private-s3-bucket ${opts.s3Bucket} --private-s3-prefix ${kmsPrivatePrefix(party)} \\
  $CMD \\
  threshold --signing-key-party-id ${party} --tls-subject ${kmsCoreName(party)} --tls-wildcard $NP`;
  return [
    "set -e",
    `echo "=== generating signing keys for ${topology.parties} parties ==="`,
    // Newer cores take a TOML --config-file; older ones take storage/AWS flags. Probe once, branch.
    "if kms-gen-keys --help 2>&1 | grep -q -- '--config-file'; then",
    ...parties.map(viaConfigFile),
    "else",
    `  if kms-gen-keys --help 2>&1 | grep -q -- '--cmd'; then CMD="--cmd signing-keys"; else CMD=""; fi`,
    `  if kms-gen-keys threshold --help 2>&1 | grep -q -- '--num-parties'; then NP="--num-parties ${topology.parties}"; else NP=""; fi`,
    ...parties.map(viaFlags),
    "fi",
  ].join("\n");
};

/**
 * Builds the threshold-mode cluster compose doc: 1 gen-keys container + N cores +
 * kms-init. This is the generated override for the `core-threshold` component
 * (a dedicated component, so it never merges with the centralized `core`
 * template — no env/healthcheck conflicts to work around).
 */
// The KMS core image is published amd64-only at every tag; pin the platform so the generated cores
// run (emulated) on arm64 hosts, matching the hardcoded pin in core-docker-compose.yml.
const CORE_PLATFORM = "linux/amd64";

export const buildKmsThresholdOverride = (
  topology: ResolvedKmsTopology,
  opts: KmsRenderOptions,
  coreVersionByNodeId: Readonly<Record<string, string>> = {},
): ComposeDoc => {
  if (topology.mode !== "threshold") {
    throw new Error("buildKmsThresholdOverride called for a non-threshold topology");
  }
  const services: Record<string, Record<string, unknown>> = {};

  services["kms-core-gen-keys"] = {
    container_name: "kms-core-gen-keys",
    image: opts.coreImage,
    platform: CORE_PLATFORM,
    entrypoint: ["/bin/sh", "-c", genKeysCommand(topology, opts)],
    environment: { AWS_ACCESS_KEY_ID: opts.s3AccessKey, AWS_SECRET_ACCESS_KEY: opts.s3SecretKey },
  };

  const configMountFor = (configName: string) =>
    `${path.join(GENERATED_CONFIG_DIR, configName)}:/app/kms/core/service/config/${configName}`;

  for (const partyId of kmsPartyIds(topology.parties)) {
    const name = kmsCoreName(partyId);
    // Cores beyond the committee boot as spares with the peers=None config (idle until a context names them).
    const configName =
      partyId > topology.committeeSize ? KMS_THRESHOLD_SPARE_CONFIG_NAME : KMS_THRESHOLD_CONFIG_NAME;
    services[name] = {
      container_name: name,
      image: coreVersionByNodeId[partyId]
        ? kmsRenderOptionsFor(coreVersionByNodeId[partyId]).coreImage
        : opts.coreImage,
      platform: CORE_PLATFORM,
      // No shell wrapper: per-party config comes from KMS_CORE__* env and AWS creds
      // come from the environment, so the core binary runs directly.
      entrypoint: ["kms-server", "--config-file", `config/${configName}`],
      // Per-party identity/ports/prefixes (override the template placeholders) + AWS creds.
      environment: thresholdCoreEnv(partyId, topology, opts),
      volumes: [configMountFor(configName)],
      // No host port mapping: connectors and kms-init dial the cores over the docker network.
      healthcheck: {
        // The core image ships no grpc_health_probe; probe the metrics port.
        test: ["CMD-SHELL", "wget -q -O /dev/null http://localhost:9646/metrics || exit 1"],
        interval: "3s",
        timeout: "3s",
        retries: 30,
        start_period: "5s",
      },
      depends_on: {
        "kms-core-gen-keys": { condition: "service_completed_successfully" },
      },
    };
  }

  // Once all cores are healthy, kms-init establishes the MPC context/epoch across the COMMITTEE
  // (the first committeeSize cores) — required before the cluster can serve requests. Spares are not
  // part of the initial context; they join later via a context switch.
  const initEndpoints = kmsPartyIds(topology.committeeSize)
    .map((partyId) => `http://${kmsCoreName(partyId)}:${kmsServicePort(partyId)}`)
    .join(" ");
  services["kms-core-init"] = {
    container_name: "kms-core-init",
    image: opts.coreImage,
    platform: CORE_PLATFORM,
    entrypoint: ["/bin/sh", "-c", `kms-init -a ${initEndpoints}`],
    depends_on: Object.fromEntries(
      kmsPartyIds(topology.parties).map((partyId) => [
        kmsCoreName(partyId),
        { condition: "service_healthy" },
      ]),
    ),
  };

  return { services } as ComposeDoc;
};
