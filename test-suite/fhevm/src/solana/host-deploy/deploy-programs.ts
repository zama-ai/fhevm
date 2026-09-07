// `solana program deploy --use-rpc` of the .so files baked into the deployer image. Does not
// compile: bytecode (and therefore `declare_id!`) is the image tag. The program keypair secret
// must match that `declare_id!` on first deploy; later runs upgrade in place with the deployer
// as upgrade authority.
import { spawn } from 'node:child_process';
import { access } from 'node:fs/promises';
import path from 'node:path';

import { type SolanaDeployProgram } from './constants';
import { type SolanaProgramProfile, programIdsFor } from './program-profile';

const declaredProgramId = (profile: SolanaProgramProfile): Record<SolanaDeployProgram, string> => {
  const ids = programIdsFor(profile);
  return {
    zama_host: ids.zamaHost,
    confidential_token: ids.confidentialToken,
    demo_vault: ids.demoVault,
    confidential_batcher: ids.confidentialBatcher,
  };
};

const run = (argv: string[]): Promise<string> =>
  new Promise((resolve, reject) => {
    const child = spawn(argv[0]!, argv.slice(1), { stdio: ['ignore', 'pipe', 'pipe'] });
    let stdout = '';
    child.stdout.on('data', (chunk: Buffer) => {
      stdout += chunk.toString();
    });
    // Drain diagnostics: Solana may echo credential-bearing RPC URLs on failure.
    child.stderr.resume();
    child.on('error', reject);
    child.on('close', (code) => {
      if (code === 0) {
        resolve(stdout.trim());
        return;
      }
      reject(new Error(`${argv[0]} ${argv[1]} failed (exit ${code})`));
    });
  });

const addressOf = (keypairPath: string): Promise<string> => run(['solana', 'address', '-k', keypairPath]);

export const deployProgramArtifacts = async (parameters: {
  readonly rpcUrl: string;
  readonly deployerKeypairPath: string;
  readonly artifactsDir: string;
  readonly programKeypairPaths: Readonly<Partial<Record<SolanaDeployProgram, string>>>;
  readonly programs: readonly SolanaDeployProgram[];
  readonly upgrade?: boolean;
  readonly profile?: SolanaProgramProfile;
}): Promise<Partial<Record<SolanaDeployProgram, string>>> => {
  const declared = declaredProgramId(parameters.profile ?? 'localnet');
  const ids: Partial<Record<SolanaDeployProgram, string>> = {};
  for (const program of parameters.programs) {
    const soPath = path.join(parameters.artifactsDir, `${program}.so`);
    const keypairPath = parameters.programKeypairPaths[program];
    if (!keypairPath) throw new Error(`missing keypair for ${program}`);
    await access(soPath);
    await access(keypairPath);
    const programId = await addressOf(keypairPath);
    const expected = declared[program];
    if (programId !== expected) {
      throw new Error(
        `${program} keypair pubkey ${programId} does not match declare_id! ${expected} ` +
          `for profile ${parameters.profile ?? 'localnet'}; rotate ids by regenerating the keypair, updating declare_id!, and rebuilding`,
      );
    }
    if (parameters.upgrade) {
      await run(['solana', 'program', 'show', '-u', parameters.rpcUrl, programId]);
    }
    ids[program] = programId;
  }
  for (const program of parameters.programs) {
    await run([
      'solana',
      'program',
      'deploy',
      '-u',
      parameters.rpcUrl,
      '-k',
      parameters.deployerKeypairPath,
      '--upgrade-authority',
      parameters.deployerKeypairPath,
      '--use-rpc',
      '--program-id',
      parameters.programKeypairPaths[program]!,
      path.join(parameters.artifactsDir, `${program}.so`),
    ]);
    console.log(`    ${program}=${ids[program]} deployed`);
  }
  return ids;
};
