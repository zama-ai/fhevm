import fs from "node:fs/promises";
import path from "node:path";

import type { RuntimePlan } from "./runtime-plan";
import { renderRelayerConfig } from "./render-config";
import { renderEnvMaps, type WalletMaterial } from "./render-env";
import {
  renderGatewayAddressesEnv,
  renderGatewayAddressesSolidity,
  renderHostAddressesEnv,
  renderHostAddressesSolidity,
  renderPaymentBridgingAddressesSolidity,
} from "./render-addresses";
import { generateComposeOverrides } from "./render-compose";
import { run } from "./shell";
import type { State } from "./types";
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
  hostAddressesPath,
  hostAddressesSolidityPath,
  paymentBridgingAddressesSolidityPath,
  relayerConfigPath,
  versionsEnvPath,
} from "./layout";
import { CommandError } from "./errors";
import { ensureDir, readEnvFile, writeEnvFile } from "./utils";

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

export const renderRuntime = async (state: State, plan: RuntimePlan) => {
  await Promise.all([
    ensureDir(ENV_DIR),
    ensureWritableDir(path.join(ADDRESS_DIR, "gateway")),
    ensureWritableDir(path.join(ADDRESS_DIR, "host")),
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
    renderRelayerConfig(state, await fs.readFile(TEMPLATE_RELAYER_CONFIG, "utf8")),
  );
  await writeWritableFile(gatewayAddressesPath, renderGatewayAddressesEnv(state));
  await writeWritableFile(gatewayAddressesSolidityPath, renderGatewayAddressesSolidity(state));
  await writeWritableFile(
    paymentBridgingAddressesSolidityPath,
    renderPaymentBridgingAddressesSolidity(rendered.componentEnvs["gateway-sc"]),
  );
  await writeWritableFile(hostAddressesPath, renderHostAddressesEnv(state));
  await writeWritableFile(hostAddressesSolidityPath, renderHostAddressesSolidity(state));

  await generateComposeOverrides(state, plan);
};
