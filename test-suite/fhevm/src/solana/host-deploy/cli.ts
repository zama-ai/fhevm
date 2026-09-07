// zama-host deployer CLI. The Helm `charts/contracts` Job runs this as `deployCommands`
// (inside `/app/deploy-contracts.sh`), then scrapes `addresses/.env.solana` into ConfigMap
// `solana-program-ids` the same way it scrapes Hardhat `addresses/.env.host`.
//
//   node /app/cli.mjs all
//
// Env: SOLANA_RPC_URL, GATEWAY_RPC_URL, GATEWAY_CONFIG_ADDRESS, INPUT_VERIFICATION_ADDRESS,
// DECRYPTION_ADDRESS, deployer + program keypairs (path or inline JSON), optional thresholds.

import path from "node:path";

import { writeSolanaAddressArtifact } from "./artifact";
import { bootstrapZamaHost } from "./bootstrap";
import { SOLANA_DEPLOY_PROGRAMS, type SolanaDeployProgram } from "./constants";
import { deployProgramArtifacts } from "./deploy-programs";
import { readGatewayBootstrapInputsFromEnv } from "./gateway";
import { loadKeypairSigner, resolveKeypairPath } from "./keypair";
import { programIdsFor, readSolanaProgramProfile } from "./program-profile";
import { createHostDeployContext } from "./send";

const requiredEnv = (name: string): string => {
  const value = process.env[name];
  if (!value) throw new Error(`missing required env ${name}`);
  return value;
};

const integerEnv = (name: string, fallback: number): number => {
  const raw = process.env[name];
  if (raw === undefined || raw === "") return fallback;
  const value = Number.parseInt(raw, 10);
  if (!Number.isSafeInteger(value) || value < 0) throw new Error(`${name} must be a non-negative integer`);
  return value;
};

const evmHex = (bytes: Uint8Array): string => `0x${Buffer.from(bytes).toString("hex")}`;

const KEYPAIR_DIR = process.env.SOLANA_KEYPAIR_DIR ?? "/tmp/solana-deploy-keypairs";
const ARTIFACTS_DIR = process.env.SOLANA_ARTIFACTS_DIR ?? "/app/programs";
const ADDRESSES_DIR = process.env.ADDRESSES_DIR ?? "/app/addresses";

const resolveProgramKeypairs = async (): Promise<Record<SolanaDeployProgram, string>> => {
  const paths = {} as Record<SolanaDeployProgram, string>;
  for (const program of SOLANA_DEPLOY_PROGRAMS) {
    const envName = `SOLANA_${program.toUpperCase()}_KEYPAIR`;
    const jsonName = `${envName}_JSON`;
    paths[program] = await resolveKeypairPath({
      pathEnv: process.env[envName],
      jsonEnv: process.env[jsonName],
      fallbackPath: path.join(ARTIFACTS_DIR, `${program}-keypair.json`),
      writePath: path.join(KEYPAIR_DIR, `${program}-keypair.json`),
    });
  }
  return paths;
};

const resolveDeployerKeypairPath = (): Promise<string> =>
  resolveKeypairPath({
    pathEnv: process.env.SOLANA_DEPLOYER_KEYPAIR,
    jsonEnv: process.env.SOLANA_DEPLOYER_KEYPAIR_JSON,
    fallbackPath: `${process.env.HOME ?? "/home/fhevm"}/.config/solana/id.json`,
    writePath: path.join(KEYPAIR_DIR, "deployer-keypair.json"),
  });

const profile = readSolanaProgramProfile();
const programIds = programIdsFor(profile);

const deploy = async () => {
  const rpcUrl = requiredEnv("SOLANA_RPC_URL");
  return deployProgramArtifacts({
    rpcUrl,
    deployerKeypairPath: await resolveDeployerKeypairPath(),
    artifactsDir: ARTIFACTS_DIR,
    programKeypairPaths: await resolveProgramKeypairs(),
    profile,
  });
};

const bootstrap = async () => {
  const rpcUrl = requiredEnv("SOLANA_RPC_URL");
  const gateway = await readGatewayBootstrapInputsFromEnv();
  console.log(`    profile=${profile}`);
  console.log(`    zama_host=${programIds.zamaHost}`);
  console.log(`    gateway_chain_id=${gateway.gatewayChainId}`);
  console.log(`    input_verification=${evmHex(gateway.inputVerificationContract)}`);
  console.log(`    decryption=${evmHex(gateway.decryptionContract)}`);
  console.log(`    coprocessor_signers=${gateway.coprocessorSigners.map(evmHex).join(",")}`);
  console.log(`    kms_signers=${gateway.kmsSigners.map(evmHex).join(",")}`);
  const payer = await loadKeypairSigner(await resolveDeployerKeypairPath());
  await bootstrapZamaHost(createHostDeployContext(rpcUrl), {
    payer,
    gateway,
    coprocessorThreshold: integerEnv("COPROCESSOR_THRESHOLD", 1),
    kmsCorruptionThreshold: integerEnv("KMS_THRESHOLD", 0),
    programAddress: programIds.zamaHost,
  });
};

const slot = async (): Promise<string> => {
  const rpcUrl = requiredEnv("SOLANA_RPC_URL");
  const { createSolanaRpc } = await import("@solana/kit");
  const rpc = createSolanaRpc(rpcUrl);
  const value = await rpc.getSlot({ commitment: "confirmed" }).send();
  return String(value);
};

const command = process.argv[2] ?? "all";

if (command === "deploy") {
  await deploy();
} else if (command === "bootstrap") {
  await bootstrap();
} else if (command === "all") {
  const ids = await deploy();
  await bootstrap();
  const bootstrapSlot = await slot();
  await writeSolanaAddressArtifact(ADDRESSES_DIR, { ...ids, bootstrapSlot });
  console.log(`solana programs ready zama_host=${ids.zamaHostId} bootstrap_slot=${bootstrapSlot}`);
} else {
  throw new Error(`unknown command ${command} (expected deploy | bootstrap | all)`);
}
