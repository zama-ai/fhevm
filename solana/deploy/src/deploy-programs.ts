import { address, createSolanaRpc } from '@solana/kit';
import { createHmac, createPrivateKey, createPublicKey } from 'node:crypto';
import { writeKeypairJson, parseKeypairBytes } from './keypair';
import { spawn } from 'node:child_process';
import { access, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';

import { type SolanaDeployProgram } from './constants';
import generated from './generated/program-ids.json';
import { DEFAULT_SOLANA_ENVIRONMENT, type SolanaEnvironment, deployedProgramIds } from './environment';

const declaredProgramId = (environment: SolanaEnvironment): Partial<Record<SolanaDeployProgram, string>> => ({
  ...deployedProgramIds(environment),
  // The e2e specimens exist only on the test validator; their ids come from the committed IDLs.
  encrypted_counter: generated.encrypted_counter,
  dep_chain: generated.dep_chain,
});

const run = (argv: string[], signal?: AbortSignal): Promise<string> =>
  new Promise((resolve, reject) => {
    const child = spawn(argv[0]!, argv.slice(1), { stdio: ['ignore', 'pipe', 'pipe'], signal });
    let stdout = '';
    child.stdout.on('data', (chunk: Buffer) => {
      stdout += chunk.toString();
    });
    child.stderr.resume(); // Drain private diagnostics without retaining or emitting them.
    child.on('error', () => reject(new Error('Solana CLI could not start')));
    child.on('close', (code) => {
      if (code === 0) {
        resolve(stdout.trim());
        return;
      }
      // CLI failures can contain a buffer recovery mnemonic or credentials in RPC URLs.
      reject(new Error(`${argv[0]} ${argv[1]} failed (exit ${code}); private CLI diagnostics suppressed`));
    });
  });

const addressOf = (keypairPath: string): Promise<string> => run(['solana', 'address', '-k', keypairPath]);

/** Stable private buffer key; an interrupted upload is recoverable without its ephemeral disk. */
export const uploadBufferBytes = async (deployerPath: string, program: string): Promise<Uint8Array> => {
  const parent = parseKeypairBytes(await readFile(deployerPath, 'utf8'));
  const seed = createHmac('sha256', parent.subarray(0, 32)).update(`fhevm-preview-upload-v1:${program}`).digest();
  const privateKey = createPrivateKey({
    key: Buffer.concat([Buffer.from('302e020100300506032b657004220420', 'hex'), seed]),
    format: 'der',
    type: 'pkcs8',
  });
  const publicKey = createPublicKey(privateKey).export({ format: 'der', type: 'spki' }).subarray(-32);
  return Uint8Array.from([...seed, ...publicKey]);
};

export const deployProgramArtifacts = async (parameters: {
  readonly rpcUrl: string;
  readonly signal?: AbortSignal;
  readonly deployerKeypairPath: string;
  readonly artifactsDir: string;
  readonly programKeypairPaths: Readonly<Partial<Record<SolanaDeployProgram, string>>>;
  readonly programs: readonly SolanaDeployProgram[];
  readonly upgrade?: boolean;
  /** Deploy absent programs and upgrade ones whose bytecode differs, when the deployer is the authority. */
  readonly allowUpgrade?: boolean;
  readonly environment?: SolanaEnvironment;
}): Promise<Partial<Record<SolanaDeployProgram, string>>> => {
  parameters.signal?.throwIfAborted();
  const declared = declaredProgramId(parameters.environment ?? DEFAULT_SOLANA_ENVIRONMENT);
  const rpc = createSolanaRpc(parameters.rpcUrl);
  const authority = await addressOf(parameters.deployerKeypairPath);
  const configPath = path.join(path.dirname(parameters.deployerKeypairPath), 'solana-cli-config.json');
  await writeFile(
    configPath,
    JSON.stringify({
      json_rpc_url: parameters.rpcUrl,
      websocket_url: '',
      keypair_path: parameters.deployerKeypairPath,
      address_labels: {},
      commitment: 'confirmed',
    }),
    { mode: 0o600 },
  );
  const command = (args: string[]) => {
    const safeArgs = args.filter((_, index) => args[index] !== '-u' && args[index - 1] !== '-u');
    return run(
      [...safeArgs, '--config', configPath, '-k', parameters.deployerKeypairPath, '--commitment', 'confirmed'],
      parameters.signal,
    );
  };
  const pending: SolanaDeployProgram[] = [];
  const ids: Partial<Record<SolanaDeployProgram, string>> = {};
  for (const program of parameters.programs) {
    parameters.signal?.throwIfAborted();
    const soPath = path.join(parameters.artifactsDir, `${program}.so`);
    const keypairPath = parameters.programKeypairPaths[program];
    await access(soPath);
    const expected = declared[program];
    if (!expected) throw new Error(`${program} is unavailable in environment ${parameters.environment}`);
    const programId = keypairPath ? await addressOf(keypairPath) : expected;
    if (programId !== expected) {
      throw new Error(
        `${program} keypair pubkey ${programId} does not match declare_id! ${expected} for environment ${parameters.environment ?? DEFAULT_SOLANA_ENVIRONMENT}`,
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
          if (!parameters.upgrade && !parameters.allowUpgrade) {
            throw new Error(`${program} bytecode differs; explicit upgrade required`);
          }
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
    // Derivable from the durable deployer key, so even a killed upload cannot lose its buffer.
    const bufferPath = await writeKeypairJson(
      path.join(path.dirname(parameters.deployerKeypairPath), `${program}-buffer.json`),
      JSON.stringify([...(await uploadBufferBytes(parameters.deployerKeypairPath, ids[program]!))]),
    );
    await command([
      'solana',
      'program',
      'deploy',
      '-u',
      parameters.rpcUrl,
      '--upgrade-authority',
      parameters.deployerKeypairPath,
      '--buffer',
      bufferPath,
      '--use-rpc',
      // ~800 write transactions for a 700 KB program; the default 5 re-sign rounds fail on
      // devnet with "Data writes to account failed: Max retries exceeded".
      '--max-sign-attempts',
      '100',
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
