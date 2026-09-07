// Solana deployer: host deploy|upgrade, demos deploy|upgrade.
import { spawnSync } from 'node:child_process';
import path from 'node:path';

import { writeSolanaAddressArtifact } from './artifact';
import { bootstrapZamaHost } from './bootstrap';
import { SOLANA_DEPLOY_PROGRAMS, type SolanaDeployProgram } from './constants';
import { registerSolanaCoprocessorSql } from './coprocessor';
import { deployProgramArtifacts } from './deploy-programs';
import { readGatewayBootstrapInputsFromEnv } from './gateway';
import { loadKeypairSigner, resolveKeypairPath } from './keypair';
import { programIdsFor, readSolanaProgramProfile } from './program-profile';
import { createHostDeployContext } from './send';

const requiredEnv = (name: string): string => {
  const value = process.env[name];
  if (!value) throw new Error(`missing required env ${name}`);
  return value;
};

const integerEnv = (name: string, fallback: number): number => {
  const raw = process.env[name];
  if (raw === undefined || raw === '') return fallback;
  const value = Number.parseInt(raw, 10);
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
    fallbackPath: `${process.env.HOME ?? '/home/fhevm'}/.config/solana/id.json`,
    writePath: path.join(KEYPAIR_DIR, 'deployer-keypair.json'),
  });

if (process.argv.includes('--help')) {
  console.log('usage: host deploy|upgrade; demos deploy|upgrade; coprocessor register');
  process.exit(0);
}

const profile = readSolanaProgramProfile();
const programIds = programIdsFor(profile);

const deploy = async (programs: readonly SolanaDeployProgram[], upgrade: boolean) => {
  const rpcUrl = requiredEnv('SOLANA_RPC_URL');
  return deployProgramArtifacts({
    rpcUrl,
    deployerKeypairPath: await resolveDeployerKeypairPath(),
    artifactsDir: ARTIFACTS_DIR,
    programKeypairPaths: await resolveProgramKeypairs(programs),
    programs,
    upgrade,
    profile,
  });
};

const bootstrap = async (validateOnly = false) => {
  const rpcUrl = requiredEnv('SOLANA_RPC_URL');
  const gateway = await readGatewayBootstrapInputsFromEnv();
  console.log(`    profile=${profile}`);
  console.log(`    zama_host=${programIds.zamaHost}`);
  console.log(`    gateway_chain_id=${gateway.gatewayChainId}`);
  console.log(`    input_verification=${evmHex(gateway.inputVerificationContract)}`);
  console.log(`    decryption=${evmHex(gateway.decryptionContract)}`);
  console.log(`    coprocessor_signers=${gateway.coprocessorSigners.map(evmHex).join(',')}`);
  console.log(`    kms_signers=${gateway.kmsSigners.map(evmHex).join(',')}`);
  const payer = await loadKeypairSigner(await resolveDeployerKeypairPath());
  await bootstrapZamaHost(createHostDeployContext(rpcUrl), {
    payer,
    gateway,
    coprocessorThreshold: integerEnv('COPROCESSOR_THRESHOLD', 1),
    kmsCorruptionThreshold: integerEnv('KMS_THRESHOLD', 0),
    programAddress: programIds.zamaHost,
    validateOnly,
  });
};

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
  if (target === 'host') await bootstrap(true);
  const ids = await deploy(programs, action === 'upgrade');
  if (target === 'host') await bootstrap();
  await writeSolanaAddressArtifact(ADDRESSES_DIR, ids);
  console.log(`${target} ${action} complete`);
}
