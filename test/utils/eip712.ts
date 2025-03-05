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
  ctHandles: Uint8Array[],
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
      CiphertextVerification: [
        { name: "ctHandles", type: "bytes32[]" },
        { name: "userAddress", type: "address" },
        { name: "contractAddress", type: "address" },
        { name: "contractChainId", type: "uint256" },
      ],
    },
    primaryType: "CiphertextVerification",
    domain: {
      name: "ZKPoKManager",
      version: "1",
      chainId,
      verifyingContract,
    },
    message: {
      ctHandles,
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
      signer.signTypedData(
        eip712.domain,
        { CiphertextVerification: eip712.types.CiphertextVerification },
        eip712.message,
      ),
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
      PublicDecryptVerification: [
        { name: "ctHandles", type: "uint256[]" },
        { name: "decryptedResult", type: "bytes" },
      ],
    },
    primaryType: "PublicDecryptVerification",
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
      signer.signTypedData(
        eip712.domain,
        { PublicDecryptVerification: eip712.types.PublicDecryptVerification },
        eip712.message,
      ),
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
      UserDecryptRequestVerification: [
        { name: "publicKey", type: "bytes" },
        { name: "contractAddresses", type: "address[]" },
        { name: "contractsChainId", type: "uint256" },
        { name: "startTimestamp", type: "uint256" },
        { name: "durationDays", type: "uint256" },
      ],
    },
    primaryType: "UserDecryptRequestVerification",
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
        { UserDecryptRequestVerification: eip712.types.UserDecryptRequestVerification },
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
      UserDecryptResponseVerification: [
        { name: "publicKey", type: "bytes" },
        { name: "ctHandles", type: "uint256[]" },
        { name: "reencryptedShare", type: "bytes" },
      ],
    },
    primaryType: "UserDecryptResponseVerification",
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
        { UserDecryptResponseVerification: eip712.types.UserDecryptResponseVerification },
        eip712.message,
      ),
    ),
  );
}
