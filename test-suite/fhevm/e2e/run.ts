// Run cleanup even when a scenario process fails; preserve the scenario's failure status.
import { randomUUID } from 'node:crypto';
import { loadEnv } from './harness/loadEnv';
import { recoverPreview } from '../../../solana/deploy/src/recover';
import { createHostDeployContext } from '../../../solana/deploy/src/send';
import { loadKeypairSigner } from '../src/solana/provision';
import { recoveryDirectory } from '../src/solana/recovery';
import { DEFAULT_SOLANA_ENVIRONMENT } from '../../../solana/deploy/src/environment';

const env = loadEnv();
const runId = randomUUID();
process.env.SOLANA_RECOVERY_RUN_ID = runId;
let status = 1;
try {
  status = await Bun.spawn(['bun', 'test', ...(process.argv.length > 2 ? process.argv.slice(2) : ['e2e/scenarios'])], { stdin: 'inherit', stdout: 'inherit', stderr: 'inherit' }).exited;
} finally {
  if (env.network === 'devnet') {
    try {
      await recoverPreview(createHostDeployContext(env.rpcUrl), await loadKeypairSigner(env.roots.deployerKeypairPath), DEFAULT_SOLANA_ENVIRONMENT, recoveryDirectory(), false, env.roots.deployerKeypairPath, false, runId);
    } catch {
      console.error('Run recovery incomplete; signing keys retained. Retry recovery before resetting the preview.');
      if (status === 0) status = 1;
    }
  }
}
process.exitCode = status;
