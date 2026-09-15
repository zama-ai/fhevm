// Client-side verification of kms-connector endpoint responses.
import { expect } from 'chai';
import { AbiCoder, Contract, getAddress, toBeHex, verifyTypedData } from 'ethers';
import { ethers } from 'hardhat';

import { kmsVerifierAddress } from '../../instance';
import {
  FHE_TYPE,
  type PostResult,
  type PublicDecryptionResponse,
  type UserDecryptionResponse,
  describeResult,
  fheTypeOf,
} from './connectorHttp';

////////////////////////////////////////////////////////////////////////////////
// KMSVerifier (host chain)
////////////////////////////////////////////////////////////////////////////////

const KMS_VERIFIER_ABI = [
  'function getKmsSigners() view returns (address[])',
  'function getThreshold() view returns (uint256)',
];

export interface KmsSignerSet {
  readonly signers: Set<string>;
  readonly threshold: number;
}

export async function readKmsSignerSet(): Promise<KmsSignerSet> {
  const verifier = new Contract(kmsVerifierAddress, KMS_VERIFIER_ABI, ethers.provider);
  const [signers, threshold] = await Promise.all([
    verifier.getKmsSigners() as Promise<string[]>,
    verifier.getThreshold() as Promise<bigint>,
  ]);
  return { signers: new Set(signers.map((s) => s.toLowerCase())), threshold: Number(threshold) };
}

////////////////////////////////////////////////////////////////////////////////
// Public decryption: EIP-712 `PublicDecryptVerification`
////////////////////////////////////////////////////////////////////////////////

/** Field order is authoritative — it determines the EIP-712 type hash (mirror of the Gateway `Decryption` struct). */
const PUBLIC_DECRYPT_TYPES = {
  PublicDecryptVerification: [
    { name: 'ctHandles', type: 'bytes32[]' },
    { name: 'decryptedResult', type: 'bytes' },
    { name: 'extraData', type: 'bytes' },
  ],
};

/** The KMS signs `PublicDecryptVerification` under the GATEWAY chain id and the gateway `Decryption` contract. */
const publicDecryptDomain = () => ({
  name: 'Decryption',
  version: '1',
  chainId: Number(process.env.CHAIN_ID_GATEWAY),
  verifyingContract: process.env.DECRYPTION_ADDRESS!,
});

/** Lowercased address that signed a party's public decryption response. */
export function recoverPublicDecryptSigner(ctHandles: string[], response: PublicDecryptionResponse): string {
  return verifyTypedData(
    publicDecryptDomain(),
    PUBLIC_DECRYPT_TYPES,
    { ctHandles, decryptedResult: response.decryptedResult, extraData: response.extraData },
    response.signature,
  ).toLowerCase();
}

export type ClearValue = boolean | bigint | string;

/**
 * Decodes `decryptedResult` (one ABI `uint256` word per handle, as published on
 * the Gateway) into typed clear values driven by each handle's FHE type byte.
 */
export function decodeDecryptedResult(ctHandles: string[], decryptedResult: string): ClearValue[] {
  const words = AbiCoder.defaultAbiCoder().decode(
    ctHandles.map(() => 'uint256'),
    decryptedResult,
  ) as unknown as bigint[];
  return ctHandles.map((handle, i) => {
    const word = BigInt(words[i]);
    switch (fheTypeOf(handle)) {
      case FHE_TYPE.ebool:
        return word !== 0n;
      case FHE_TYPE.eaddress:
        return getAddress(toBeHex(word, 20));
      default:
        return word;
    }
  });
}

export interface VerifiedPublicDecrypt {
  readonly decryptionId: string;
  readonly clearValues: ClearValue[];
  readonly signers: string[];
}

/**
 * Asserts a fan-out of public decryption responses is a valid quorum:
 * - every party answered `200` (unless `allowFailures`),
 * - every signature recovers to a registered KMS signer, all distinct,
 * - `decryptionId` and `decryptedResult` are byte-identical across parties,
 * - at least `max(KMSVerifier.getThreshold(), 2t+1)` distinct signers.
 */
export function verifyPublicDecryptQuorum(
  ctHandles: string[],
  results: PostResult<PublicDecryptionResponse>[],
  kms: KmsSignerSet,
  opts: { minSigners: number; allowFailures?: boolean },
): VerifiedPublicDecrypt {
  const ok = results.filter((r) => r.httpStatus === 200);
  if (!opts.allowFailures) {
    expect(ok.length, `parties answering 200:\n${results.map(describeResult).join('\n')}`).to.equal(results.length);
  }
  expect(ok.length, 'at least one party must answer 200').to.be.greaterThan(0);

  const first = ok[0].body as PublicDecryptionResponse;
  expect(first.decryptionId).to.match(/^0x[0-9a-f]{64}$/i);
  const signers = new Set<string>();
  for (const r of ok) {
    const body = r.body as PublicDecryptionResponse;
    expect(body.decryptionId, `decryptionId differs at ${r.url}`).to.equal(first.decryptionId);
    expect(body.decryptedResult, `decryptedResult differs at ${r.url}`).to.equal(first.decryptedResult);
    const signer = recoverPublicDecryptSigner(ctHandles, body);
    expect(kms.signers.has(signer), `${signer} (from ${r.url}) is not a registered KMS signer`).to.equal(true);
    expect(signers.has(signer), `${signer} signed twice (${r.url})`).to.equal(false);
    signers.add(signer);
  }
  const required = Math.max(kms.threshold, opts.minSigners);
  expect(signers.size, `distinct KMS signers below quorum ${required}`).to.be.at.least(required);

  return {
    decryptionId: first.decryptionId,
    clearValues: decodeDecryptedResult(ctHandles, first.decryptedResult),
    signers: [...signers],
  };
}

////////////////////////////////////////////////////////////////////////////////
// User decryption: structural checks only (share reconstruction is a follow-up)
////////////////////////////////////////////////////////////////////////////////

export function verifyUserDecryptResponses(results: PostResult<UserDecryptionResponse>[]): { decryptionId: string } {
  expect(results.length).to.be.greaterThan(0);
  for (const r of results) {
    expect(r.httpStatus, describeResult(r)).to.equal(200);
  }
  const first = results[0].body as UserDecryptionResponse;
  expect(first.decryptionId).to.match(/^0x[0-9a-f]{64}$/i);
  for (const r of results) {
    const body = r.body as UserDecryptionResponse;
    expect(body.decryptionId, `decryptionId differs at ${r.url}`).to.equal(first.decryptionId);
    expect(body.userDecryptedShares, `empty shares at ${r.url}`).to.match(/^0x[0-9a-f]{2,}$/i);
    expect(body.signature, `signature at ${r.url} is not 65 bytes`).to.match(/^0x[0-9a-f]{130}$/i);
  }
  return { decryptionId: first.decryptionId };
}
