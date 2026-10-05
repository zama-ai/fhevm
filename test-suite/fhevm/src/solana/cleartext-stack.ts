// cleartext-stack — a local Solana chain whose zama-host is the cleartext build: every handle's
// plaintext is kept in the accounts the host writes, so encrypt, compute and decrypt run with no
// coprocessor, KMS, relayer or gateway. The SDK's `@fhevm/sdk/solana/cleartext` clients sign as the
// parties with the keys this stack registers and read plaintexts back from the same accounts.
//
// The programs load at genesis at their deployed ids (the cleartext host in place of zama_host),
// and the only transactions before a scenario are the host bootstrap's. The stack is its own
// validator on its own ports, so it runs next to the real local stack.
//
// Run directly to keep one up for a development loop (stops on Ctrl-C), then point scenarios at it:
//   bun run src/solana/cleartext-stack.ts
//   SOLANA_E2E_SOURCE=cleartext bun test e2e/scenarios/fhe-vertical.scenario.test.ts
// `SOLANA_E2E_SOURCE=cleartext bun run test:e2e` starts and stops its own instead.
import { mkdir, rm, writeFile } from 'node:fs/promises';
import path from 'node:path';

import { createSolanaRpc } from '@solana/kit';

import { bootstrapZamaHost } from '../../../../solana/deploy/src/bootstrap';
import { evmAddressBytes } from '../../../../solana/deploy/src/gateway';
import { createHostDeployContext } from '../../../../solana/deploy/src/send';
import { solanaPubkeyFromKeypairFile } from '../generate/solana';
import {
  REPO_ROOT,
  SOLANA_CLEARTEXT_FAUCET_PORT,
  SOLANA_CLEARTEXT_GOSSIP_PORT,
  SOLANA_CLEARTEXT_DIR,
  SOLANA_CLEARTEXT_RPC_PORT,
  solanaCleartextDeployerPath,
} from '../layout';
import { runStreaming } from '../utils/process';
import { until } from '../utils/until';
import { CLEARTEXT_SOLANA_ENDPOINTS } from './endpoints';
import { createProvisioningContext, generateSolanaKeypair, loadKeypairSigner } from './provision';
import {
  SOLANA_E2E_PROGRAMS,
  SOLANA_SPECIMEN_PROGRAMS,
  assertAlpenglowActive,
  genesisDeployedPrograms,
  validatorStartArgs,
} from './validator';

const SOLANA_DIR = path.join(REPO_ROOT, 'solana');
const SDK_DIR = path.join(REPO_ROOT, 'sdk', 'js-sdk');
const DEPLOY_DIR = path.join(SOLANA_DIR, 'target', 'deploy');
const BUILT_PROGRAMS = ['zama_host_cleartext', ...SOLANA_E2E_PROGRAMS.filter((program) => program !== 'zama_host')];

export type CleartextStack = {
  readonly rpcUrl: string;
  /** The stack's own funded wallet: upgrade authority of every program and HostConfig admin. */
  readonly deployerKeypairPath: string;
  stop(): Promise<void>;
};

/**
 * Builds the programs and the SDK, starts a fresh validator with the programs at genesis, and
 * bootstraps the host with the SDK's cleartext parties.
 */
export const startCleartextStack = async (): Promise<CleartextStack> => {
  const rpcUrl = CLEARTEXT_SOLANA_ENDPOINTS.validatorRpc;
  const rpc = createSolanaRpc(rpcUrl);
  // Starting would wipe the ledger under a running one.
  if ((await rpc.getHealth().send().catch(() => undefined)) === 'ok') {
    throw new Error(`a cleartext stack already runs at ${rpcUrl}; stop it, or run \`bun test\` against it`);
  }
  await Promise.all([
    runStreaming(['bash', 'scripts/build-programs.sh', 'preview-env', ...BUILT_PROGRAMS], { cwd: SOLANA_DIR }),
    runStreaming(['npm', 'run', 'build:esm'], { cwd: SDK_DIR }),
  ]);
  // After the build: the package resolves to its build output.
  const { SOLANA_CLEARTEXT_GATEWAY, SOLANA_CLEARTEXT_SIGNER_ADDRESSES } = await import('@fhevm/sdk/solana/cleartext');

  const ledgerDir = path.join(SOLANA_CLEARTEXT_DIR, 'ledger');
  await rm(ledgerDir, { recursive: true, force: true });
  await mkdir(ledgerDir, { recursive: true });
  const deployerKeypairPath = solanaCleartextDeployerPath;
  await writeFile(deployerKeypairPath, JSON.stringify([...(await generateSolanaKeypair()).bytes]), { mode: 0o600 });

  const specimens = SOLANA_SPECIMEN_PROGRAMS.map((program) => ({
    address: solanaPubkeyFromKeypairFile(path.join(SOLANA_DIR, 'scripts/e2e/test-keypairs', `${program}-keypair.json`)),
    soPath: path.join(DEPLOY_DIR, `${program}.so`),
  }));
  const deployed = genesisDeployedPrograms(deployerKeypairPath).map((program) =>
    program.soPath.endsWith('/zama_host.so')
      ? { ...program, soPath: path.join(DEPLOY_DIR, 'zama_host_cleartext.so') }
      : program,
  );
  const logPath = path.join(SOLANA_CLEARTEXT_DIR, 'validator.log');
  const validator = Bun.spawn(
    [
      ...validatorStartArgs({
        ledgerDir,
        rpcPort: SOLANA_CLEARTEXT_RPC_PORT,
        genesisPrograms: specimens,
        genesisUpgradeablePrograms: deployed,
      }),
      '--faucet-port',
      String(SOLANA_CLEARTEXT_FAUCET_PORT),
      '--gossip-port',
      String(SOLANA_CLEARTEXT_GOSSIP_PORT),
    ],
    { stdin: 'ignore', stdout: Bun.file(logPath), stderr: Bun.file(logPath) },
  );
  const stop = async () => {
    validator.kill();
    await validator.exited;
  };

  try {
    const health = await until(
      async () => (validator.exitCode !== null ? 'exited' : (await rpc.getHealth().send()) === 'ok' && 'ok'),
      { timeoutMs: 60_000, intervalMs: 1_000, description: `cleartext validator health; see ${logPath}` },
    );
    if (health === 'exited') throw new Error(`cleartext validator exited; see ${logPath}`);
    await assertAlpenglowActive(rpcUrl);

    const payer = await loadKeypairSigner(deployerKeypairPath);
    await createProvisioningContext(rpcUrl, CLEARTEXT_SOLANA_ENDPOINTS.validatorWs).fundSol(payer.address, 1_000);

    const gateway = SOLANA_CLEARTEXT_GATEWAY;
    await bootstrapZamaHost(createHostDeployContext(rpcUrl), {
      payer,
      gateway: {
        gatewayChainId: BigInt(gateway.id),
        inputVerificationContract: evmAddressBytes(gateway.contracts.inputVerification.address),
        decryptionContract: evmAddressBytes(gateway.contracts.decryption.address),
        coprocessorSigners: SOLANA_CLEARTEXT_SIGNER_ADDRESSES.coprocessor.map(evmAddressBytes),
        kmsSigners: SOLANA_CLEARTEXT_SIGNER_ADDRESSES.kms.map(evmAddressBytes),
      },
    });
    return { rpcUrl, deployerKeypairPath, stop };
  } catch (error) {
    await stop();
    throw error;
  }
};

if (import.meta.main) {
  const stack = await startCleartextStack();
  console.log(`cleartext Solana stack up at ${stack.rpcUrl} (deployer ${stack.deployerKeypairPath})`);
  process.on('SIGINT', () => void stack.stop().then(() => process.exit(0)));
}
