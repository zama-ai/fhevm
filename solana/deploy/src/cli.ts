// Solana deployer: host deploy|upgrade|wipe, demos deploy|upgrade, coprocessor register.
import { spawnSync } from 'node:child_process';
import path from 'node:path';

import { writeSolanaAddressArtifact } from './artifact';
import { SOLANA_DEPLOY_PROGRAMS, type SolanaDeployProgram } from './constants';
import { registerSolanaCoprocessorSql } from './coprocessor';
import { deployHostProgram } from './deploy-host';
import { deployProgramArtifacts } from './deploy-programs';
import { integerEnv, readGatewayBootstrapInputsFromEnv, requiredEnv } from './gateway';
import { loadKeypairSigner, resolveKeypairPath } from './keypair';
import { programIdsFor, readSolanaEnvironment } from './environment';
import { createHostDeployContext } from './send';
import { wipeZamaHost } from './wipe';

const evmHex = (bytes: Uint8Array): string => `0x${Buffer.from(bytes).toString('hex')}`;

const USAGE = 'usage: host deploy [--allow-upgrade]|upgrade|wipe; demos deploy [--allow-upgrade]|upgrade; coprocessor register';
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
    // Only an explicit keypair (first deploy of a program on a cluster). The build writes a
    // throwaway <program>-keypair.json next to the .so, which never matches the shipped id.
    if (!process.env[envName] && !process.env[jsonName]) continue;
    paths[program] = await resolveKeypairPath({
      pathEnv: process.env[envName],
      jsonEnv: process.env[jsonName],
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
  console.log(USAGE);
  process.exit(0);
}

const main = async () => {
  const environment = readSolanaEnvironment();
  const programIds = programIdsFor(environment);

  const [target, action, ...flags] = process.argv.slice(2);
  // Disposable environments redeploy every branch head onto the same program ids, so their
  // deploy may upgrade in place; persistent ones keep deploy and upgrade separate.
  const allowUpgrade = flags.includes('--allow-upgrade');
  if (flags.some((flag) => flag !== '--allow-upgrade') || (allowUpgrade && action !== 'deploy')) {
    throw new Error(USAGE);
  }
  if (target === 'coprocessor' && action === 'register') {
    const result = spawnSync('psql', ['-X', '-d', requiredEnv('DATABASE_URL'), '--set', 'ON_ERROR_STOP=1'], {
      input: registerSolanaCoprocessorSql(programIds.zamaHost, requiredEnv('SOLANA_KEY_SOURCE_CHAIN_ID')),
      encoding: 'utf8',
    });
    if (result.error) throw new Error(`coprocessor registration failed: ${result.error.message}`);
    // psql reports RAISE EXCEPTION messages and connection failures on stderr.
    if (result.status !== 0) throw new Error(`coprocessor registration failed: ${result.stderr.trim()}`);
    await writeSolanaAddressArtifact(ADDRESSES_DIR, { zama_host: programIds.zamaHost });
  } else if (target === 'host' && action === 'wipe') {
    const context = createHostDeployContext(requiredEnv('SOLANA_RPC_URL'));
    const payer = await loadKeypairSigner(await resolveDeployerKeypairPath());
    const closed = await wipeZamaHost(context, { payer, programAddress: programIds.zamaHost });
    console.log(`environment=${environment}; host=${programIds.zamaHost}; swept ${closed} program-owned accounts; none remain`);
  } else {
    if ((target !== 'host' && target !== 'demos') || (action !== 'deploy' && action !== 'upgrade')) {
      throw new Error(USAGE);
    }
    const programs =
      target === 'host' ? (['zama_host'] as const) : SOLANA_DEPLOY_PROGRAMS.filter((p) => p !== 'zama_host');
    const programKeypairPaths = await resolveProgramKeypairs(programs);
    const parameters = {
      rpcUrl: requiredEnv('SOLANA_RPC_URL'),
      deployerKeypairPath: await resolveDeployerKeypairPath(),
      artifactsDir: ARTIFACTS_DIR,
      upgrade: action === 'upgrade',
      allowUpgrade,
      environment,
    };
    let ids;
    if (target === 'host') {
      const gateway = await readGatewayBootstrapInputsFromEnv();
      console.log(`environment=${environment}; host=${programIds.zamaHost}; gateway_chain_id=${gateway.gatewayChainId}`);
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
      ids = await deployProgramArtifacts({ ...parameters, programs, programKeypairPaths });
    }
    await writeSolanaAddressArtifact(ADDRESSES_DIR, ids);
    console.log(`${target} ${action} complete`);
  }
};
main().catch((error: unknown) => {
  // Transport and psql errors can carry credential-bearing RPC or database URLs.
  const message = error instanceof Error ? error.message : 'deployment failed';
  console.error(message.replace(/[a-z][a-z0-9+.-]*:\/\/[^\s"'<>]+/gi, '<URL>'));
  process.exitCode = 1;
});
