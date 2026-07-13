import { Option, type Command } from "@commander-js/extra-typings";
import { readdir } from "node:fs/promises";

import {
  emitJson,
  envFromCommand,
  parseBoundedInt,
  parseBoundedIntOrAuto,
  parsePositiveInt,
  parseValueTypes,
  useJsonOutput,
  withEnvOptions,
  withFormatOption,
} from "../shared";

const MAX_THREADS = 128;
const MAX_LANES = 64;
const MAX_ENCRYPT_CONCURRENCY = 256;

const FLOWS = ["input-proof", "public-decrypt", "user-decrypt", "delegated-user-decrypt"] as const;

type PoolAddOptions = Readonly<{
  flow: (typeof FLOWS)[number];
  threads?: number;
  lanes?: number;
  encryptConcurrency?: number | "auto";
  delegationDays?: number;
}>;

export const validatePoolAddOptions = (options: PoolAddOptions): void => {
  if (options.flow === "input-proof") {
    if (
      options.lanes !== undefined ||
      options.encryptConcurrency !== undefined ||
      options.delegationDays !== undefined
    ) {
      throw new Error(
        "--lanes, --encrypt-concurrency, and --delegation-days are not valid for input-proof pools.",
      );
    }
    return;
  }
  if (options.threads !== undefined) {
    throw new Error("--threads is only valid for input-proof pools.");
  }
  if (options.flow !== "delegated-user-decrypt" && options.delegationDays !== undefined) {
    throw new Error("--delegation-days is only valid for delegated-user-decrypt pools.");
  }
};

export const describeAclExpiration = (
  expiration: string | undefined,
  nowSeconds: bigint,
): string => {
  if (expiration === undefined) return "missing";
  return BigInt(expiration) <= nowSeconds
    ? `expired at ${expiration}`
    : `healthy until ${expiration}`;
};

export const registerPoolCommands = (program: Command): void => {
  const pool = program.command("pool").description("Manage payload and handle pools");

  withEnvOptions(pool.command("add")
    .description("Add flow-appropriate items to a pool (proof payloads or on-chain handles)"))
    .addOption(new Option("--flow <flow>", "flow the pool serves").choices([...FLOWS]).makeOptionMandatory())
    .requiredOption("--count <n>", "pool items to add", parsePositiveInt)
    .option("--types <list>", "comma-separated FHE value types", parseValueTypes)
    .option("--threads <n>", "input-proof worker threads", parseBoundedInt("--threads", MAX_THREADS))
    .option("--lanes <n>", "handle-pool wallet lanes (HD accounts)", parseBoundedInt("--lanes", MAX_LANES))
    .option(
      "--encrypt-concurrency <n|auto>",
      "handle preparation concurrency per wallet lane",
      parseBoundedIntOrAuto("--encrypt-concurrency", MAX_ENCRYPT_CONCURRENCY),
    )
    .option("--delegation-days <n>", "delegated-user-decrypt ACL duration", parsePositiveInt)
    .action(async (options, command) => {
      validatePoolAddOptions(options);
      const env = await envFromCommand(command);
      const { logger } = await import("../../shared/logger");
      if (options.flow === "input-proof") {
        const { generateInputProofPool } = await import("../../pool/input-proof");
        await generateInputProofPool(env, {
          count: options.count,
          valueTypes: options.types as never,
          threads: options.threads,
          onProgress: (done, total) => {
            if (done % 25 === 0 || done === total) {
              logger.info(`generated ${done.toString()}/${total.toString()} payload(s)`);
            }
          },
        });
        return;
      }
      const { createHandlePool } = await import("../../pool/handles");
      await createHandlePool(env, {
        flow: options.flow,
        count: options.count,
        valueTypes: options.types as never,
        lanes: options.lanes,
        encryptConcurrency: options.encryptConcurrency,
        delegationDurationDays: options.delegationDays,
        onProgress: (done, total) =>
          logger.info(`created ${done.toString()}/${total.toString()} handle(s)`),
      });
    });

  withFormatOption(withEnvOptions(pool.command("inspect").description("Show flow-aware capacity, consumption, owners, and ACL health")))
    .action(async (options, command) => {
      const json = await useJsonOutput(options);
      const env = await envFromCommand(command);
      const [{ poolDir }, { HANDLE_POOLS }, { INPUT_PROOF_POOL }, { PoolStore }, { binomial }, { logger }] = await Promise.all([
        import("../../env"), import("../../pool/handles"), import("../../pool/input-proof"),
        import("../../pool/store"), import("../../pool/combinations"), import("../../shared/logger"),
      ]);
      const nowSeconds = BigInt(Math.floor(Date.now() / 1000));
      const statuses: unknown[] = [];
      for (const name of [INPUT_PROOF_POOL, ...Object.values(HANDLE_POOLS)]) {
        const store = await PoolStore.openIfExists(poolDir(env, name));
        if (!store) {
          if (json) statuses.push({ pool: name, status: "not-created" });
          else logger.info(`${name}: not created`);
          continue;
        }
        if (store.meta.flow === "input-proof") {
          const consumed = store.cursor("submit").position;
          const remaining = BigInt(store.meta.count) > consumed
            ? BigInt(store.meta.count) - consumed
            : 0n;
          if (json) {
            statuses.push({
              pool: name,
              flow: store.meta.flow,
              count: store.meta.count,
              consumed: consumed.toString(),
              remaining: remaining.toString(),
            });
          } else {
            logger.info(
              `${name}: ${store.meta.count.toString()} payload(s), ${consumed.toString()} consumed, ` +
                `${remaining.toString()} remaining`,
            );
          }
          continue;
        }
        if (store.meta.flow === "public-decrypt") {
          const entries = await readdir(store.dir);
          const usedKs = entries.flatMap((entry) => {
            const match = /^cursor-combos-k(\d+)\.json$/.exec(entry);
            return match?.[1] ? [Number(match[1])] : [];
          });
          const ks = [...new Set([
            ...usedKs,
            ...Array.from({ length: Math.min(4, store.meta.count) }, (_, index) => index + 1),
          ])].sort((a, b) => a - b);
          const combinations = ks.map((k) => {
            const capacity = binomial(store.meta.count, k);
            const used = store.cursor(`combos-k${k.toString()}`).position;
            return {
              k,
              capacity: capacity.toString(),
              consumed: used.toString(),
              remaining: (capacity > used ? capacity - used : 0n).toString(),
            };
          });
          if (json) {
            statuses.push({ pool: name, flow: store.meta.flow, count: store.meta.count, combinations });
          } else {
            logger.info(`${name}: ${store.meta.count.toString()} reusable handle(s)`);
            for (const entry of combinations) {
              logger.info(
                `  k=${entry.k.toString()}: ${entry.capacity} combinations, ${entry.consumed} consumed, ` +
                  `${entry.remaining} remaining`,
              );
            }
          }
          continue;
        }
        const owners = store.meta.ownerIndices ?? [];
        const acl = store.meta.flow === "delegated-user-decrypt"
          ? owners.map((owner) => {
              const expiration = (store.meta.delegationExpirations ?? {})[owner.toString()];
              const state = expiration === undefined
                ? "missing"
                : BigInt(expiration) <= nowSeconds ? "expired" : "healthy";
              return { owner, expiration, state };
            })
          : undefined;
        if (json) {
          statuses.push({
            pool: name,
            flow: store.meta.flow,
            count: store.meta.count,
            owners,
            ...(acl ? { acl } : {}),
          });
        } else {
          logger.info(
            `${name}: ${store.meta.count.toString()} reusable handle(s), owners ` +
              `[${owners.join(", ")}]`,
          );
          if (store.meta.flow === "delegated-user-decrypt") {
            const expirations = store.meta.delegationExpirations ?? {};
            for (const owner of owners) {
              logger.info(
                `  owner ${owner.toString()} ACL ` +
                  describeAclExpiration(expirations[owner.toString()], nowSeconds),
              );
            }
          }
        }
      }
      if (json) emitJson(statuses);
    });
};
