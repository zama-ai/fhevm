import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { toBufferBE } from "bigint-buffer";
import { ContractMethodArgs, Typed } from "ethers";
import { Signer } from "ethers";
import { ethers, network } from "hardhat";
import operatorsPrices from "./operatorsPrices.json";
import { ALL_FHE_TYPES } from "./types";

import type { Counter } from "../types";
import { TypedContractMethod } from "../types/common";
import { getSigners } from "./signers";

const hre = require("hardhat");

export async function checkIsHardhatSigner(signer: HardhatEthersSigner) {
  const signers = await hre.ethers.getSigners();
  if (signers.findIndex((s) => s.address === signer.address) === -1) {
    throw new Error(
      `The provided address (${signer.address}) is not the address of a valid hardhat signer.
      Please use addresses listed via the 'npx hardhat get-accounts --network hardhat' command.`
    );
  }
}

export const waitForBlock = (blockNumber: bigint | number) => {
  if (network.name === "hardhat") {
    return new Promise((resolve, reject) => {
      const intervalId = setInterval(async () => {
        try {
          const currentBlock = await ethers.provider.getBlockNumber();
          if (BigInt(currentBlock) >= blockNumber) {
            clearInterval(intervalId);
            resolve(currentBlock);
          }
        } catch (error) {
          clearInterval(intervalId);
          reject(error);
        }
      }, 50); // Check every 50 milliseconds
    });
  } else {
    return new Promise((resolve, reject) => {
      const waitBlock = async (currentBlock: number) => {
        if (blockNumber <= BigInt(currentBlock)) {
          await ethers.provider.off("block", waitBlock);
          resolve(blockNumber);
        }
      };
      ethers.provider.on("block", waitBlock).catch((err) => {
        reject(err);
      });
    });
  }
};

export const waitNBlocks = async (Nblocks: number) => {
  const currentBlock = await ethers.provider.getBlockNumber();
  if (network.name === "hardhat") {
    await produceDummyTransactions(Nblocks);
  }
  await waitForBlock(currentBlock + Nblocks);
};

export const waitForBalance = async (address: string): Promise<void> => {
  return new Promise((resolve, reject) => {
    const checkBalance = async () => {
      const balance = await ethers.provider.getBalance(address);
      if (balance > 0) {
        await ethers.provider.off("block", checkBalance);
        resolve();
      }
    };
    ethers.provider.on("block", checkBalance).catch((err) => {
      reject(err);
    });
  });
};

export const createTransaction = async <
  A extends [...{ [I in keyof A]-?: A[I] | Typed }]
>(
  method: TypedContractMethod<A>,
  ...params: A
) => {
  const gasLimit = await method.estimateGas(...params);
  const updatedParams: ContractMethodArgs<A> = [
    ...params,
    { gasLimit: Math.min(Math.round(+gasLimit.toString() * 1.2), 10000000) },
  ];
  return method(...updatedParams);
};

export const produceDummyTransactions = async (blockCount: number) => {
  const contract = await deployCounterContract();
  let counter = blockCount;
  while (counter > 0) {
    counter--;
    const tx = await contract.increment();
    const _ = await tx.wait();
  }
};

async function deployCounterContract(): Promise<Counter> {
  const signers = await getSigners();

  const contractFactory = await ethers.getContractFactory("Counter");
  const contract = await contractFactory.connect(signers.dave).deploy();
  await contract.waitForDeployment();

  return contract;
}

export const mineNBlocks = async (n: number) => {
  for (let index = 0; index < n; index++) {
    await ethers.provider.send("evm_mine");
  }
};

export const bigIntToBytes64 = (value: bigint) => {
  return new Uint8Array(toBufferBE(value, 64));
};

export const bigIntToBytes128 = (value: bigint) => {
  return new Uint8Array(toBufferBE(value, 128));
};

export const bigIntToBytes256 = (value: bigint) => {
  return new Uint8Array(toBufferBE(value, 256));
};

export const userDecryptSingleHandle = async (
  handle: string,
  contractAddress: string,
  instance: any,
  signer: Signer,
  privateKey: string,
  publicKey: string
): Promise<bigint | boolean | string> => {
  const HandleContractPairs = [
    {
      handle: handle,
      contractAddress: contractAddress,
    },
  ];
  const startTimeStamp = Math.floor(Date.now() / 1000).toString();
  const durationDays = "10"; // String for consistency
  const contractAddresses = [contractAddress];

  // Use the new createEIP712 function
  const eip712 = instance.createEIP712(
    publicKey,
    contractAddresses,
    startTimeStamp,
    durationDays
  );

  // Update the signing to match the new primaryType
  const signature = await signer.signTypedData(
    eip712.domain,
    {
      UserDecryptRequestVerification:
        eip712.types.UserDecryptRequestVerification,
    },
    eip712.message
  );

  const result = await instance.userDecrypt(
    HandleContractPairs,
    privateKey,
    publicKey,
    signature.replace("0x", ""),
    contractAddresses,
    signer.address,
    startTimeStamp,
    durationDays
  );

  const decryptedValue = result[handle];
  return decryptedValue;
};

const abi = [
  "event FheAdd(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)",
  "event FheSub(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)",
  "event FheMul(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)",
  "event FheDiv(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)",
  "event FheRem(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)",
  "event FheBitAnd(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)",
  "event FheBitOr(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)",
  "event FheBitXor(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)",
  "event FheShl(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)",
  "event FheShr(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)",
  "event FheRotl(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)",
  "event FheRotr(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)",
  "event FheEq(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)",
  "event FheEqBytes(address indexed caller, bytes32 lhs, bytes rhs, bytes1 scalarByte, bytes32 result)",
  "event FheNe(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)",
  "event FheNeBytes(address indexed caller, bytes32 lhs, bytes rhs, bytes1 scalarByte, bytes32 result)",
  "event FheGe(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)",
  "event FheGt(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)",
  "event FheLe(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)",
  "event FheLt(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)",
  "event FheMin(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)",
  "event FheMax(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)",
  "event FheNeg(address indexed caller, bytes32 ct, bytes32 result)",
  "event FheNot(address indexed caller, bytes32 ct, bytes32 result)",
  "event VerifyCiphertext(address indexed caller, bytes32 inputHandle, address userAddress, bytes inputProof, uint8 inputType, bytes32 result)",
  "event Cast(address indexed caller, bytes32 ct, uint8 toType, bytes32 result)",
  "event TrivialEncrypt(address indexed caller, uint256 pt, uint8 toType, bytes32 result)",
  "event TrivialEncryptBytes(address indexed caller, bytes pt, uint8 toType, bytes32 result)",
  "event FheIfThenElse(address indexed caller, bytes32 control, bytes32 ifTrue, bytes32 ifFalse, bytes32 result)",
  "event FheRand(address indexed caller, uint8 randType, bytes16 seed, bytes32 result)",
  "event FheRandBounded(address indexed caller, uint256 upperBound, uint8 randType, bytes16 seed, bytes32 result)",
];

export function getFHEGasFromTxReceipt(
  receipt: ethers.TransactionReceipt,
  FheTypes: FheType[] = ALL_FHE_TYPES
): number {
  if (receipt.status === 0) {
    throw new Error("Transaction reverted");
  }
  const coprocAddress = process.env.FHEVM_EXECUTOR_ADDRESS!;
  const contract = new ethers.Contract(coprocAddress, abi, ethers.provider);
  const relevantLogs = receipt.logs.filter((log: ethers.Log) => {
    if (log.address.toLowerCase() !== coprocAddress.toLowerCase()) {
      return false;
    }
    try {
      const parsedLog = contract.interface.parseLog({
        topics: log.topics,
        data: log.data,
      });
      return abi.some(
        (item) =>
          item.startsWith(`event ${parsedLog.name}`) &&
          parsedLog.name !== "VerifyCiphertext"
      );
    } catch {
      return false;
    }
  });
  const FHELogs = relevantLogs.map((log: ethers.Log) => {
    const parsedLog = contract.interface.parseLog({
      topics: log.topics,
      data: log.data,
    });
    return {
      name: parsedLog.name,
      args: parsedLog.args,
    };
  });
  let FHEGasConsumed = 0;
  for (const event of FHELogs) {
    let type: string | undefined;
    let typeIndex: number;
    let handle;
    switch (event.name) {
      case "TrivialEncrypt":
        typeIndex = parseInt(event.args[2]);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }

        FHEGasConsumed += (
          operatorsPrices["trivialEncrypt"].types as Record<string, number>
        )[type];
        break;

      case "TrivialEncryptBytes":
        typeIndex = parseInt(event.args[2]);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        FHEGasConsumed += (
          operatorsPrices["trivialEncrypt"].types as Record<string, number>
        )[type];
        break;

      case "FheAdd":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }

        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheAdd"].scalar as Record<string, number>
          )[type];
        } else {
          FHEGasConsumed += (
            operatorsPrices["fheAdd"].nonScalar as Record<string, number>
          )[type];
        }

        break;

      case "FheSub":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }

        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheSub"].scalar as Record<string, number>
          )[type];
        } else {
          FHEGasConsumed += (
            operatorsPrices["fheSub"].nonScalar as Record<string, number>
          )[type];
        }
        break;

      case "FheMul":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheMul"].scalar as Record<string, number>
          )[type];
        } else {
          FHEGasConsumed += (
            operatorsPrices["fheMul"].nonScalar as Record<string, number>
          )[type];
        }
        break;

      case "FheDiv":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheDiv"].scalar as Record<string, number>
          )[type];
        } else {
          throw new Error("Non-scalar div not implemented yet");
        }
        break;

      case "FheRem":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheRem"].scalar as Record<string, number>
          )[type];
        } else {
          throw new Error("Non-scalar rem not implemented yet");
        }
        break;

      case "FheBitAnd":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheBitAnd"].scalar as Record<string, number>
          )[type];
        } else {
          FHEGasConsumed += (
            operatorsPrices["fheBitAnd"].nonScalar as Record<string, number>
          )[type];
        }
        break;

      case "FheBitOr":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheBitOr"].scalar as Record<string, number>
          )[type];
        } else {
          FHEGasConsumed += (
            operatorsPrices["fheBitOr"].nonScalar as Record<string, number>
          )[type];
        }
        break;

      case "FheBitXor":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheBitXor"].scalar as Record<string, number>
          )[type];
        } else {
          FHEGasConsumed += (
            operatorsPrices["fheBitXor"].nonScalar as Record<string, number>
          )[type];
        }
        break;

      case "FheShl":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheShl"].scalar as Record<string, number>
          )[type];
        } else {
          FHEGasConsumed += (
            operatorsPrices["fheShl"].nonScalar as Record<string, number>
          )[type];
        }
        break;

      case "FheShr":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheShr"].scalar as Record<string, number>
          )[type];
        } else {
          FHEGasConsumed += (
            operatorsPrices["fheShr"].nonScalar as Record<string, number>
          )[type];
        }
        break;

      case "FheRotl":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheRotl"].scalar as Record<string, number>
          )[type];
        } else {
          FHEGasConsumed += (
            operatorsPrices["fheRotl"].nonScalar as Record<string, number>
          )[type];
        }
        break;

      case "FheRotr":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheRotr"].scalar as Record<string, number>
          )[type];
        } else {
          FHEGasConsumed += (
            operatorsPrices["fheRotr"].nonScalar as Record<string, number>
          )[type];
        }
        break;

      case "FheEq":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheEq"].scalar as Record<string, number>
          )[type];
        } else {
          FHEGasConsumed += (
            operatorsPrices["fheEq"].nonScalar as Record<string, number>
          )[type];
        }
        break;

      case "FheEqBytes":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheEq"].scalar as Record<string, number>
          )[type];
        } else {
          FHEGasConsumed += (
            operatorsPrices["fheEq"].nonScalar as Record<string, number>
          )[type];
        }

      case "FheNe":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheNe"].scalar as Record<string, number>
          )[type];
        } else {
          FHEGasConsumed += (
            operatorsPrices["fheNe"].nonScalar as Record<string, number>
          )[type];
        }
        break;

      case "FheNeBytes":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheNe"].scalar as Record<string, number>
          )[type];
        } else {
          FHEGasConsumed += (
            operatorsPrices["fheNe"].nonScalar as Record<string, number>
          )[type];
        }
        break;

      case "FheGe":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheGe"].scalar as Record<string, number>
          )[type];
        } else {
          FHEGasConsumed += (
            operatorsPrices["fheGe"].nonScalar as Record<string, number>
          )[type];
        }
        break;

      case "FheGt":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheGt"].scalar as Record<string, number>
          )[type];
        } else {
          FHEGasConsumed += (
            operatorsPrices["fheGt"].nonScalar as Record<string, number>
          )[type];
        }
        break;

      case "FheLe":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheLe"].scalar as Record<string, number>
          )[type];
        } else {
          FHEGasConsumed += (
            operatorsPrices["fheLe"].nonScalar as Record<string, number>
          )[type];
        }
        break;

      case "FheLt":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheLt"].scalar as Record<string, number>
          )[type];
        } else {
          FHEGasConsumed += (
            operatorsPrices["fheLt"].nonScalar as Record<string, number>
          )[type];
        }
        break;

      case "FheMax":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheMax"].scalar as Record<string, number>
          )[type];
        } else {
          FHEGasConsumed += (
            operatorsPrices["fheMax"].nonScalar as Record<string, number>
          )[type];
        }
        break;

      case "FheMin":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === "0x01") {
          FHEGasConsumed += (
            operatorsPrices["fheMin"].scalar as Record<string, number>
          )[type];
        } else {
          FHEGasConsumed += (
            operatorsPrices["fheMin"].nonScalar as Record<string, number>
          )[type];
        }
        break;

      case "Cast":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        FHEGasConsumed += (
          operatorsPrices["cast"].types as Record<string, number>
        )[type];
        break;

      case "FheNot":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        FHEGasConsumed += (
          operatorsPrices["fheNot"].types as Record<string, number>
        )[type];
        break;

      case "FheNeg":
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        FHEGasConsumed += (
          operatorsPrices["fheNeg"].types as Record<string, number>
        )[type];
        break;

      case "FheIfThenElse":
        handle = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        FHEGasConsumed += (
          operatorsPrices["ifThenElse"].types as Record<string, number>
        )[type];
        break;

      case "FheRand":
        typeIndex = parseInt(event.args[1]);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        FHEGasConsumed += (
          operatorsPrices["fheRand"].types as Record<string, number>
        )[type];
        break;

      case "FheRandBounded":
        typeIndex = parseInt(event.args[2]);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        FHEGasConsumed += (
          operatorsPrices["fheRandBounded"].types as Record<string, number>
        )[type];
        break;
    }
  }
  return FHEGasConsumed;
}
