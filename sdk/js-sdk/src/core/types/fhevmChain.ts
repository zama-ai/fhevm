import type { ChecksummedAddress, Uint64BigInt } from './primitives.js';

export type FhevmChain = {
  readonly id: number;
} & FhevmChainExtension;

/**
 * {
 *   id: 1,
 *   fhevm: {
 *     contracts: {
 *       acl: "0xdeadbeef..."
 *       inputVerifier: "0xdeadbeef..."
 *       kmsVerifier: "0xdeadbeef..."
 *     },
 *     relayerUrl: "http://foo",
 *     gateway?: {
 *       id: 1234,
 *       contracts: {
 *         decryption: ChecksummedAddress;
 *         inputVerification: ChecksummedAddress;
 *       }
 *     }
 *   }
 * }
 */

export type FhevmChainExtension = {
  readonly fhevm: {
    readonly contracts: FhevmChainContracts;
    readonly relayerUrl: string;
    readonly gateway: FhevmGatewayChain;
  };
};

type ChainContract = {
  readonly address: `0x${string}`;
  readonly blockCreated?: number | undefined;
};

export type FhevmChainContracts = {
  readonly acl: ChainContract;
  readonly inputVerifier: ChainContract;
  readonly kmsVerifier: ChainContract;
};

export type FhevmGatewayChain = {
  readonly id: number;
  readonly contracts: {
    readonly decryption: ChainContract;
    readonly inputVerification: ChainContract;
  };
};

////////////////////////////////////////////////////////////////////////////////

export type ResolvedFhevmChain = {
  readonly id: Uint64BigInt;
} & ResolvedFhevmChainExtension;

type ResolvedFhevmChainExtension = {
  readonly fhevm: {
    readonly contracts: ResolvedFhevmChainContracts;
    readonly relayerUrl: string;
    readonly gateway: ResolvedFhevmGatewayChain;
  };
};

type ResolvedChainContract = {
  readonly address: ChecksummedAddress;
  readonly blockCreated?: number | undefined;
};

type ResolvedFhevmChainContracts = {
  readonly acl: ResolvedChainContract;
  readonly inputVerifier: ResolvedChainContract;
  readonly kmsVerifier: ResolvedChainContract;
};

type ResolvedFhevmGatewayChain = {
  readonly id: Uint64BigInt;
  readonly contracts: {
    readonly decryption: ResolvedChainContract;
    readonly inputVerification: ResolvedChainContract;
  };
};
