import { describe, expect, it } from 'vitest';
import { createSolanaRpc } from '@solana/kit';
import { asBytes32Hex, bytesToHex } from '../../core/base/bytes.js';
import { recoverAddress } from '../../core/base/sign.js';
import { toSolanaZkProof } from '../../core/coprocessor/SolanaZkProof-p.js';
import { setFhevmRuntimeConfig } from '../internal/config.js';
import { createFhevmCleartextEncryptClient } from './createFhevmCleartextEncryptClient.js';
import { ciphertextVerificationDigest, cleartextInputExtraData } from './inputAttestation.js';
import { SOLANA_CLEARTEXT_SIGNER_ADDRESSES, signAsCleartextParty } from './parties.js';

const fill = (length: number, byte: number) => new Uint8Array(length).fill(byte);
const chain = {
  id: 72057594037940281n,
  fhevm: {
    relayerUrl: 'https://relayer.example.test',
    programs: { host: { address: asBytes32Hex(`0x${'22'.repeat(32)}`) } },
  },
};
const identities = {
  contractAddress: asBytes32Hex(`0x${'05'.repeat(32)}`),
  userAddress: asBytes32Hex(`0x${'04'.repeat(32)}`),
};

setFhevmRuntimeConfig({});
const client = createFhevmCleartextEncryptClient({ chain, rpc: createSolanaRpc('http://127.0.0.1:1') });

describe('cleartext input attestation', () => {
  it('signs the digest the host verifies', () => {
    // The vector `zama_host::eip712::tests::recovers_coprocessor_input_signer` pins.
    expect(
      ciphertextVerificationDigest({
        gatewayChainId: 31337n,
        inputVerificationContract: fill(20, 0xcd),
        ctHandles: [fill(32, 3)],
        userAddress: fill(32, 4),
        contractAddress: fill(32, 5),
        contractChainId: 72057594037940281n,
        extraData: Uint8Array.of(0),
      }),
    ).toBe('0x6921062a83bd4a2174c90c298454b562aa5bedb4fc8b22417d74b89fd37c5aab');
  });

  it('carries each plaintext in the width of its type, in handle order', async () => {
    const proof = await client.generateZkProof({
      ...identities,
      values: [
        { type: 'bool', value: true },
        { type: 'uint8', value: 7 },
        { type: 'uint64', value: 400n },
        { type: 'uint128', value: 2n ** 127n + 5n },
      ],
    });
    expect(bytesToHex(cleartextInputExtraData(proof))).toBe(
      // c1 | bool | uint8 | uint64 | uint128
      '0xc1' + '01' + '07' + '0000000000000190' + '80000000000000000000000000000005',
    );
  });

  it('refuses inputs that exceed the extraData the host accepts', async () => {
    const values = (count: number) => Array.from({ length: count }, () => ({ type: 'uint128', value: 1n }));
    const fifteen = await client.generateZkProof({ ...identities, values: values(15) });
    expect(cleartextInputExtraData(fifteen)).toHaveLength(241);
    const sixteen = await client.generateZkProof({ ...identities, values: values(16) });
    expect(() => cleartextInputExtraData(sixteen)).toThrow('the host accepts 256');
  });

  it('refuses a proof that carries no plaintexts', () => {
    const proof = toSolanaZkProof({
      chainId: chain.id,
      aclContractAddress: chain.fhevm.programs.host.address,
      ...identities,
      ciphertextWithZkProof: new Uint8Array([1]),
      encryptionBits: [64],
    });
    expect(() => cleartextInputExtraData(proof)).toThrow('not built by the cleartext encrypt module');
  });
});

describe('cleartext parties', () => {
  const digest = bytesToHex(fill(32, 9));
  const registered = SOLANA_CLEARTEXT_SIGNER_ADDRESSES.coprocessor.map((address) => Buffer.from(address.slice(2), 'hex'));

  it('sign as the registered signer', () => {
    const [signature] = signAsCleartextParty('coprocessor', registered, 1, digest);
    expect(recoverAddress({ hash: digest, signature: signature! })).toBe(SOLANA_CLEARTEXT_SIGNER_ADDRESSES.coprocessor[0]);
  });

  it('refuse a signer set with a key they do not hold', () => {
    expect(() => signAsCleartextParty('coprocessor', [...registered, fill(20, 0xbb)], 1, digest)).toThrow(
      'is not a cleartext key',
    );
  });
});
