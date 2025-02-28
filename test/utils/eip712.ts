import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { ethers } from "hardhat";

export interface EIP712Type {
  name: string;
  type: string;
}

export interface EIP712 {
  domain: {
    chainId: number;
    name: string;
    verifyingContract: string;
    version: string;
  };
  message: {
    [key: string]: string | string[] | number | number[] | Uint8Array | Uint8Array[];
  };
  primaryType: string;
  types: {
    [key: string]: EIP712Type[];
  };
}

// Create an EIP712 message for a ZKPoK response
export function createEIP712ResponseZKPoK(
  chainId: number,
  verifyingContract: string,
  handles: Uint8Array[],
  userAddress: string,
  contractAddress: string,
  contractChainId: number,
): EIP712 {
  if (!ethers.isAddress(verifyingContract)) {
    throw new Error("Invalid verifying contract address.");
  }
  return {
    types: {
      EIP712Domain: [
        { name: "name", type: "string" },
        { name: "version", type: "string" },
        { name: "chainId", type: "uint256" },
        { name: "verifyingContract", type: "address" },
      ],
      EIP712ZKPoK: [
        { name: "handles", type: "bytes32[]" },
        { name: "userAddress", type: "address" },
        { name: "contractAddress", type: "address" },
        { name: "contractChainId", type: "uint256" },
      ],
    },
    primaryType: "EIP712ZKPoK",
    domain: {
      name: "ZKPoKManager",
      version: "1",
      chainId,
      verifyingContract,
    },
    message: {
      handles,
      userAddress,
      contractAddress,
      contractChainId,
    },
  };
}

// Get signatures from signers using the EIP712 message response for proof verification
export async function getSignaturesZKPoK(eip712: EIP712, signers: HardhatEthersSigner[]): Promise<string[]> {
  return Promise.all(
    signers.map((signer) =>
      signer.signTypedData(eip712.domain, { EIP712ZKPoK: eip712.types.EIP712ZKPoK }, eip712.message),
    ),
  );
}

// Create an EIP712 message for a public decryption response
export function createEIP712ResponsePublicDecrypt(
  chainId: number,
  verifyingContract: string,
  ctHandles: number[],
  decryptedResult: Uint8Array,
): EIP712 {
  if (!ethers.isAddress(verifyingContract)) {
    throw new Error("Invalid verifying contract address.");
  }
  return {
    types: {
      EIP712Domain: [
        { name: "name", type: "string" },
        { name: "version", type: "string" },
        { name: "chainId", type: "uint256" },
        { name: "verifyingContract", type: "address" },
      ],
      EIP712PublicDecrypt: [
        { name: "ctHandles", type: "uint256[]" },
        { name: "decryptedResult", type: "bytes" },
      ],
    },
    primaryType: "EIP712PublicDecrypt",
    domain: {
      name: "DecryptionManager",
      version: "1",
      chainId,
      verifyingContract,
    },
    message: {
      ctHandles: ctHandles,
      decryptedResult: decryptedResult,
    },
  };
}

// Get signatures from signers using the EIP712 message response for public decryption
export async function getSignaturesPublicDecrypt(eip712: EIP712, signers: HardhatEthersSigner[]): Promise<string[]> {
  return Promise.all(
    signers.map((signer) =>
      signer.signTypedData(eip712.domain, { EIP712PublicDecrypt: eip712.types.EIP712PublicDecrypt }, eip712.message),
    ),
  );
}

// Create an EIP712 message for a user decryption request
export function createEIP712RequestUserDecrypt(
  chainId: number,
  verifyingContract: string,
  publicKey: Uint8Array,
  contractAddresses: string[],
  contractsChainId: number,
  startTimestamp: string,
  durationDays: string,
): EIP712 {
  if (!ethers.isAddress(verifyingContract)) {
    throw new Error("Invalid verifying contract address.");
  }
  return {
    types: {
      EIP712Domain: [
        { name: "name", type: "string" },
        { name: "version", type: "string" },
        { name: "chainId", type: "uint256" },
        { name: "verifyingContract", type: "address" },
      ],
      EIP712UserDecryptRequest: [
        { name: "publicKey", type: "bytes" },
        { name: "contractAddresses", type: "address[]" },
        { name: "contractsChainId", type: "uint256" },
        { name: "startTimestamp", type: "uint256" },
        { name: "durationDays", type: "uint256" },
      ],
    },
    primaryType: "EIP712UserDecryptRequest",
    domain: {
      name: "DecryptionManager",
      version: "1",
      chainId,
      verifyingContract,
    },
    message: {
      publicKey,
      contractAddresses,
      contractsChainId,
      startTimestamp,
      durationDays,
    },
  };
}

// Get signatures from signers using the EIP712 message request for user decryption
export async function getSignaturesUserDecryptRequest(
  eip712: EIP712,
  signers: HardhatEthersSigner[],
): Promise<string[]> {
  return Promise.all(
    signers.map((signer) =>
      signer.signTypedData(
        eip712.domain,
        { EIP712UserDecryptRequest: eip712.types.EIP712UserDecryptRequest },
        eip712.message,
      ),
    ),
  );
}

// Create an EIP712 message for a user decryption response
export function createEIP712ResponseUserDecrypt(
  chainId: number,
  verifyingContract: string,
  publicKey: Uint8Array,
  ctHandles: number[],
  reencryptedShare: Uint8Array,
): EIP712 {
  if (!ethers.isAddress(verifyingContract)) {
    throw new Error("Invalid verifying contract address.");
  }
  return {
    types: {
      EIP712Domain: [
        { name: "name", type: "string" },
        { name: "version", type: "string" },
        { name: "chainId", type: "uint256" },
        { name: "verifyingContract", type: "address" },
      ],
      EIP712UserDecryptResponse: [
        { name: "publicKey", type: "bytes" },
        { name: "ctHandles", type: "uint256[]" },
        { name: "reencryptedShare", type: "bytes" },
      ],
    },
    primaryType: "EIP712UserDecryptResponse",
    domain: {
      name: "DecryptionManager",
      version: "1",
      chainId,
      verifyingContract,
    },
    message: {
      publicKey,
      ctHandles,
      reencryptedShare,
    },
  };
}

// Get signatures from signers using the EIP712 message response for user decryption
export async function getSignaturesUserDecryptResponse(
  eip712: EIP712,
  signers: HardhatEthersSigner[],
): Promise<string[]> {
  return Promise.all(
    signers.map((signer) =>
      signer.signTypedData(
        eip712.domain,
        { EIP712UserDecryptResponse: eip712.types.EIP712UserDecryptResponse },
        eip712.message,
      ),
    ),
  );
}
