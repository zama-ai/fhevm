/**
 * Materializes generated runtime artifacts such as env files, compose overrides, relayer config, and address outputs.
 */
import fs from "node:fs/promises";
import path from "node:path";

import type { StackSpec } from "../stack-spec/stack-spec";
import { renderRelayerConfig } from "./config";
import { renderEnvMaps, type WalletMaterial } from "./env";
import {
  renderGatewayAddressesEnv,
  renderGatewayAddressesSolidity,
  renderHostChainAddresses,
  renderHostChainAddressesSolidity,
  renderPaymentBridgingAddressesSolidity,
} from "./addresses";
import { generateComposeOverrides } from "./compose";
import { run } from "../utils/process";
import type { State } from "../types";
import {
  ADDRESS_DIR,
  COMPONENTS,
  ENV_DIR,
  GENERATED_CONFIG_DIR,
  TEMPLATE_ENV_DIR,
  TEMPLATE_RELAYER_CONFIG,
  envPath,
  gatewayAddressesPath,
  gatewayAddressesSolidityPath,
  hostChainAddressesPath,
  hostChainAddressesSolidityPath,
  paymentBridgingAddressesSolidityPath,
  relayerConfigPath,
  versionsEnvPath,
} from "../layout";
import { CommandError } from "../errors";
import { ensureDir, readEnvFile, writeEnvFile } from "../utils/fs";

const ensureWritableDir = async (dir: string) => {
  await ensureDir(dir);
  await fs.chmod(dir, 0o777);
};

const writeWritableFile = async (file: string, contents: string) => {
  await fs.writeFile(file, contents);
  await fs.chmod(file, 0o666);
};

const deriveWallet = async (mnemonic: string, index: number): Promise<WalletMaterial> => {
  let addrResult;
  let keyResult;
  try {
    [addrResult, keyResult] = await Promise.all([
      run(["cast", "wallet", "address", "--mnemonic", mnemonic, "--mnemonic-index", String(index)]),
      run(["cast", "wallet", "private-key", "--mnemonic", mnemonic, "--mnemonic-index", String(index)]),
    ]);
  } catch (error) {
    if (error instanceof CommandError) {
      throw new Error(`cast wallet failed: ${error.stderr}`);
    }
    throw error;
  }
  const address = addrResult.stdout.trim();
  const privateKey = keyResult.stdout.trim();
  if (!/^0x[a-fA-F0-9]{40}$/.test(address)) {
    throw new Error(`cast returned invalid address for wallet ${index}: ${address}`);
  }
  if (!/^0x[a-fA-F0-9]{64}$/.test(privateKey)) {
    throw new Error(`cast returned invalid private key for wallet ${index}`);
  }
  return { address, privateKey };
};

/** Materializes all generated runtime files needed to boot the current plan. */
export const generateRuntime = async (state: State, plan: StackSpec) => {
  await Promise.all([
    ensureDir(ENV_DIR),
    ensureWritableDir(path.join(ADDRESS_DIR, "gateway")),
    ...plan.hostChains.map((chain) => ensureWritableDir(path.join(ADDRESS_DIR, chain.key))),
    ensureDir(GENERATED_CONFIG_DIR),
  ]);

  const templateEnvs = Object.fromEntries(
    await Promise.all(
      COMPONENTS.map(async (component) => [
        component,
        await readEnvFile(path.join(TEMPLATE_ENV_DIR, `.env.${component}`)),
      ]),
    ),
  ) as Record<string, Record<string, string>>;

  const rendered = await renderEnvMaps(state, plan, templateEnvs, deriveWallet);
  for (const component of COMPONENTS) {
    await writeEnvFile(envPath(component), rendered.componentEnvs[component]);
  }
  for (const [name, env] of Object.entries(rendered.instanceEnvs)) {
    await writeEnvFile(envPath(name), env);
  }
  await writeEnvFile(versionsEnvPath, rendered.versionsEnv);

  await fs.writeFile(
    relayerConfigPath,
    renderRelayerConfig(state, await fs.readFile(TEMPLATE_RELAYER_CONFIG, "utf8"), plan),
  );
  await writeWritableFile(gatewayAddressesPath, renderGatewayAddressesEnv(state));
  await writeWritableFile(gatewayAddressesSolidityPath, renderGatewayAddressesSolidity(state));
  await writeWritableFile(
    paymentBridgingAddressesSolidityPath,
    renderPaymentBridgingAddressesSolidity(rendered.componentEnvs["gateway-sc"]),
  );
  for (const chain of plan.hostChains) {
    await writeWritableFile(hostChainAddressesPath(chain.key), renderHostChainAddresses(state, chain.key));
    await writeWritableFile(hostChainAddressesSolidityPath(chain.key), renderHostChainAddressesSolidity(state, chain.key));
  }

  await generateComposeOverrides(state, plan);
};
