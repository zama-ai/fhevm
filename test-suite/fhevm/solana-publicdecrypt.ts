import {
  buildSolanaUserDecryptMmrProofExtraData,
  solanaProofBytesToHex,
  solanaProofHexToBytes,
  verifyPublicDecryptProof,
  type MmrProof,
} from '@fhevm/sdk/solana';

function reqEnv(name: string): string {
  const value = process.env[name];
  if (value === undefined || value === '') throw new Error(`missing env ${name}`);
  return value;
}

function hexCsvToBytes(csv: string): Uint8Array[] {
  return csv
    .split(',')
    .map((s) => s.trim())
    .filter((s) => s.length > 0)
    .map(solanaProofHexToBytes);
}

function relayerUrl(path: string): string {
  return `${reqEnv('PD_RELAYER_URL').replace(/\/+$/, '')}${path}`;
}

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function decryptedValueAsBigInt(value: string): bigint {
  return BigInt(value.startsWith('0x') ? value : `0x${value}`);
}

const handle = solanaProofHexToBytes(reqEnv('PD_HANDLE'));
const encryptedValueAccount = solanaProofHexToBytes(reqEnv('PD_MMR_ENCRYPTED_VALUE_ACCOUNT'));
const aclValueKey = solanaProofHexToBytes(reqEnv('PD_ACL_VALUE_KEY'));
const peaks = hexCsvToBytes(process.env.PD_MMR_PEAKS ?? '');
const leafCount = BigInt(reqEnv('PD_MMR_LEAF_COUNT'));
const proofSlot = BigInt(reqEnv('PD_MMR_PROOF_SLOT'));
const proof: MmrProof = {
  leafIndex: BigInt(reqEnv('PD_MMR_LEAF_INDEX')),
  siblings: hexCsvToBytes(process.env.PD_MMR_SIBLINGS ?? ''),
};
const mmrProofBytes = solanaProofHexToBytes(reqEnv('PD_MMR_PROOF_BYTES'));

if (mmrProofBytes[0] !== 0x02) {
  throw new Error('public-decrypt proof bytes must use mode 0x02');
}
if (!verifyPublicDecryptProof(encryptedValueAccount, peaks, leafCount, handle, proof)) {
  throw new Error('public-decrypt MMR proof failed client-side verification');
}

const extraData = solanaProofBytesToHex(
  buildSolanaUserDecryptMmrProofExtraData(
    solanaProofHexToBytes(reqEnv('PD_CONTEXT_ID')),
    aclValueKey,
    proofSlot,
    mmrProofBytes,
  ),
);

const post = await fetch(relayerUrl('/v2/public-decrypt'), {
  method: 'POST',
  headers: { 'content-type': 'application/json' },
  body: JSON.stringify({
    ciphertextHandles: [reqEnv('PD_HANDLE')],
    extraData,
  }),
});
const postBody = await post.json();
if (!post.ok) {
  throw new Error(`public-decrypt POST failed: ${JSON.stringify(postBody)}`);
}
const jobId = postBody?.result?.jobId;
if (typeof jobId !== 'string' || jobId.length === 0) {
  throw new Error(`public-decrypt POST response missing jobId: ${JSON.stringify(postBody)}`);
}

const maxPolls = Number(process.env.PD_TIMEOUT_POLLS ?? '50');
const pollMs = Number(process.env.PD_POLL_MS ?? '3000');
let finalBody: unknown;
for (let i = 1; i <= maxPolls; i++) {
  const response = await fetch(relayerUrl(`/v2/public-decrypt/${jobId}`));
  finalBody = await response.json();
  const status = (finalBody as { status?: unknown }).status;
  if (status === 'succeeded') break;
  if (status === 'failed') {
    throw new Error(`public-decrypt failed: ${JSON.stringify(finalBody)}`);
  }
  if (i === maxPolls) {
    throw new Error(`public-decrypt timed out: ${JSON.stringify(finalBody)}`);
  }
  await sleep(pollMs);
}

const result = (finalBody as { result?: { decryptedValue?: string } }).result;
if (result?.decryptedValue === undefined) {
  throw new Error(`public-decrypt response missing decryptedValue: ${JSON.stringify(finalBody)}`);
}
if (process.env.PD_EXPECTED !== undefined && process.env.PD_EXPECTED !== '') {
  const actual = decryptedValueAsBigInt(result.decryptedValue);
  const expected = BigInt(process.env.PD_EXPECTED);
  if (actual !== expected) {
    throw new Error(`public-decrypt cleartext ${actual} != expected ${expected}`);
  }
}

process.stdout.write(`${JSON.stringify(finalBody)}\n`);
