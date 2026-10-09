// A decrypt step run in its own process. The SDK caches zama-host's KMS reads for 15 minutes per
// process, as the EVM SDK caches ProtocolConfig's, so a step that must see a KMS context switch
// starts from an empty cache, as each EVM decrypt of the kms-context-switch profile does through
// `runNamedE2e`. The request goes in on stdin; the result comes back on one marked stdout line.
import { address } from '@solana/kit';

import { certifiedPublicDecrypt, type FheVerticalConfig, type PublicDecryptOutcome, userDecryptExpect } from './fhe-vertical';

const RESULT_MARKER = 'fresh-decrypt-result ';

type WireConfig = Omit<FheVerticalConfig, 'chainId'> & { readonly chainId: string };

type FreshDecryptRequest =
  | {
      readonly kind: 'certify';
      readonly config: WireConfig;
      readonly encryptedStore: string;
      readonly handle: `0x${string}`;
    }
  | {
      readonly kind: 'userDecrypt';
      readonly config: WireConfig;
      readonly encryptedStore: string;
      readonly handle: `0x${string}`;
      readonly secretKey: `0x${string}`;
      readonly expected: string;
    };

const hex = (bytes: Uint8Array): `0x${string}` => `0x${Buffer.from(bytes).toString('hex')}`;
const wireConfig = (config: FheVerticalConfig): WireConfig => ({ ...config, chainId: config.chainId.toString() });

const runFresh = async (request: FreshDecryptRequest): Promise<unknown> => {
  const child = Bun.spawn([process.execPath, import.meta.path], { stdin: 'pipe', stdout: 'pipe', stderr: 'inherit' });
  child.stdin.write(JSON.stringify(request));
  await child.stdin.end();
  const [stdout, exitCode] = await Promise.all([new Response(child.stdout).text(), child.exited]);
  const lines = stdout.split('\n');
  for (const line of lines.filter((line) => line !== '' && !line.startsWith(RESULT_MARKER))) console.log(line);
  if (exitCode !== 0) throw new Error(`the fresh-process ${request.kind} exited with code ${exitCode}`);
  const result = lines.find((line) => line.startsWith(RESULT_MARKER));
  if (result === undefined) throw new Error(`the fresh-process ${request.kind} returned no result`);
  return JSON.parse(result.slice(RESULT_MARKER.length));
};

/** `certifiedPublicDecrypt` in a fresh process. */
export const certifiedPublicDecryptInFreshProcess = async (
  config: FheVerticalConfig,
  params: { readonly encryptedStore: string; readonly handle: Uint8Array },
): Promise<PublicDecryptOutcome> => {
  const { cleartext, certificate } = (await runFresh({
    kind: 'certify',
    config: wireConfig(config),
    encryptedStore: params.encryptedStore,
    handle: hex(params.handle),
  })) as { cleartext: string; certificate: PublicDecryptOutcome['certificate'] };
  return { cleartext: BigInt(cleartext), certificate };
};

/** `userDecryptExpect` in a fresh process. */
export const userDecryptExpectInFreshProcess = async (
  config: FheVerticalConfig,
  params: { readonly encryptedStore: string; readonly handle: Uint8Array; readonly secretKey: `0x${string}`; readonly expected: bigint },
): Promise<void> => {
  await runFresh({
    kind: 'userDecrypt',
    config: wireConfig(config),
    encryptedStore: params.encryptedStore,
    handle: hex(params.handle),
    secretKey: params.secretKey,
    expected: params.expected.toString(),
  });
};

const serve = async (request: FreshDecryptRequest): Promise<unknown> => {
  const config = { ...request.config, chainId: BigInt(request.config.chainId) };
  const handle = Buffer.from(request.handle.slice(2), 'hex');
  if (request.kind === 'certify') {
    const { cleartext, certificate } = await certifiedPublicDecrypt(config, {
      encryptedStore: address(request.encryptedStore),
      handle,
    });
    return { cleartext: cleartext.toString(), certificate };
  }
  await userDecryptExpect(config, {
    encryptedStore: address(request.encryptedStore),
    handle,
    secretKey: request.secretKey,
    expected: BigInt(request.expected),
  });
  return {};
};

if (import.meta.main) {
  const result = await serve(JSON.parse(await Bun.stdin.text()) as FreshDecryptRequest);
  console.log(`${RESULT_MARKER}${JSON.stringify(result)}`);
  process.exit(0);
}
