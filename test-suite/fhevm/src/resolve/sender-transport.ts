import { localServicesForInstance, coprocessorBuildServices } from "../stack-spec/stack-spec";
import { GitHubApiError } from "../errors";
import { REPO_ROOT } from "../layout";
import type { LocalOverride, ResolvedScenario, VersionBundle } from "../types";
import { run } from "../utils/process";
import { fhevmFileAt } from "./github";

export type SenderGatewayTransport = "http" | "ws";
const SENDER_SOURCE = "coprocessor/fhevm-engine/transaction-sender/src/bin/transaction_sender.rs";

/** The selected image's source is authoritative, including release backports. */
export const senderTransportFromSource = (source: string): SenderGatewayTransport => {
  const ws = /\.connect_ws\s*\(/.test(source);
  const http = /\.connect_reqwest\s*\(/.test(source);
  if (ws === http) {
    throw new GitHubApiError(
      "Cannot determine sender Gateway transport: expected exactly one of connect_ws or connect_reqwest",
    );
  }
  return ws ? "ws" : "http";
};

export const senderSourceRevision = (tag: string): string => tag.replace(/^feature-solana-([0-9a-f]{40})$/, "$1");

const readSenderSource = async (tag: string): Promise<string> => {
  if (!/^[A-Za-z0-9._-]+$/.test(tag)) {
    throw new GitHubApiError(`Invalid sender image tag: ${tag}`);
  }
  const revision = senderSourceRevision(tag);
  const local = await run(["git", "show", `${revision}:${SENDER_SOURCE}`], { cwd: REPO_ROOT, allowFailure: true });
  if (local.code === 0) return local.stdout;
  return fhevmFileAt(SENDER_SOURCE, revision);
};

export const selectedSenderTags = (
  bundle: VersionBundle,
  scenario?: ResolvedScenario,
  overrides: LocalOverride[] = [],
): string[] => {
  if (!scenario) return [bundle.env.COPROCESSOR_TX_SENDER_VERSION!];
  const instances =
    scenario.kind === "blue-green"
      ? [
          { ...scenario.bcs, index: 0 },
          { ...scenario.gcs, index: 0 },
        ]
      : scenario.instances;
  const inheritedLocal = coprocessorBuildServices({ overrides });
  return [
    ...new Set(
      instances.flatMap((instance) => {
        if (instance.source.mode === "registry") return [instance.source.tag];
        const local = instance.source.mode === "local" ? localServicesForInstance(instance) : inheritedLocal;
        return local.has("coprocessor-transaction-sender") ? [] : [bundle.env.COPROCESSOR_TX_SENDER_VERSION!];
      }),
    ),
  ];
};

/** Resolve after image fallbacks and overrides; persist facts by the image tag, never compatTag. */
export const resolveSenderTransports = async (
  bundle: VersionBundle,
  tags?: string[],
  options: { offline?: boolean; readSource?: (tag: string) => Promise<string> } = {},
): Promise<VersionBundle> => {
  const transports = { ...bundle.senderGatewayTransports };
  const selected = new Set(
    tags ?? (bundle.env.COPROCESSOR_TX_SENDER_VERSION ? [bundle.env.COPROCESSOR_TX_SENDER_VERSION] : []),
  );
  await Promise.all(
    [...selected].map(async (tag) => {
      if (transports[tag] === "http" || transports[tag] === "ws") return;
      if (options.offline) {
        throw new GitHubApiError(
          `Lock has no Gateway transport for sender ${tag}. Run fhevm-cli resolve --lock-file <file> --reset (with --scenario for pinned fleets) to update it online.`,
        );
      }
      transports[tag] = senderTransportFromSource(await (options.readSource ?? readSenderSource)(tag));
    }),
  );
  return { ...bundle, senderGatewayTransports: transports };
};

export const senderTransportForTag = (bundle: VersionBundle, tag: string): SenderGatewayTransport => {
  const transport = bundle.senderGatewayTransports?.[tag];
  if (transport !== "http" && transport !== "ws") {
    throw new GitHubApiError(
      `No resolved Gateway transport for sender ${tag}. Resolve the target and pinned fleet before rendering.`,
    );
  }
  return transport;
};
