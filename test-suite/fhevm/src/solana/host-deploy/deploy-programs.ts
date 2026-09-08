import { address, createSolanaRpc } from '@solana/kit';
import { spawn } from 'node:child_process';
import { access, mkdtemp, readFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';

import { DEP_CHAIN_PROGRAM_ADDRESS } from '../internal/generated/depChain/programAddress';
import { ENCRYPTED_COUNTER_PROGRAM_ADDRESS } from '../internal/generated/encryptedCounter/programAddress';
import { type SolanaDeployProgram } from './constants';
import { type SolanaProgramProfile, programIdsFor } from './program-profile';

const declaredProgramId = (profile: SolanaProgramProfile): Partial<Record<SolanaDeployProgram, string>> => {
  const ids = programIdsFor(profile);
  return {
    zama_host: ids.zamaHost,
    confidential_token: ids.confidentialToken,
    demo_vault: ids.demoVault,
    confidential_batcher: ids.confidentialBatcher,
    ...(profile === 'localnet'
      ? {
          encrypted_counter: ENCRYPTED_COUNTER_PROGRAM_ADDRESS,
          dep_chain: DEP_CHAIN_PROGRAM_ADDRESS,
        }
      : {}),
  };
};

const run = (argv: string[], signal?: AbortSignal): Promise<string> =>
  new Promise((resolve, reject) => {
    const child = spawn(argv[0]!, argv.slice(1), { stdio: ['ignore', 'pipe', 'pipe'], signal });
    let stdout = '';
    child.stdout.on('data', (chunk: Buffer) => {
      stdout += chunk.toString();
    });
    let stderr = '';
    child.stderr.on('data', (chunk: Buffer) => {
      if (stderr.length < 16_000) stderr += chunk.toString();
    });
    child.on('error', reject);
    child.on('close', (code) => {
      if (code === 0) {
        resolve(stdout.trim());
        return;
      }
      const diagnostic =
        stderr.length >= 16_000
          ? 'diagnostic omitted (too long)'
          : stderr.replace(/https?:\/\/[^\s"'<>]+/g, '<RPC URL>');
      reject(new Error(`${argv[0]} ${argv[1]} failed (exit ${code}): ${diagnostic.trim()}`));
    });
  });

const addressOf = (keypairPath: string): Promise<string> => run(['solana', 'address', '-k', keypairPath]);

export const deployProgramArtifacts = async (parameters: {
  readonly rpcUrl: string;
  readonly signal?: AbortSignal;
  readonly deployerKeypairPath: string;
  readonly artifactsDir: string;
  readonly programKeypairPaths: Readonly<Partial<Record<SolanaDeployProgram, string>>>;
  readonly programs: readonly SolanaDeployProgram[];
  readonly upgrade?: boolean;
  readonly profile?: SolanaProgramProfile;
}): Promise<Partial<Record<SolanaDeployProgram, string>>> => {
  parameters.signal?.throwIfAborted();
  const declared = declaredProgramId(parameters.profile ?? 'localnet');
  const rpc = createSolanaRpc(parameters.rpcUrl);
  const authority = await addressOf(parameters.deployerKeypairPath);
  const command = (args: string[]) =>
    run([...args, '-k', parameters.deployerKeypairPath, '--commitment', 'confirmed'], parameters.signal);
  const pending: SolanaDeployProgram[] = [];
  const ids: Partial<Record<SolanaDeployProgram, string>> = {};
  for (const program of parameters.programs) {
    parameters.signal?.throwIfAborted();
    const soPath = path.join(parameters.artifactsDir, `${program}.so`);
    const keypairPath = parameters.programKeypairPaths[program];
    await access(soPath);
    const expected = declared[program];
    if (!expected) throw new Error(`${program} is unavailable in profile ${parameters.profile}`);
    const programId = keypairPath ? await addressOf(keypairPath) : expected;
    if (programId !== expected) {
      throw new Error(
        `${program} keypair pubkey ${programId} does not match declare_id! ${expected} for profile ${parameters.profile ?? 'localnet'}`,
      );
    }
    let exists: boolean;
    try {
      exists =
        (await rpc.getAccountInfo(address(programId), { encoding: 'base64', commitment: 'confirmed' }).send()).value !==
        null;
    } catch {
      throw new Error(`cannot inspect ${program}; check RPC connectivity`);
    }
    if (!exists) {
      if (parameters.upgrade) throw new Error(`${program} does not exist; use deploy first`);
      if (!keypairPath) throw new Error(`missing first-deployment keypair for ${program}`);
      pending.push(program);
    } else {
      const info = JSON.parse(
        await command(['solana', 'program', 'show', '-u', parameters.rpcUrl, '--output', 'json', programId]),
      ) as { authority?: string };
      const directory = await mkdtemp(path.join(tmpdir(), 'solana-bytecode-'));
      try {
        const deployedPath = path.join(directory, 'deployed.so');
        await command(['solana', 'program', 'dump', '-u', parameters.rpcUrl, programId, deployedPath]);
        const [expectedBytes, deployedBytes] = await Promise.all([readFile(soPath), readFile(deployedPath)]);
        // Upgradeable program accounts reserve extra space, which the loader zero-fills.
        const unchanged =
          deployedBytes.length >= expectedBytes.length &&
          deployedBytes.subarray(0, expectedBytes.length).equals(expectedBytes) &&
          deployedBytes.subarray(expectedBytes.length).every((byte) => byte === 0);
        if (unchanged) console.log(`    ${program}=${programId} unchanged`);
        else {
          if (!parameters.upgrade) throw new Error(`${program} bytecode differs; explicit upgrade required`);
          if (info.authority !== authority) throw new Error(`${program} upgrade authority does not match deployer`);
          pending.push(program);
        }
      } finally {
        await rm(directory, { recursive: true, force: true });
      }
    }
    ids[program] = programId;
  }
  for (const program of pending) {
    await command([
      'solana',
      'program',
      'deploy',
      '-u',
      parameters.rpcUrl,
      '--upgrade-authority',
      parameters.deployerKeypairPath,
      '--use-rpc',
      '--program-id',
      parameters.programKeypairPaths[program] ?? ids[program]!,
      path.join(parameters.artifactsDir, `${program}.so`),
    ]);
    const info = JSON.parse(
      await command(['solana', 'program', 'show', '-u', parameters.rpcUrl, '--output', 'json', ids[program]!]),
    ) as { lastDeploySlot: number };
    if (!Number.isSafeInteger(info.lastDeploySlot)) throw new Error('Solana CLI returned an invalid deployment slot');
    // The loader activates new bytecode in the next slot. Use the same commitment as bootstrap.
    const deadline = Date.now() + 60_000;
    while ((await rpc.getSlot({ commitment: 'confirmed' }).send()) <= BigInt(info.lastDeploySlot)) {
      parameters.signal?.throwIfAborted();
      if (Date.now() > deadline) throw new Error(`${program} did not become active within 60 seconds`);
      await new Promise((resolve) => setTimeout(resolve, 400));
    }
    console.log(`    ${program}=${ids[program]} deployed`);
  }
  return ids;
};
