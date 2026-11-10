import type { EthereumModule } from '../../modules/ethereum/types.js';
import type { RelayerModule } from '../../modules/relayer/types.js';
import type { FhevmClientFrozenContext } from '../../types/fhevmClientFrozenContext-p.js';
import type { Address, BytesHex } from '../../types/primitives.js';
import type { Uint256BigInt } from '../../types/primitives.js';
import { describe, expect, it, vi } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../../ethers/internal/ethers-p.js';
import { addressToChecksummedAddress } from '../../base/address.js';
import { sepolia } from '../../chains/definitions/sepolia.js';
import { createKmsUserDecryptEip712V1 } from '../../kms/createKmsUserDecryptEip712V1.js';
import { createKmsExtraDataV1 } from '../../kms/kmsExtraData-p.js';
import { parseSignedDecryptionPermitV1 } from '../../kms/SignedDecryptionPermitV1-p.js';
import { toTransportKeyPair } from '../../kms/TransportKeyPair-p.js';
import { createCoreFhevm } from '../../runtime/CoreFhevm-p.js';
import { createFhevmRuntime } from '../../runtime/CoreFhevmRuntime-p.js';
import { assertPermitMatchesKeyPair } from './assertPermitMatchesKeyPair.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/utils-p/decrypt/assertPermitMatchesKeyPair.test.ts
//
// Genuine SDK objects, built offline: the transport key pairs from raw bytes (no
// TKMS WASM), the permit by parsing a v1 EIP-712 whose signature check is mocked
// (recoverTypedDataAddress returns the signer).
////////////////////////////////////////////////////////////////////////////////

const SIGNER = addressToChecksummedAddress(`0x${'de'.repeat(20)}` as Address);
const CONTRACT = addressToChecksummedAddress(`0x${'ab'.repeat(20)}` as Address);
const SIGNATURE = `0x${'11'.repeat(65)}` as BytesHex;

const PUBLIC_KEY_A = `0x${'aa'.repeat(32)}` as BytesHex;
const PUBLIC_KEY_B = `0x${'bb'.repeat(32)}` as BytesHex;

function makeContext() {
  const ethereum = { recoverTypedDataAddress: vi.fn(async () => SIGNER) } as unknown as EthereumModule;
  const runtime = createFhevmRuntime(PRIVATE_ETHERS_TOKEN, { ethereum, relayer: {} as RelayerModule, config: {} });
  return createCoreFhevm(PRIVATE_ETHERS_TOKEN, { chain: sepolia, client: {}, runtime });
}

const fhevmContext = {} as FhevmClientFrozenContext;

function makeTransportKeyPair(context: ReturnType<typeof makeContext>, publicKey: BytesHex) {
  return toTransportKeyPair(context, { value: { publicKey, privateKey: `0x${'01'.repeat(32)}` }, fhevmContext });
}

/** A genuine v1 permit, signed (mock) for `transportKeyPair`. */
async function makePermit(
  context: ReturnType<typeof makeContext>,
  transportKeyPair: ReturnType<typeof makeTransportKeyPair>,
) {
  const eip712 = createKmsUserDecryptEip712V1({
    verifyingContractAddressDecryption: sepolia.fhevm.gateway.contracts.decryption.address,
    chainId: sepolia.id,
    publicKey: transportKeyPair.publicKey,
    contractAddresses: [CONTRACT],
    startTimestamp: Math.floor(Date.now() / 1000),
    durationDays: 1,
    extraData: createKmsExtraDataV1({ kmsContextId: 1n as Uint256BigInt }),
  });
  return await parseSignedDecryptionPermitV1(context, {
    transportKeyPair,
    permit: { eip712, signature: SIGNATURE, signerAddress: SIGNER },
    fhevmContext,
  });
}

////////////////////////////////////////////////////////////////////////////////

describe('assertPermitMatchesKeyPair', () => {
  it('accepts the key pair the permit was signed for', async () => {
    const context = makeContext();
    const keyPairA = makeTransportKeyPair(context, PUBLIC_KEY_A);
    const permit = await makePermit(context, keyPairA);

    expect(() => assertPermitMatchesKeyPair(permit, keyPairA)).not.toThrow();
  });

  it('compares the public keys case-insensitively', async () => {
    const context = makeContext();
    const keyPairA = makeTransportKeyPair(context, PUBLIC_KEY_A);
    const permit = await makePermit(context, keyPairA);
    const keyPairAUpper = makeTransportKeyPair(context, `0x${'AA'.repeat(32)}` as BytesHex);

    expect(() => assertPermitMatchesKeyPair(permit, keyPairAUpper)).not.toThrow();
  });

  it('throws InvalidSignedPermitError for another key pair', async () => {
    const context = makeContext();
    const keyPairA = makeTransportKeyPair(context, PUBLIC_KEY_A);
    const keyPairB = makeTransportKeyPair(context, PUBLIC_KEY_B);
    const permit = await makePermit(context, keyPairA);

    expect(() => assertPermitMatchesKeyPair(permit, keyPairB)).toThrow(
      expect.objectContaining({
        name: 'InvalidSignedPermitError',
        message: expect.stringContaining(
          "The permit's publicKey does not match the TransportKeyPair's publicKey. " +
            'Ensure the permit was signed with the same key pair.',
        ),
      }),
    );
  });

  it('throws InvalidSignedPermitError for a value that is not an SDK permit', () => {
    const context = makeContext();
    const keyPairA = makeTransportKeyPair(context, PUBLIC_KEY_A);
    const lookalike = { version: 1, transportPublicKey: PUBLIC_KEY_A };

    expect(() => assertPermitMatchesKeyPair(lookalike as never, keyPairA)).toThrow(
      expect.objectContaining({ name: 'InvalidSignedPermitError' }),
    );
  });

  it('throws InvalidTypeError for a value that is not a TransportKeyPair', async () => {
    const context = makeContext();
    const keyPairA = makeTransportKeyPair(context, PUBLIC_KEY_A);
    const permit = await makePermit(context, keyPairA);

    expect(() => assertPermitMatchesKeyPair(permit, { publicKey: PUBLIC_KEY_A } as never)).toThrow(
      expect.objectContaining({ name: 'InvalidTypeError' }),
    );
  });
});
