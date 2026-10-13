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

// The trace prints once the logs are read; the scenario's fetch does not wait for it.
interceptDecryptionCalls((call) => {
  if (call.status === 200) return;
  if (parties === undefined) {
    console.log(`[decryption] ${call.route} -> ${call.status} ${call.code ?? ''} request_id=${call.requestId ?? 'none'}`);
    return;
  }
  describeDecryptionCall(call, parties).then(
    (trace) => console.log(trace),
    (error: unknown) => console.log(`[decryption] request ${call.requestId}: its logs could not be read: ${String(error)}`),
  );
});
