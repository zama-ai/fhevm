import {
  type Address,
  getAddressDecoder,
  getAddressEncoder,
  getBase58Decoder,
  getProgramDerivedAddress,
} from '@solana/kit';
import {
  findHostConfigPda,
  findKmsContextPda,
  getHostConfigEncoder,
  getKmsContextEncoder,
} from '@fhevm/solana-zama-host';
import type { SolanaRpc } from '../../encryptedStore.js';
import { RelayerAbortError } from '../../../core/errors/RelayerAbortError.js';
// The permit-path actions, assembled onto the client.
//
// `signPermit` is the piece worth pinning end to end: it is the only writer of the permit's
// derived fields, and a mistake here is signed by a real wallet and refused by every verifier
// after it. The wallet below is the conforming one — it builds the envelope itself around the
// text it is handed — and the transport pair is the real TKMS module's, so the permit that
// comes out is exactly what production would mint.

import type { FhevmSolanaChain } from '../../../core/types/fhevmSolanaChain.js';
import { RelayerTimeoutError } from '../../../core/errors/RelayerTimeoutError.js';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { ed25519 } from '@noble/curves/ed25519.js';
import { asBytes32Hex, hexToBytes32 } from '../../../core/base/bytes.js';
import { PERMIT_TRANSPORT_KEY_LEN, SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE } from '../../permit/index.js';
import { compileSolanaPermitEnvelope } from '../../permit/envelope.js';
import { createFhevmDecryptClient } from '../createFhevmDecryptClient.js';
import * as responseVerification from '../../userDecrypt/response.js';
import * as hostKms from '../../actions/hostKms.js';
import { setFhevmRuntimeConfig } from '../../internal/config.js';
import { getSolanaRuntime } from '../../internal/runtime.js';
import { initTkmsModule } from '../../../core/modules/decrypt/module/init-p.js';

////////////////////////////////////////////////////////////////////////////////

const PROGRAM_ID = asBytes32Hex(`0x${'22'.repeat(32)}`);
const CONTEXT_ID = asBytes32Hex(`0x${'33'.repeat(32)}`);
const EPOCH_ID = asBytes32Hex(`0x${'44'.repeat(32)}`);
const APP_PROGRAM = getAddressDecoder().decode(new Uint8Array(32).fill(0x01));
const MINT_A = getAddressDecoder().decode(new Uint8Array(32).fill(0x0a));
const MINT_B = getAddressDecoder().decode(new Uint8Array(32).fill(0x0b));

const chain = {
  id: 72057594037940281n,
  fhevm: {
    relayerUrl: 'http://relayer.local',
    programs: { host: { address: PROGRAM_ID } },
  },
} as const satisfies FhevmSolanaChain;

// The host's KMS trust, as the client reads it: `HostConfig` names the active context and epoch,
// and that context's `KmsContext` lists its signers.
const HOST_PROGRAM = getAddressDecoder().decode(hexToBytes32(PROGRAM_ID));
const [HOST_CONFIG_ADDRESS, HOST_CONFIG_BUMP] = await findHostConfigPda({ programAddress: HOST_PROGRAM });
const [KMS_CONTEXT_ADDRESS, KMS_CONTEXT_BUMP] = await findKmsContextPda(
  { contextId: hexToBytes32(CONTEXT_ID) },
  { programAddress: HOST_PROGRAM },
);
const hostAccount = (data: Uint8Array) => ({
  data: [Buffer.from(data).toString('base64'), 'base64'],
  owner: HOST_PROGRAM,
  executable: false,
  lamports: 1n,
  space: BigInt(data.length),
});
/** `HostConfig` with `currentKmsContextId` as the active context. */
const hostConfigAccountFor = (currentKmsContextId: Uint8Array) =>
  hostAccount(
    new Uint8Array(
      getHostConfigEncoder().encode({
        admin: HOST_PROGRAM,
        chainId: chain.id,
        gatewayChainId: 31337n,
        inputVerificationContract: new Uint8Array(20),
        coprocessorSigners: Array.from({ length: 8 }, () => new Uint8Array(20)),
        coprocessorSignerCount: 1,
        coprocessorThreshold: 1,
        decryptionContract: new Uint8Array(20).fill(0x42),
        currentKmsContextId,
        currentKmsEpochId: hexToBytes32(EPOCH_ID),
        paused: { execution: false, verifiedInputs: false, aclWrites: false },
        grantDenyListEnabled: false,
        maxHcuPerTx: 1n,
        maxHcuDepthPerTx: 1n,
        hcuBlockCapPerApp: 1n,
        bump: HOST_CONFIG_BUMP,
      }),
    ),
  );
const hostConfigAccount = hostConfigAccountFor(hexToBytes32(CONTEXT_ID));
const kmsContextAccount = hostAccount(
  new Uint8Array(
    getKmsContextEncoder().encode({
      contextId: hexToBytes32(CONTEXT_ID),
      signers: [new Uint8Array(20).fill(0x01)],
      thresholds: { publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 1 },
      destroyed: false,
      bump: KMS_CONTEXT_BUMP,
    }),
  ),
);

const USER_SEED = new Uint8Array(32).fill(0x07);
const USER_PUBKEY = ed25519.getPublicKey(USER_SEED);
/** The full Wallet Standard account the conforming wallet below selects. */
const USER_ACCOUNT = {
  address: getBase58Decoder().decode(USER_PUBKEY),
  publicKey: USER_PUBKEY,
  chains: ['solana:localnet'],
  features: [SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE],
} as const;

/**
 * The conforming wallet, in the official feature shape: for each handed text it wraps the content
 * in its own envelope, signs it, and returns one result carrying the signed bytes verbatim.
 */
function conformingWallet() {
  const signOffchainMessage = vi.fn((...inputs: readonly { readonly message: string }[]) =>
    Promise.resolve(
      inputs.map(({ message }) => {
        const envelope = compileSolanaPermitEnvelope(USER_PUBKEY, message);
        return { signedOffchainMessage: envelope, signature: ed25519.sign(envelope, USER_SEED) };
      }),
    ),
  );
  return {
    signOffchainMessage,
    wallet: {
      account: USER_ACCOUNT,
      features: {
        [SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE]: { supportedMessageVersions: [1], signOffchainMessage },
      },
    },
  };
}

// No permit-invalidation record: every account but the host's KMS trust is absent.
const hostAccounts = (config: ReturnType<typeof hostAccount>) => (address: Address) =>
  address === HOST_CONFIG_ADDRESS ? config : address === KMS_CONTEXT_ADDRESS ? kmsContextAccount : null;
const servingHostConfig = (config: ReturnType<typeof hostAccount>) => ({
  getAccountInfo: (address: Address) => ({ send: async () => ({ value: hostAccounts(config)(address) }) }),
  getMultipleAccounts: (addresses: readonly Address[]) => ({
    send: async () => ({ value: addresses.map(hostAccounts(config)) }),
  }),
});
const rpc = {
  getAccountInfo: vi.fn(servingHostConfig(hostConfigAccount).getAccountInfo),
  getMultipleAccounts: vi.fn(servingHostConfig(hostConfigAccount).getMultipleAccounts),
} as unknown as SolanaRpc;

function client() {
  setFhevmRuntimeConfig({});
  return createFhevmDecryptClient({ rpc, chain, fheParameter: 'test' });
}

const scopeBytes = (program: Address, scope: Address): Uint8Array => {
  const bytes = new Uint8Array(64);
  bytes.set(getAddressEncoder().encode(program), 0);
  bytes.set(getAddressEncoder().encode(scope), 32);
  return bytes;
};

afterEach(() => {
  vi.useRealTimers();
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
  hostKms.clearSolanaHostKmsReads();
});

////////////////////////////////////////////////////////////////////////////////

describe('signing a permit through the client', () => {
  it('mints the fields the host and chain pin, and one wallet prompt signs them', async () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date('2026-08-18T12:34:56Z'));
    const { wallet, signOffchainMessage } = conformingWallet();

    const session = await client().signPermit({
      wallet,
      durationSeconds: 604_800n,
      allowedScopes: [{ program: APP_PROGRAM, scope: MINT_A }],
    });

    const fields = session.signedPermit.fields;
    // The start norm: rounded down to the minute, and 12:34:56 rounds to 12:34:00.
    expect(fields.startTimestamp).toBe(BigInt(Math.floor(Date.parse('2026-08-18T12:34:00Z') / 1000)));
    expect(fields.durationSeconds).toBe(604_800n);
    expect(fields.chainId).toBe(chain.id);
    expect(fields.verifyingProgramId).toEqual(hexToBytes32(PROGRAM_ID));
    expect(fields.allowedScopes).toEqual([scopeBytes(APP_PROGRAM, MINT_A)]);
    expect(fields.kmsRouting.kmsContextId).toEqual(hexToBytes32(CONTEXT_ID));
    expect(fields.kmsRouting.kmsEpochId).toEqual(hexToBytes32(EPOCH_ID));
    expect(fields.userAddress).toEqual(USER_PUBKEY);

    // The permit commits to the real TKMS module's transport key, generated for this session.
    expect(fields.transportKey).toEqual(session.keyPair.publicKeyBytes);
    expect(fields.transportKey).toHaveLength(PERMIT_TRANSPORT_KEY_LEN);
    // Both halves are WASM keys of that same pair: a key with no WASM behind it throws here.
    const kmsLib = await initTkmsModule(getSolanaRuntime());
    expect(kmsLib.ml_kem_pke_pk_to_u8vec(session.keyPair.publicKey)).toEqual(fields.transportKey);
    expect(kmsLib.ml_kem_pke_pk_to_u8vec(kmsLib.ml_kem_pke_get_pk(session.keyPair.secretKey))).toEqual(
      fields.transportKey,
    );

    expect(signOffchainMessage).toHaveBeenCalledTimes(1);
    expect(session.warnings).toEqual([]);
  });

  // The permit signs its scopes in byte order; the caller lists them in whichever order it thinks in.
  it('signs the scopes sorted by bytes, whatever order the caller gave', async () => {
    const { wallet } = conformingWallet();

    const session = await client().signPermit({
      wallet,
      durationSeconds: 3_600n,
      allowedScopes: [
        { program: APP_PROGRAM, scope: MINT_B },
        { program: APP_PROGRAM, scope: MINT_A },
      ],
    });

    expect(session.signedPermit.fields.allowedScopes).toEqual([
      scopeBytes(APP_PROGRAM, MINT_A),
      scopeBytes(APP_PROGRAM, MINT_B),
    ]);
  });

  it.each([
    ['a donated PDA', '11111111111111111111111111111111', 0, true],
    ['a System account with data', '11111111111111111111111111111111', 1, false],
    ['an empty initialized host account', getBase58Decoder().decode(hexToBytes32(PROGRAM_ID)), 0, false],
  ] as const)('handles %s according to host initialization rules', async (_label, owner, size, valid) => {
    const { wallet } = conformingWallet();
    vi.spyOn(rpc, 'getAccountInfo').mockReturnValueOnce({
      send: async () => ({
        value: {
          data: [Buffer.alloc(size).toString('base64'), 'base64'],
          owner,
          executable: false,
          lamports: 1n,
          space: BigInt(size),
        },
      }),
    } as never);
    const result = client().signPermit({ wallet, durationSeconds: 3_600n });
    if (valid) await expect(result).resolves.toHaveProperty('signedPermit');
    else await expect(result).rejects.toThrow('Invalid permit invalidation');
  });

  it('starts no earlier than the watermark', async () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date('2026-08-18T12:34:56Z'));
    const watermark = BigInt(Math.floor(Date.parse('2026-08-18T13:00:00Z') / 1000));
    const { wallet } = conformingWallet();

    const [pda, bump] = await getProgramDerivedAddress({
      programAddress: getAddressDecoder().decode(hexToBytes32(PROGRAM_ID)),
      seeds: [new TextEncoder().encode('permit-invalidation'), USER_PUBKEY],
    });
    const data = new Uint8Array(49);
    data.set([0xec, 0x8b, 0xdb, 0xa9, 0xb9, 0x22, 0xe9, 0x88]);
    data.set(USER_PUBKEY, 8);
    new DataView(data.buffer).setBigUint64(40, watermark, true);
    data[48] = bump;
    const read = vi.spyOn(rpc, 'getAccountInfo').mockReturnValueOnce({
      send: async () => ({
        value: {
          data: [Buffer.from(data).toString('base64'), 'base64'],
          owner: getBase58Decoder().decode(hexToBytes32(PROGRAM_ID)),
          executable: false,
          lamports: 1n,
          space: 49n,
        },
      }),
    } as never);
    const session = await client().signPermit({ wallet, durationSeconds: 3_600n });
    expect(read).toHaveBeenCalledWith(pda, expect.anything());

    expect(session.signedPermit.fields.startTimestamp).toBe(watermark);
  });

  it('is permissive when no scope is named, warns when that outlives a week, and still signs it', async () => {
    const { wallet } = conformingWallet();

    const session = await client().signPermit({ wallet, durationSeconds: 604_801n });

    expect(session.signedPermit.fields.allowedScopes).toEqual([]);
    expect(session.warnings.map((warning) => warning.code)).toEqual(['PermissiveLongWindow']);
    expect(session.signedPermit.signature).toHaveLength(64);
  });
});

////////////////////////////////////////////////////////////////////////////////

describe('running a user decryption through the client', () => {
  // What this pins is the client's routing of entry keys — the one derived field of a delegated
  // request: an explicit key travels as given (the delegator), an omitted one defaults to the
  // permit's own user. Everything below the client is stubbed at the network seam; the relayer
  // refuses the request so the run ends after the submission whose body the test reads.
  // A well-formed handle: bytes 22..30 embed the host chain id big-endian, byte 30 is the FHE
  // type (5 = euint64), byte 31 the handle version.
  const HANDLE = new Uint8Array(32).fill(0xab);
  HANDLE.set([0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x39], 22); // 72057594037940281
  HANDLE[30] = 5;
  HANDLE[31] = 0;
  const ENCRYPTED_VALUE_ACCOUNT = new Uint8Array(32).fill(0xcd);
  const DELEGATOR = new Uint8Array(32).fill(0x66);

  const hex = (bytes: Uint8Array) =>
    `0x${Array.from(bytes)
      .map((byte) => byte.toString(16).padStart(2, '0'))
      .join('')}`;

  // Deriving the KMS context PDA hashes off the main thread, which fake timers do not drive; the tests
  // that advance them pin the deadline, not the host read.
  const trustHostVerification = () =>
    vi.spyOn(hostKms, 'userDecryptVerification').mockResolvedValue({
      signers: [{ partyId: 1, address: '0x0101010101010101010101010101010101010101' }],
      fheParameter: 'test',
      gatewayEip712Domain: {
        name: 'Decryption',
        version: '1',
        chainId: 31337n,
        verifyingContract: '0x4242424242424242424242424242424242424242',
      },
    });

  function jsonResponse(body: unknown, status = 200): Response {
    return new Response(JSON.stringify(body), {
      status,
      headers: { 'content-type': 'application/json', 'Retry-After': '0' },
    });
  }

  it('sends the delegated key as given and defaults the direct one to the permit user', async () => {
    const { wallet } = conformingWallet();
    const decryptClient = client();
    const session = await decryptClient.signPermit({ wallet, durationSeconds: 3_600n });

    let capturedBody:
      | { attestedPayload: { handles: readonly { ownerAddress: string; encryptedStore: string }[] } }
      | undefined;
    vi.stubGlobal(
      'fetch',
      vi.fn(async (input: RequestInfo | URL, init?: RequestInit) => {
        const url = String(input instanceof Request ? input.url : input);
        if (url.startsWith('http://relayer.local')) {
          capturedBody = JSON.parse(String(init?.body));
          return jsonResponse(
            {
              status: 'failed',
              error: {
                label: 'validation_failed',
                message: 'refused by the test relayer',
                details: [{ field: 'handles', issue: 'refused by the test relayer' }],
              },
            },
            400,
          );
        }
        throw new Error(`unexpected fetch to ${url}`);
      }),
    );

    await expect(
      decryptClient.decryptValues({
        session,
        entries: [
          { handle: HANDLE, encryptedStore: ENCRYPTED_VALUE_ACCOUNT, ownerAddress: DELEGATOR },
          { handle: HANDLE, encryptedStore: ENCRYPTED_VALUE_ACCOUNT },
        ],
        attempts: 1,
      }),
    ).rejects.toThrow('refused');

    expect(capturedBody?.attestedPayload.handles.map((entry) => entry.ownerAddress)).toEqual([
      hex(DELEGATOR),
      hex(USER_PUBKEY),
    ]);
    expect(capturedBody?.attestedPayload.handles.map((entry) => entry.encryptedStore)).toEqual([
      hex(ENCRYPTED_VALUE_ACCOUNT),
      hex(ENCRYPTED_VALUE_ACCOUNT),
    ]);
  });
  // A permit stays answerable after the host switches context: its response is verified against
  // the context the permit names, not the one HostConfig now holds. The cache is emptied after
  // signing, so the decryption reads the switched HostConfig.
  it('verifies a permit against its own KMS context after the host switches to another', async () => {
    const { wallet } = conformingWallet();
    const session = await client().signPermit({ wallet, durationSeconds: 3_600n });
    hostKms.clearSolanaHostKmsReads();
    const switched = servingHostConfig(hostConfigAccountFor(new Uint8Array(32).fill(0x55)));
    vi.spyOn(rpc, 'getAccountInfo').mockImplementation(switched.getAccountInfo as never);
    const hostRead = vi.spyOn(rpc, 'getMultipleAccounts').mockImplementation(switched.getMultipleAccounts as never);
    const relayer = vi.fn(async () =>
      jsonResponse(
        {
          status: 'failed',
          error: {
            label: 'validation_failed',
            message: 'refused by the test relayer',
            details: [{ field: 'handles', issue: 'refused by the test relayer' }],
          },
        },
        400,
      ),
    );
    vi.stubGlobal('fetch', relayer);

    await expect(
      client().decryptValues({
        session,
        entries: [{ handle: HANDLE, encryptedStore: ENCRYPTED_VALUE_ACCOUNT }],
        attempts: 1,
      }),
    ).rejects.toThrow('refused');

    expect(hostRead).toHaveBeenCalledWith([HOST_CONFIG_ADDRESS, KMS_CONTEXT_ADDRESS], expect.anything());
    expect(relayer).toHaveBeenCalled();
  });

  it.each(['timeout', 'abort'])('does not return plaintext after %s during verification', async (cause) => {
    const { wallet } = conformingWallet();
    const decryptClient = client();
    const session = await decryptClient.signPermit({ wallet, durationSeconds: 3_600n });
    let release!: () => void;
    const pending = new Promise<void>((resolve) => {
      release = resolve;
    });
    const verify = vi.spyOn(responseVerification, 'verifySolanaUserDecryptResponse').mockImplementation(async () => {
      await pending;
      return [];
    });
    trustHostVerification();
    vi.useFakeTimers();
    vi.stubGlobal(
      'fetch',
      vi
        .fn()
        .mockResolvedValueOnce(jsonResponse({ status: 'queued', requestId: 'req-1', result: { jobId: 'job-1' } }, 202))
        .mockResolvedValueOnce(
          jsonResponse({
            status: 'succeeded',
            requestId: 'req-1',
            result: {
              result: [{ payload: 'deadbeef', signature: 'ab'.repeat(65), extraData: '0x01' }],
            },
          }),
        ),
    );
    const controller = new AbortController();
    const onProgress = vi.fn();
    const result = decryptClient.decryptValues({
      session,
      entries: [{ handle: HANDLE, encryptedStore: ENCRYPTED_VALUE_ACCOUNT }],
      options: { timeout: 1500, signal: controller.signal, onProgress },
    });
    const rejection =
      cause === 'timeout'
        ? expect(result).rejects.toBeInstanceOf(RelayerTimeoutError)
        : expect(result).rejects.toBeInstanceOf(RelayerAbortError);
    await vi.advanceTimersByTimeAsync(1001);
    expect(verify).toHaveBeenCalledOnce();
    if (cause === 'timeout') {
      await vi.advanceTimersByTimeAsync(500);
    } else {
      controller.abort();
    }
    release();
    await rejection;
    await vi.runAllTicks();
    if (cause === 'abort') expect(onProgress.mock.calls.filter(([event]) => event.type === 'abort')).toHaveLength(1);
  });

  it('aborts during retry backoff without another submission or wallet prompt', async () => {
    const { wallet, signOffchainMessage } = conformingWallet();
    const decryptClient = client();
    const session = await decryptClient.signPermit({ wallet, durationSeconds: 3_600n });
    trustHostVerification();
    vi.useFakeTimers();
    const fetch = vi
      .fn()
      .mockResolvedValueOnce(jsonResponse({ status: 'queued', requestId: 'req-1', result: { jobId: 'job-1' } }, 202))
      .mockResolvedValueOnce(jsonResponse({ status: 'succeeded', requestId: 'req-1', result: { result: [] } }));
    vi.stubGlobal('fetch', fetch);
    const controller = new AbortController();
    const onProgress = vi.fn();
    const result = decryptClient.decryptValues({
      session,
      entries: [{ handle: HANDLE, encryptedStore: ENCRYPTED_VALUE_ACCOUNT }],
      options: { signal: controller.signal, onProgress },
    });
    const rejection = expect(result).rejects.toBeInstanceOf(RelayerAbortError);
    await vi.advanceTimersByTimeAsync(1_001);
    expect(fetch).toHaveBeenCalledTimes(2);
    controller.abort();
    await rejection;
    await vi.advanceTimersByTimeAsync(10_000);
    expect(onProgress.mock.calls.filter(([event]) => event.type === 'abort')).toHaveLength(1);
    expect(fetch).toHaveBeenCalledTimes(2);
    expect(signOffchainMessage).toHaveBeenCalledTimes(1);
  });
  it.each(['backoff', 'polling'])(
    'expires the operation deadline during %s with consistent progress',
    async (phase) => {
      const { wallet, signOffchainMessage } = conformingWallet();
      const decryptClient = client();
      const session = await decryptClient.signPermit({ wallet, durationSeconds: 3_600n });
      trustHostVerification();
      vi.useFakeTimers();
      const fetch = vi
        .fn()
        .mockResolvedValueOnce(jsonResponse({ status: 'queued', requestId: 'req-1', result: { jobId: 'job-1' } }, 202))
        .mockResolvedValueOnce(
          phase === 'backoff'
            ? jsonResponse({ status: 'succeeded', requestId: 'req-1', result: { result: [] } })
            : jsonResponse({ status: 'queued', requestId: 'req-1' }, 202),
        );
      vi.stubGlobal('fetch', fetch);
      const onProgress = vi.fn();
      const result = decryptClient.decryptValues({
        session,
        entries: [{ handle: HANDLE, encryptedStore: ENCRYPTED_VALUE_ACCOUNT }],
        options: { timeout: 1500, onProgress },
      });
      const rejection = expect(result).rejects.toBeInstanceOf(RelayerTimeoutError);
      await vi.advanceTimersByTimeAsync(1_001);
      expect(fetch).toHaveBeenCalledTimes(2);
      await vi.advanceTimersByTimeAsync(500);
      await rejection;
      const timeoutEvents = onProgress.mock.calls.filter(([event]) => event.type === 'timeout');
      expect(timeoutEvents).toHaveLength(1);
      if (phase === 'backoff') {
        expect(timeoutEvents[0]![0]).not.toHaveProperty('jobId');
        expect(timeoutEvents[0]![0]).not.toHaveProperty('method');
      }
      expect(onProgress.mock.calls.some(([event]) => event.type === 'abort')).toBe(false);
      await vi.advanceTimersByTimeAsync(10_000);
      expect(fetch).toHaveBeenCalledTimes(2);
      expect(signOffchainMessage).toHaveBeenCalledTimes(1);
    },
  );
});
