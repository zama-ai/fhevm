import { Context, Effect, Layer } from "effect";
import fs from "node:fs/promises";
import path from "node:path";
import { CommandRunner } from "./CommandRunner";
import type { RuntimePlan } from "../runtime-plan";
import type { State, VersionBundle } from "../types";
import { renderRelayerConfig } from "../render-config";
import {
  renderEnvMaps,
  type WalletMaterial,
} from "../render-env";
import {
  renderGatewayAddressesEnv,
  renderGatewayAddressesSolidity,
  renderHostAddressesEnv,
  renderHostAddressesSolidity,
  renderPaymentBridgingAddressesSolidity,
} from "../render-addresses";
import {
  COMPONENTS,
  TEMPLATE_ENV_DIR,
  ENV_DIR,
  envPath,
  versionsEnvPath,
  GENERATED_CONFIG_DIR,
  TEMPLATE_RELAYER_CONFIG,
  relayerConfigPath,
  ADDRESS_DIR,
  gatewayAddressesPath,
  gatewayAddressesSolidityPath,
  hostAddressesPath,
  hostAddressesSolidityPath,
  paymentBridgingAddressesSolidityPath,
} from "../layout";
import { readEnvFile, writeEnvFile, ensureDir } from "../utils";

const ensureWritableDir = async (dir: string) => {
  await ensureDir(dir);
  await fs.chmod(dir, 0o777);
};

export const writeWritableFile = async (file: string, contents: string) => {
  await fs.writeFile(file, contents);
  await fs.chmod(file, 0o666);
};

export { resolveEnvMap } from "../render-env";
export { rewriteRelayerConfig } from "../render-config";

export class EnvWriter extends Context.Tag("EnvWriter")<
  EnvWriter,
  {
    readonly generateEnvFiles: (state: State, plan: RuntimePlan) => Effect.Effect<void>;
    readonly writeVersionsEnv: (bundle: VersionBundle) => Effect.Effect<void>;
  }
>() {
  static Live = Layer.effect(
    EnvWriter,
    Effect.gen(function* () {
      const cmd = yield* CommandRunner;

      const deriveWallet = (mnemonic: string, index: number) =>
        Effect.gen(function* () {
          const [addrResult, keyResult] = yield* Effect.all(
            [
              cmd.run([
                "cast", "wallet", "address",
                "--mnemonic", mnemonic, "--mnemonic-index", String(index),
              ]),
              cmd.run([
                "cast", "wallet", "private-key",
                "--mnemonic", mnemonic, "--mnemonic-index", String(index),
              ]),
            ],
            { concurrency: 2 },
          );
          const address = addrResult.stdout.trim();
          const privateKey = keyResult.stdout.trim();
          if (!/^0x[a-fA-F0-9]{40}$/.test(address)) {
            return yield* Effect.die(
              new Error(`cast returned invalid address for wallet ${index}: ${address}`),
            );
          }
          if (!/^0x[a-fA-F0-9]{64}$/.test(privateKey)) {
            return yield* Effect.die(
              new Error(`cast returned invalid private key for wallet ${index}`),
            );
          }
          return { address, privateKey } satisfies WalletMaterial;
        }).pipe(
          Effect.catchTag("CommandError", (e) =>
            Effect.die(new Error(`cast wallet failed: ${e.stderr}`)),
          ),
        );

      return {
        generateEnvFiles: (state, plan) =>
          Effect.gen(function* () {
            yield* Effect.promise(() =>
              Promise.all([
                ensureDir(ENV_DIR),
                ensureWritableDir(path.join(ADDRESS_DIR, "gateway")),
                ensureWritableDir(path.join(ADDRESS_DIR, "host")),
                ensureDir(GENERATED_CONFIG_DIR),
              ]),
            );

            const templateEnvs = yield* Effect.promise(async () =>
              Object.fromEntries(
                await Promise.all(
                  COMPONENTS.map(async (component) => [
                    component,
                    await readEnvFile(path.join(TEMPLATE_ENV_DIR, `.env.${component}`)),
                  ]),
                ),
              ) as Record<string, Record<string, string>>,
            );
            const rendered = yield* renderEnvMaps(
              state,
              plan,
              templateEnvs,
              deriveWallet,
            );

            yield* Effect.promise(async () => {
              for (const component of COMPONENTS) {
                await writeEnvFile(
                  envPath(component),
                  rendered.componentEnvs[component],
                );
              }
              for (const [name, env] of Object.entries(rendered.instanceEnvs)) {
                await writeEnvFile(envPath(name), env);
              }
              await writeEnvFile(versionsEnvPath, rendered.versionsEnv);

              await fs.writeFile(
                relayerConfigPath,
                renderRelayerConfig(
                  state,
                  await fs.readFile(TEMPLATE_RELAYER_CONFIG, "utf8"),
                ),
              );
              if (state.discovery) {
                await writeWritableFile(
                  gatewayAddressesPath,
                  renderGatewayAddressesEnv(state),
                );
                await writeWritableFile(
                  gatewayAddressesSolidityPath,
                  renderGatewayAddressesSolidity(state),
                );
                await writeWritableFile(
                  paymentBridgingAddressesSolidityPath,
                  renderPaymentBridgingAddressesSolidity(
                    rendered.componentEnvs["gateway-sc"],
                  ),
                );
                await writeWritableFile(
                  hostAddressesPath,
                  renderHostAddressesEnv(state),
                );
                await writeWritableFile(
                  hostAddressesSolidityPath,
                  renderHostAddressesSolidity(state),
                );
              }
            });
          }),

        writeVersionsEnv: (bundle) =>
          Effect.promise(() => writeEnvFile(versionsEnvPath, bundle.env)),
      };
    }),
  );
}
