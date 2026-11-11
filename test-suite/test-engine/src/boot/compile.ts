/**
 * `test-engine compile`: compiles the e2e contracts for the profile's environment. The coprocessor
 * config (`contracts/E2ECoprocessorConfigLocal.sol`) is written by the e2e project's own generator
 * (`scripts/generate-coprocessor-config.ts`, the one `run-tests.sh` uses), fed with the profile's
 * addresses; the engine adds no generator of its own. After `hardhat compile`, the compiled config
 * must embed the addresses, and a build stamp records what the artifacts were compiled for.
 */
import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import path from 'node:path';

import type { ResolvedProfile } from '../config/profiles/types.js';
import type { Redactor } from '../infrastructure/secrets/redactor.js';
import {
  COMPILED_ADDRESSES,
  type CompiledAddresses,
  compiledAddressesOf,
  writeBuildStamp,
} from '../runtime/modules/contracts/build-stamp.js';
import { Contracts } from '../runtime/modules/contracts/contracts.js';

/** Generator of the e2e project, relative to the project root. */
export const GENERATOR_SCRIPT = path.join('scripts', 'generate-coprocessor-config.ts');
/** The contract the generated config defines; every e2e test contract inherits it. */
export const CONFIG_CONTRACT = 'E2ECoprocessorConfig';

/** Runs a command in `cwd` with `env`, forwarding its output; resolves to its exit code. */
export type CommandRunner = (
  command: string,
  args: string[],
  options: { cwd: string; env: NodeJS.ProcessEnv },
) => Promise<number>;

export interface CompileOptions {
  /** Root of the e2e Hardhat project. */
  project: string;
  /** Applied to the commands' output: the generator prints the addresses, which may be secret. */
  redactor: Redactor;
  /** Replaces the process runner (unit tests). */
  run?: CommandRunner;
  /** Base environment of the commands (default: `process.env`). */
  env?: NodeJS.ProcessEnv;
  now?: () => Date;
}

export interface CompileResult {
  artifactsDir: string;
  stampFile: string;
  addresses: CompiledAddresses;
}

/** A compilation that could not be done or verified. The CLI exits 2. */
export class CompileError extends Error {}

/** Spawns the command and forwards its output line by line through `redactor`. */
export function processRunner(redactor: Redactor): CommandRunner {
  return (command, args, { cwd, env }) =>
    new Promise((resolve, reject) => {
      const child = spawn(command, args, { cwd, env, stdio: ['ignore', 'pipe', 'pipe'] });
      const forward = (stream: NodeJS.ReadableStream, out: NodeJS.WriteStream) => {
        let pending = '';
        stream.setEncoding('utf8');
        stream.on('data', (chunk: string) => {
          const lines = (pending + chunk).split('\n');
          pending = lines.pop() ?? '';
          for (const line of lines) out.write(`${redactor.redact(line)}\n`);
        });
        stream.on('end', () => {
          if (pending !== '') out.write(`${redactor.redact(pending)}\n`);
        });
      };
      forward(child.stdout, process.stdout);
      forward(child.stderr, process.stderr);
      child.once('error', reject);
      child.once('close', (code) => resolve(code ?? 1));
    });
}

export async function compileContracts(profile: ResolvedProfile, options: CompileOptions): Promise<CompileResult> {
  const { contracts } = profile;
  if (!contracts) throw new CompileError(`profile '${profile.name}' has no contracts section: nothing to compile for`);
  const project = path.resolve(options.project);
  const missing = [GENERATOR_SCRIPT, 'hardhat.config.ts'].filter((file) => !existsSync(path.join(project, file)));
  if (missing.length > 0) {
    throw new CompileError(
      `${project} is not an e2e project with the coprocessor config generator (missing ${missing.join(', ')}); pass --project <test-suite/e2e>`,
    );
  }

  const addresses = compiledAddressesOf(contracts);
  const env: NodeJS.ProcessEnv = {
    ...(options.env ?? process.env),
    E2E_COPROCESSOR_CONFIG_FROM_ENV: 'true',
    ACL_CONTRACT_ADDRESS: addresses.acl,
    FHEVM_EXECUTOR_CONTRACT_ADDRESS: addresses.executor,
    KMS_VERIFIER_CONTRACT_ADDRESS: addresses.kmsVerifier,
  };
  const run = options.run ?? processRunner(options.redactor);
  const step = async (what: string, command: string, args: string[]) => {
    let code: number;
    try {
      code = await run(command, args, { cwd: project, env });
    } catch (error) {
      throw new CompileError(`${what}: cannot start '${command}': ${(error as Error).message}`);
    }
    if (code !== 0) throw new CompileError(`${what} failed (exit code ${code}); see the output above`);
  };
  await step('generating the coprocessor config', 'npx', ['ts-node', '--transpile-only', GENERATOR_SCRIPT]);
  await step('compiling the contracts', 'npx', ['hardhat', 'compile']);

  // Stale bytecode would still hold the old addresses: check the compiled config itself.
  const artifactsDir = path.join(project, 'artifacts');
  let bytecode: string;
  try {
    bytecode = new Contracts(artifactsDir).artifact(CONFIG_CONTRACT).bytecode.toLowerCase();
  } catch (error) {
    throw new CompileError(`cannot verify the compiled coprocessor config: ${(error as Error).message}`);
  }
  const absent = COMPILED_ADDRESSES.filter((field) => !bytecode.includes(addresses[field].slice(2).toLowerCase()));
  if (absent.length > 0) {
    throw new CompileError(
      `the compiled ${CONFIG_CONTRACT} does not embed the profile's ${absent.map((field) => `contracts.${field}`).join(', ')}`,
    );
  }

  const stampFile = writeBuildStamp(artifactsDir, {
    profile: profile.name,
    addresses,
    compiledAt: (options.now ?? (() => new Date()))().toISOString(),
  });
  return { artifactsDir, stampFile, addresses };
}
