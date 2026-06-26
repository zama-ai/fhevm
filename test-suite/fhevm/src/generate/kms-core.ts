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

/** The single cluster-shared threshold config filename (mounted into every core). */
export const KMS_THRESHOLD_CONFIG_NAME = "kms-core-threshold.toml";
/** Marker in the checked-in template where the per-cluster peer roster is injected. */
export const THRESHOLD_PEERS_MARKER = "# __THRESHOLD_PEERS__";

/** The `[[threshold.peers]]` roster for the whole cluster (identical for every party). */
export const renderThresholdPeers = (topology: ResolvedKmsTopology): string =>
  kmsPartyIds(topology.parties)
    .map(
      (peer) => `[[threshold.peers]]
party_id = ${peer}
address = "${kmsCoreName(peer)}"
port = ${kmsMpcPort(peer)}`,
    )
    .join("\n\n");

/** Injects the peer roster into the checked-in template; the rest of the config is static. */
export const renderThresholdCoreConfig = (templateText: string, topology: ResolvedKmsTopology): string => {
  if (!templateText.includes(THRESHOLD_PEERS_MARKER)) {
    throw new Error(`threshold core config template is missing the ${THRESHOLD_PEERS_MARKER} marker`);
  }
  return templateText.replace(THRESHOLD_PEERS_MARKER, renderThresholdPeers(topology));
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

/** Shell for the signing-key setup container, one invocation per party (unrolled in TS rather
 * than a shell loop, so prefixes come from kms-party.ts and no `$$` compose-interpolation
 * escaping is needed). Generates ONLY each party's signing key + self-signed CA cert into S3,
 * mirroring the KMS reference threshold compose. The FHE key shares and CRS are NOT pre-generated
 * here; they come from the real on-chain DKG (keygen/crsgen). `--num-parties` must match the
 * cluster size (the CLI rejects a signing-key-party-id greater than num-parties; it also defaults
 * to 4). The `--tls-*` flags shape the generated cert material — CN = the core name — which the
 * KMS context wiring surfaces as each node's caCert / mpcIdentity.
 *
 * The kms-gen-keys CLI differs across core images: older ones scope to signing keys with a
 * `--cmd signing-keys` selector (their `--cmd` default is `all`, which would also generate FHE
 * keys centrally), while newer ones dropped it and have the `threshold` subcommand emit the
 * signing keys + CA certs directly. Probe `--help` once and inject the selector only when the
 * image still understands it, so a pinned old or new CORE_VERSION both boot. AWS creds come from
 * the container env. */
const genKeysCommand = (topology: ResolvedKmsTopology, opts: KmsRenderOptions) =>
  [
    "set -e",
    `echo "=== generating signing keys for ${topology.parties} parties ==="`,
    // Old cores need `--cmd signing-keys`; new cores removed the flag. Detect which form this image speaks.
    `if kms-gen-keys --help 2>&1 | grep -q -- '--cmd'; then CMD="--cmd signing-keys"; else CMD=""; fi`,
    ...kmsPartyIds(topology.parties).map(
      (party) => `kms-gen-keys --aws-region ${opts.s3Region} \\
  --public-storage s3 --public-s3-bucket ${opts.s3Bucket} --public-s3-prefix ${kmsPublicPrefix(party)} \\
  --aws-s3-endpoint ${opts.s3Endpoint} \\
  --private-storage s3 --private-s3-bucket ${opts.s3Bucket} --private-s3-prefix ${kmsPrivatePrefix(party)} \\
  $CMD \\
  threshold --signing-key-party-id ${party} --tls-subject ${kmsCoreName(party)} --tls-wildcard --num-parties ${topology.parties}`,
    ),
  ].join("\n");

/**
 * Builds the threshold-mode cluster compose doc: 1 gen-keys container + N cores +
 * kms-init. This is the generated override for the `core-threshold` component
 * (a dedicated component, so it never merges with the centralized `core`
 * template — no env/healthcheck conflicts to work around).
 */
export const buildKmsThresholdOverride = (
  topology: ResolvedKmsTopology,
  opts: KmsRenderOptions,
): ComposeDoc => {
  if (topology.mode !== "threshold") {
    throw new Error("buildKmsThresholdOverride called for a non-threshold topology");
  }
  const services: Record<string, Record<string, unknown>> = {};

  services["kms-core-gen-keys"] = {
    container_name: "kms-core-gen-keys",
    image: opts.coreImage,
    entrypoint: ["/bin/sh", "-c", genKeysCommand(topology, opts)],
    environment: { AWS_ACCESS_KEY_ID: opts.s3AccessKey, AWS_SECRET_ACCESS_KEY: opts.s3SecretKey },
  };

  const sharedConfigMount = `${path.join(GENERATED_CONFIG_DIR, KMS_THRESHOLD_CONFIG_NAME)}:/app/kms/core/service/config/${KMS_THRESHOLD_CONFIG_NAME}`;

  for (const partyId of kmsPartyIds(topology.parties)) {
    const name = kmsCoreName(partyId);
    services[name] = {
      container_name: name,
      image: opts.coreImage,
      // No shell wrapper: per-party config comes from KMS_CORE__* env and AWS creds
      // come from the environment, so the core binary runs directly.
      entrypoint: ["kms-server", "--config-file", `config/${KMS_THRESHOLD_CONFIG_NAME}`],
      // Per-party identity/ports/prefixes (override the template placeholders) + AWS creds.
      environment: thresholdCoreEnv(partyId, topology, opts),
      volumes: [sharedConfigMount],
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

  // Once all cores are healthy, kms-init establishes the MPC context/epoch
  // across the parties (required before the cluster can serve requests).
  const initEndpoints = kmsPartyIds(topology.parties)
    .map((partyId) => `http://${kmsCoreName(partyId)}:${kmsServicePort(partyId)}`)
    .join(" ");
  services["kms-core-init"] = {
    container_name: "kms-core-init",
    image: opts.coreImage,
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
