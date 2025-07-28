import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { HDNodeWallet, Wallet } from "ethers";
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
  ctHandles: string[],
  userAddress: string,
  contractAddress: string,
  contractChainId: number,
  extraData: string,
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
        { name: "extraData", type: "bytes" },
      ],
    },
    primaryType: "CiphertextVerification",
    domain: {
      name: "InputVerification",
      version: "1",
      chainId,
      verifyingContract,
    },
    message: {
      ctHandles,
      userAddress,
      contractAddress,
      contractChainId,
      extraData,
    },
  };
}

// Get signatures from signers using the EIP712 message response for proof verification
export async function getSignaturesZKPoK(
  eip712: EIP712,
  signers: (HardhatEthersSigner | HDNodeWallet | Wallet)[],
): Promise<string[]> {
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
  ctHandles: string[],
  decryptedResult: string,
  extraData: string,
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
        { name: "ctHandles", type: "bytes32[]" },
        { name: "decryptedResult", type: "bytes" },
        { name: "extraData", type: "bytes" },
      ],
    },
    primaryType: "PublicDecryptVerification",
    domain: {
      name: "Decryption",
      version: "1",
      chainId,
      verifyingContract,
    },
    message: {
      ctHandles,
      decryptedResult,
      extraData,
    },
  };
}

// Get signatures from signers using the EIP712 message response for public decryption
export async function getSignaturesPublicDecrypt(
  eip712: EIP712,
  signers: (HardhatEthersSigner | Wallet | HDNodeWallet)[],
): Promise<string[]> {
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
  verifyingContract: string,
  publicKey: string,
  contractAddresses: string[],
  contractsChainId: number,
  startTimestamp: string,
  durationDays: string,
  extraData: string,
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
        { name: "extraData", type: "bytes" },
      ],
    },
    primaryType: "UserDecryptRequestVerification",
    domain: {
      name: "Decryption",
      version: "1",
      chainId: contractsChainId,
      verifyingContract,
    },
    message: {
      publicKey,
      contractAddresses,
      contractsChainId,
      startTimestamp,
      durationDays,
      extraData,
    },
  };
}

// Get signatures from signers using the EIP712 message request for user decryption
export async function getSignaturesUserDecryptRequest(
  eip712: EIP712,
  signers: (HardhatEthersSigner | Wallet | HDNodeWallet)[],
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

// Create an EIP712 message for a user decryption request
export function createEIP712RequestDelegatedUserDecrypt(
  verifyingContract: string,
  publicKey: string,
  contractAddresses: string[],
  delegatorAddress: string,
  contractsChainId: number,
  startTimestamp: string,
  durationDays: string,
  extraData: string,
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
      DelegatedUserDecryptRequestVerification: [
        { name: "publicKey", type: "bytes" },
        { name: "contractAddresses", type: "address[]" },
        { name: "delegatorAddress", type: "address" },
        { name: "contractsChainId", type: "uint256" },
        { name: "startTimestamp", type: "uint256" },
        { name: "durationDays", type: "uint256" },
        { name: "extraData", type: "bytes" },
      ],
    },
    primaryType: "DelegatedUserDecryptRequestVerification",
    domain: {
      name: "Decryption",
      version: "1",
      chainId: contractsChainId,
      verifyingContract,
    },
    message: {
      publicKey,
      contractAddresses,
      delegatorAddress,
      contractsChainId,
      startTimestamp,
      durationDays,
      extraData,
    },
  };
}

// Get signatures from signers using the EIP712 message request for user decryption
export async function getSignaturesDelegatedUserDecryptRequest(
  eip712: EIP712,
  signers: (HardhatEthersSigner | HDNodeWallet | Wallet)[],
): Promise<string[]> {
  return Promise.all(
    signers.map((signer) =>
      signer.signTypedData(
        eip712.domain,
        { DelegatedUserDecryptRequestVerification: eip712.types.DelegatedUserDecryptRequestVerification },
        eip712.message,
      ),
    ),
  );
}

// Create an EIP712 message for a user decryption response
export function createEIP712ResponseUserDecrypt(
  chainId: number,
  verifyingContract: string,
  publicKey: string,
  ctHandles: string[],
  userDecryptedShare: string,
  extraData: string,
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
        { name: "ctHandles", type: "bytes32[]" },
        { name: "userDecryptedShare", type: "bytes" },
        { name: "extraData", type: "bytes" },
      ],
    },
    primaryType: "UserDecryptResponseVerification",
    domain: {
      name: "Decryption",
      version: "1",
      chainId,
      verifyingContract,
    },
    message: {
      publicKey,
      ctHandles,
      userDecryptedShare,
      extraData,
    },
  };
}

// Get signatures from signers using the EIP712 message response for user decryption
export async function getSignaturesUserDecryptResponse(
  eip712s: EIP712[],
  signers: (HardhatEthersSigner | HDNodeWallet | Wallet)[],
): Promise<string[]> {
  if (eip712s.length !== signers.length) {
    throw new Error("The number of EIP712 messages must match the number of signers.");
  }

  return Promise.all(
    signers.map((signer, index) =>
      signer.signTypedData(
        eip712s[index].domain,
        { UserDecryptResponseVerification: eip712s[index].types.UserDecryptResponseVerification },
        eip712s[index].message,
      ),
    ),
  );
}
