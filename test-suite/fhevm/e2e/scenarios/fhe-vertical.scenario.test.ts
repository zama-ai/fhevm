// Scenario: the specimen decrypt vertical — one wallet drives the `encrypted-counter` specimen
// program (`solana/programs/encrypted-counter`) and decrypts what it wrote, through the SDK's
// one-path authorization: the request names the handle and its `EncryptedValue` account, and the
// KMS Connector reads the account, proves the wallet's allow leaf against the coprocessors, and
// releases the shares. The client fetches no proof and names no ACL key.
//
// Why a specimen program and not a raw `fhe_execute`: the host only accepts a value whose authority
// is a PDA of the program that declares it (RFC 035), so a wallet cannot own a value it wrote
// itself. The counter is the smallest program that does: `initialize` creates the owner's count at
// 0, `increment(amount)` adds to it, and both allow the owner on the handle they install.
//
// Assertion map:
//   [compute]  the write confirmed and installed a nonzero current handle -> `initializeCounter` /
//              `incrementCounter` return the handle they read back from the account.
//   [compute]  SNS commit -> `stack.waitForSnsCommit(handle)`.
//   [user-decrypt]  pure-SDK cleartext == expected -> `userDecryptExpect` (ML-KEM keygen, ed25519
//              permit, in-SDK de-signcryption) with UD_EXPECTED pinned.
//   [historical-user-decrypt]  a later write replaces the current handle, and the OLD handle still
//              decrypts to its old value: the Connector proves the old allow leaf from the account's
//              history, no client-side proof involved. The scenario asserts the update really
//              rotated the current handle before decrypting both.
import { describe, expect, test } from 'bun:test';
import { cp, mkdtemp, readFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';

import { restartDemoSolanaListener } from '../../demo/lifecycle';
import { REPO_ROOT } from '../../src/layout';
import { readGatewayBootstrapInputs } from '../../src/solana/addresses';
import { readCoprocessorDatabaseUrl } from '../../src/solana/deploy';
import { userDecryptExpect } from '../../src/solana/fhe-vertical';
import { deployHostProgram } from '../../../../solana/deploy/src/deploy-host';
import { incrementCounter, initializeCounter } from '../../src/solana/specimens';
import { runStreaming } from '../../src/utils/process';
import { verticalSetup } from '../harness/solana/vertical';

// Each phase does its own write + SNS commit wait (up to ~3min) + KMS round-trips.
const SCENARIO_TIMEOUT_MS = 15 * 60_000;

const hex = (bytes: Uint8Array): string => `0x${Buffer.from(bytes).toString('hex')}`;

describe('solana specimen decrypt vertical', () => {
  test(
    'counter: initialize -> increment(42) -> pure-SDK user-decrypt == 42',
    async () => {
      const { stack, context, wallet, config, secretKey } = await verticalSetup();

      await initializeCounter(context, wallet.signer);
      const { value, handle } = await incrementCounter(context, wallet.signer, 42n);
      expect(handle.some((byte) => byte !== 0)).toBe(true);
      await stack.waitForSnsCommit(hex(handle));

      const decrypted = await userDecryptExpect(config, {
        encryptedState: value.encryptedState,
        handle,
        secretKey,
        expected: 42n,
      });
      expect(decrypted).toBe(42n);
    },
    SCENARIO_TIMEOUT_MS,
  );

  test(
    'historical decrypt: increment again, then user-decrypt the OLD handle and the current one',
    async () => {
      const { stack, context, wallet, config, secretKey } = await verticalSetup();

      await initializeCounter(context, wallet.signer);
      const original = await incrementCounter(context, wallet.signer, 42n);
      // The old handle must be SNS-committed before the update buries it.
      await stack.waitForSnsCommit(hex(original.handle));

      const updated = await incrementCounter(context, wallet.signer, 7n);
      expect(hex(updated.handle)).not.toBe(hex(original.handle));
      expect(updated.value.encryptedState).toBe(original.value.encryptedState);
      await stack.waitForSnsCommit(hex(updated.handle));

      const encryptedState = original.value.encryptedState;
      expect(await userDecryptExpect(config, { encryptedState, handle: original.handle, secretKey, expected: 42n })).toBe(42n);
      expect(await userDecryptExpect(config, { encryptedState, handle: updated.handle, secretKey, expected: 49n })).toBe(49n);
    },
    SCENARIO_TIMEOUT_MS,
  );
});

// The same source compiled with another optimization level supplies a genuinely different,
// compatible executable without introducing a test-only instruction into the host program.
test(
  'host upgrade and listener restart retain old decryptable values',
  async () => {
    const { env, stack, context, wallet, config, secretKey } = await verticalSetup();
    const directory = await mkdtemp(path.join(tmpdir(), 'solana-upgrade-'));
    const artifactsDir = path.join(REPO_ROOT, 'solana/target/deploy');
    const databaseUrl = await readCoprocessorDatabaseUrl();
    const bootstrap = {
      gateway: await readGatewayBootstrapInputs({ gatewayRpcUrl: env.gatewayRpcUrl }),
      coprocessorThreshold: Number(process.env.COPROCESSOR_THRESHOLD ?? 1),
      kmsCorruptionThreshold: Number(process.env.KMS_THRESHOLD ?? 0),
    };
    const rollout = (directory: string, upgrade: boolean) =>
      deployHostProgram({
        ...bootstrap,
        databaseUrl,
        rpcUrl: env.rpcUrl,
        deployerKeypairPath: env.roots.deployerKeypairPath,
        artifactsDir: directory,
        upgrade,
      });
    let upgraded = false;
    let passed = false;
    try {
      await initializeCounter(context, wallet.signer);
      const original = await incrementCounter(context, wallet.signer, 42n);
      await stack.waitForSnsCommit(hex(original.handle));
      await rollout(artifactsDir, false);
      await cp(path.join(artifactsDir, 'zama_host.so'), path.join(directory, 'zama_host.so'));
      await runStreaming(['bash', 'scripts/build-programs.sh', 'localnet', 'zama_host'], {
        cwd: path.join(REPO_ROOT, 'solana'),
        env: { CARGO_PROFILE_RELEASE_OPT_LEVEL: '2', SBF_OUT_PATH: path.join(directory, 'upgrade') },
      });
      expect(await readFile(path.join(directory, 'upgrade/zama_host.so'))).not.toEqual(
        await readFile(path.join(directory, 'zama_host.so')),
      );
      await rollout(path.join(directory, 'upgrade'), true);
      upgraded = true;
      await restartDemoSolanaListener();
      const encryptedState = original.value.encryptedState;
      expect(
        await userDecryptExpect(config, { encryptedState, handle: original.handle, secretKey, expected: 42n }),
      ).toBe(42n);
      const updated = await incrementCounter(context, wallet.signer, 7n);
      await stack.waitForSnsCommit(hex(updated.handle));
      expect(
        await userDecryptExpect(config, { encryptedState, handle: updated.handle, secretKey, expected: 49n }),
      ).toBe(49n);
      expect(
        await userDecryptExpect(config, { encryptedState, handle: original.handle, secretKey, expected: 42n }),
      ).toBe(42n);
      passed = true;
    } finally {
      try {
        if (upgraded) await rollout(directory, true);
      } catch (error) {
        if (passed) throw error;
        console.error('Restoring the original host after a failed upgrade test also failed:', error);
      } finally {
        await rm(directory, { recursive: true, force: true });
      }
    }
  },
  20 * 60_000,
);
