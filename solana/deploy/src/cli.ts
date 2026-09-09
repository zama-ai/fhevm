// Solana deployer: host deploy|upgrade, demos deploy|upgrade.
import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import path from 'node:path';

import { writeSolanaAddressArtifact } from './artifact';
import { SOLANA_DEPLOY_PROGRAMS, type SolanaDeployProgram } from './constants';
import { registerSolanaCoprocessorSql } from './coprocessor';
import { deployHostProgram } from './deploy-host';
import { deployProgramArtifacts } from './deploy-programs';
import { readGatewayBootstrapInputsFromEnv } from './gateway';
import { resolveKeypairPath } from './keypair';
import { withDeploymentLock } from './lock';
import { programIdsFor, readSolanaProgramProfile } from './program-profile';

const requiredEnv = (name: string): string => {
  const value = process.env[name];
  if (!value) throw new Error(`missing required env ${name}`);
  return value;
};

const integerEnv = (name: string, fallback: number): number => {
  const raw = process.env[name];
  if (raw === undefined || raw === '') return fallback;
  const value = Number(raw);
  if (!Number.isSafeInteger(value) || value < 0) throw new Error(`${name} must be a non-negative integer`);
  return value;
};

const evmHex = (bytes: Uint8Array): string => `0x${Buffer.from(bytes).toString('hex')}`;

const KEYPAIR_DIR = process.env.SOLANA_KEYPAIR_DIR ?? '/tmp/solana-deploy-keypairs';
const ARTIFACTS_DIR = process.env.SOLANA_ARTIFACTS_DIR ?? '/app/programs';
const ADDRESSES_DIR = process.env.ADDRESSES_DIR ?? '/app/addresses';

const resolveProgramKeypairs = async (
  programs: readonly SolanaDeployProgram[],
): Promise<Partial<Record<SolanaDeployProgram, string>>> => {
  const paths: Partial<Record<SolanaDeployProgram, string>> = {};
  for (const program of programs) {
    const envName = `SOLANA_${program.toUpperCase()}_KEYPAIR`;
    const jsonName = `${envName}_JSON`;
    const fallbackPath = path.join(ARTIFACTS_DIR, `${program}-keypair.json`);
    if (!process.env[envName] && !process.env[jsonName] && !existsSync(fallbackPath)) continue;
    paths[program] = await resolveKeypairPath({
      pathEnv: process.env[envName],
      jsonEnv: process.env[jsonName],
      fallbackPath,
      writePath: path.join(KEYPAIR_DIR, `${program}-keypair.json`),
    });
  }
  return paths;
};

const resolveDeployerKeypairPath = (): Promise<string> =>
  resolveKeypairPath({
    pathEnv: process.env.SOLANA_DEPLOYER_KEYPAIR,
    jsonEnv: process.env.SOLANA_DEPLOYER_KEYPAIR_JSON,
    fallbackPath: `${process.env.HOME ?? '/home/fhevm'}/.config/solana/id.json`,
    writePath: path.join(KEYPAIR_DIR, 'deployer-keypair.json'),
  });

if (process.argv.includes('--help')) {
  console.log('usage: host deploy|upgrade; demos deploy|upgrade; coprocessor register');
  process.exit(0);
}

const main = async () => {
  const profile = readSolanaProgramProfile();
  const programIds = programIdsFor(profile);

  const [target, action] = process.argv.slice(2);
  if (target === 'coprocessor' && action === 'register') {
    const result = spawnSync('psql', ['-X', '--set', 'ON_ERROR_STOP=1'], {
      env: { ...process.env, PGDATABASE: requiredEnv('DATABASE_URL') },
      input: registerSolanaCoprocessorSql(programIds.zamaHost, requiredEnv('SOLANA_KEY_SOURCE_CHAIN_ID')),
      encoding: 'utf8',
    });
    if (result.error || result.status !== 0)
      throw new Error(
        'coprocessor registration failed; check database connectivity, host identity and canonical key availability',
      );
    await writeSolanaAddressArtifact(ADDRESSES_DIR, { zama_host: programIds.zamaHost });
  } else {
    if ((target !== 'host' && target !== 'demos') || (action !== 'deploy' && action !== 'upgrade')) {
      throw new Error('usage: host deploy|upgrade OR demos deploy|upgrade');
    }
    const programs =
      target === 'host' ? (['zama_host'] as const) : SOLANA_DEPLOY_PROGRAMS.filter((p) => p !== 'zama_host');
    const programKeypairPaths = await resolveProgramKeypairs(programs);
    const parameters = {
      rpcUrl: requiredEnv('SOLANA_RPC_URL'),
      databaseUrl: requiredEnv('SOLANA_DEPLOY_DATABASE_URL'),
      deployerKeypairPath: await resolveDeployerKeypairPath(),
      artifactsDir: ARTIFACTS_DIR,
      upgrade: action === 'upgrade',
      profile,
    };
    let ids;
    if (target === 'host') {
      const gateway = await readGatewayBootstrapInputsFromEnv();
      console.log(`profile=${profile}; host=${programIds.zamaHost}; gateway_chain_id=${gateway.gatewayChainId}`);
      console.log(
        `coprocessor_signers=${gateway.coprocessorSigners.map(evmHex).join(',')}; kms_signers=${gateway.kmsSigners.map(evmHex).join(',')}`,
      );
      ids = await deployHostProgram({
        ...parameters,
        programKeypairPath: programKeypairPaths.zama_host,
        gateway,
        coprocessorThreshold: integerEnv('COPROCESSOR_THRESHOLD', 1),
        kmsCorruptionThreshold: integerEnv('KMS_THRESHOLD', 0),
      });
    } else {
      ids = await withDeploymentLock(parameters.databaseUrl, parameters.rpcUrl, programIds.zamaHost, (signal) =>
        deployProgramArtifacts({ ...parameters, programs, programKeypairPaths, signal }),
      );
    }
    await writeSolanaAddressArtifact(ADDRESSES_DIR, ids);
    console.log(`${target} ${action} complete`);
  }
};
main().catch((error: unknown) => {
  // Transport error objects can contain credential-bearing URLs in nested context.
  const message = error instanceof Error ? error.message : 'deployment failed';
  console.error(message.replace(/https?:\/\/[^\s"'<>]+/g, '<RPC URL>'));
  process.exitCode = 1;
});
