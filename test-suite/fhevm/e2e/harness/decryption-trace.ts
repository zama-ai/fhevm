// Preloaded into every scenario by e2e/run.ts: prints where each refused decryption went. Every v4
// decryption POST answered with an error gets its request id, the decryption id relayer-http derived,
// each KMS node's outcome and the Connectors' lines for that id (src/solana/decryption-trace.ts).
// Only the local stack's container logs are readable; elsewhere the client's view is printed alone.
import { kmsPartyIds } from '../../src/kms-party';
import { describeDecryptionCall, interceptDecryptionCalls } from '../../src/solana/decryption-trace';
import { readStackState } from '../../src/solana/deploy';
import { loadEnv } from './loadEnv';

const env = loadEnv();
const parties = env.source === 'local' ? kmsPartyIds((await readStackState()).scenario.kms.parties) : undefined;

interceptDecryptionCalls(async (call) => {
  if (call.status === 200) return;
  if (parties === undefined) {
    console.log(`[decryption] ${call.route} -> ${call.status} ${call.code ?? ''} request_id=${call.requestId}`);
    return;
  }
  try {
    console.log(await describeDecryptionCall(call, parties));
  } catch (error) {
    console.log(`[decryption] request ${call.requestId}: its logs could not be read: ${String(error)}`);
  }
});
