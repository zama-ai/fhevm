import type {
  ClearValueType,
  ClearValues,
  FhevmInstance,
  HandleContractPair,
  PublicDecryptResults,
  UserDecryptResults,
} from '@zama-fhe/relayer-sdk/node';
import { createEIP712, generateKeypair } from '@zama-fhe/relayer-sdk/node';
import { toBufferBE } from 'bigint-buffer';
import crypto from 'crypto';
import dotenv from 'dotenv';
import { ethers } from 'ethers';
import type { ethers as EthersT } from 'ethers';
import { readFileSync } from 'fs';
import hre from 'hardhat';
import { Keccak } from 'sha3';

import { getRequiredEnvVar } from '../tasks/utils/loadVariables';
import { getTxHCUFromCoprocessorTxReceipt, insertSQL } from './coprocessorUtils';
import { getCoprocessorClearText, processAllPastFHEVMExecutorEvents } from './coprocessorUtils';
import { checkIsHardhatSigner } from './utils';

////////////////////////////////////////////////////////////////////////////////
// Config
////////////////////////////////////////////////////////////////////////////////

export type EnvFhevmMockConfig = {
  kmsContractAddress: `0x${string}`;
  aclContractAddress: `0x${string}`;
  coprocessorContractAddress: `0x${string}`;
  inputVerifierContractAddress: `0x${string}`;
  verifyingContractAddressDecryption: `0x${string}`;
  verifyingContractAddressInputVerification: `0x${string}`;
  gatewayChainId: number;
  chainId: number;
  kmsThreshold: number;
  coprocessorThreshold: number;
};

function loadEnvFhevmMockConfig(): EnvFhevmMockConfig {
  const hostEnvPath = getRequiredEnvVar('ENV_HOST_ADDRESSES_PATH');
  const hosts = dotenv.parse(readFileSync(hostEnvPath));
  return {
    aclContractAddress: getAddress(hosts.ACL_CONTRACT_ADDRESS),
    coprocessorContractAddress: getAddress(hosts.FHEVM_EXECUTOR_CONTRACT_ADDRESS),
    kmsContractAddress: getAddress(hosts.KMS_VERIFIER_CONTRACT_ADDRESS),
    inputVerifierContractAddress: getAddress(hosts.INPUT_VERIFIER_CONTRACT_ADDRESS),
    verifyingContractAddressDecryption: getAddress(getRequiredEnvVar('DECRYPTION_ADDRESS')),
    verifyingContractAddressInputVerification: getAddress(getRequiredEnvVar('INPUT_VERIFICATION_ADDRESS')),
    gatewayChainId: Number(+getRequiredEnvVar('CHAIN_ID_GATEWAY')),
    chainId: Number(hre.network.config.chainId),
    kmsThreshold: Number(+getRequiredEnvVar('PUBLIC_DECRYPTION_THRESHOLD')),
    coprocessorThreshold: Number(+getRequiredEnvVar('COPROCESSOR_THRESHOLD')),
  };
}

const envFhevmMockConfig = loadEnvFhevmMockConfig();

export function getEnvFhevmMockConfig(): EnvFhevmMockConfig {
  return envFhevmMockConfig;
}

////////////////////////////////////////////////////////////////////////////////
// Public API
////////////////////////////////////////////////////////////////////////////////

export function assertNetwork(name: string) {
  if (hre.network.name !== name) {
    throw new Error(`Unsupported Hardhat network ${name}, expecting '${name}'`);
  }
}

export function getTxHCUFromTxReceipt(receipt: EthersT.TransactionReceipt): {
  globalTxHCU: number;
  maxTxHCUDepth: number;
  HCUDepthPerHandle: Record<string, number>;
} {
  return getTxHCUFromCoprocessorTxReceipt(getEnvFhevmMockConfig().coprocessorContractAddress, receipt);
}

export async function awaitCoprocessor() {
  return processAllPastFHEVMExecutorEvents(getEnvFhevmMockConfig().coprocessorContractAddress);
}

export async function getClearText(handle: string | bigint): Promise<string> {
  return getCoprocessorClearText(handle);
}

export async function createInstanceMocked(envFhevmMockConfig: EnvFhevmMockConfig): Promise<FhevmInstance> {
  const instance: FhevmInstance = {
    userDecrypt: userDecryptRequestMocked({
      aclContractAddress: envFhevmMockConfig.aclContractAddress,
      coprocessorContractAddress: envFhevmMockConfig.coprocessorContractAddress,
    }),
    publicDecrypt: publicDecryptRequestMocked({
      hostKMSSigners: await getHostKMSSigners(envFhevmMockConfig.kmsContractAddress),
      thresholdSigners: envFhevmMockConfig.kmsThreshold, // maybe host value instead of env ?
      aclContractAddress: envFhevmMockConfig.aclContractAddress,
      coprocessorContractAddress: envFhevmMockConfig.coprocessorContractAddress,
      gatewayChainId: envFhevmMockConfig.gatewayChainId,
      verifyingContractAddress: envFhevmMockConfig.verifyingContractAddressDecryption,
    }),
    createEncryptedInput: createEncryptedInputMocked,
    getPublicKey: () => null,
    getPublicParams: () => null,
    generateKeypair: generateKeypair,
    createEIP712: createEIP712(envFhevmMockConfig.verifyingContractAddressDecryption, envFhevmMockConfig.chainId),
  };
  return instance;
}

////////////////////////////////////////////////////////////////////////////////
// Types
////////////////////////////////////////////////////////////////////////////////

const MAX_USER_DECRYPT_CONTRACT_ADDRESSES = 10;
const MAX_USER_DECRYPT_DURATION_DAYS = BigInt(365);

export const ENCRYPTION_TYPES = {
  2: 0, // ebool takes 2 bits
  8: 2,
  16: 3,
  32: 4,
  64: 5,
  128: 6,
  160: 7,
  256: 8,
} as const;

const CiphertextAbiType = {
  0: 'bool',
  2: 'uint256',
  3: 'uint256',
  4: 'uint256',
  5: 'uint256',
  6: 'uint256',
  7: 'address',
  8: 'uint256',
} as const;

type EncryptedBits = keyof typeof ENCRYPTION_TYPES;
type EncryptedType = keyof typeof CiphertextAbiType;

const NumEncryptedBits: Record<EncryptedType, number> = {
  0: 2, // ebool
  2: 8, // euint8
  3: 16, // euint16
  4: 32, // euint32
  5: 64, // euint64
  6: 128, // euint128
  7: 160, // eaddress
  8: 256, // euint256
} as const;

////////////////////////////////////////////////////////////////////////////////
// userDecrypt
////////////////////////////////////////////////////////////////////////////////

const userDecryptRequestMocked =
  (params: { aclContractAddress: `0x${string}`; coprocessorContractAddress: `0x${string}` }) =>
  async (
    _handles: HandleContractPair[],
    privateKey: string,
    publicKey: string,
    signature: string,
    contractAddresses: string[],
    userAddress: string,
    startTimestamp: string | number,
    durationDays: string | number,
  ): Promise<UserDecryptResults> => {
    const { aclContractAddress, coprocessorContractAddress } = params;
    // Casting handles if string
    const handleContractPairs: { handle: `0x${string}`; contractAddress: string }[] = _handles.map((h) => ({
      handle: typeof h.handle === 'string' ? toHexString(fromHexString(h.handle)) : toHexString(h.handle),
      contractAddress: h.contractAddress,
    }));
    const handles: `0x${string}`[] = handleContractPairs.map((pair) => pair.handle);

    // Signature checking:
    const domain = {
      name: 'Decryption',
      version: '1',
      chainId: hre.network.config.chainId,
      verifyingContract: process.env.DECRYPTION_ADDRESS,
    };
    const types = {
      UserDecryptRequestVerification: [
        { name: 'publicKey', type: 'bytes' },
        { name: 'contractAddresses', type: 'address[]' },
        { name: 'startTimestamp', type: 'uint256' },
        { name: 'durationDays', type: 'uint256' },
        { name: 'extraData', type: 'bytes' },
      ],
    };
    const value = {
      publicKey: `0x${publicKey}`,
      contractAddresses: contractAddresses,
      startTimestamp: startTimestamp,
      durationDays: durationDays,
      extraData: '0x00',
    };
    const signerAddress = ethers.verifyTypedData(domain, types, value, `0x${signature}`);
    const normalizedSignerAddress = ethers.getAddress(signerAddress);
    const normalizedUserAddress = ethers.getAddress(userAddress);

    if (normalizedSignerAddress !== normalizedUserAddress) {
      throw new Error('Invalid EIP-712 signature!');
    }

    checkEncryptedBits(handles);

    checkDeadlineValidity(BigInt(startTimestamp), BigInt(durationDays));

    const contractAddressesLength = contractAddresses.length;
    if (contractAddressesLength === 0) {
      throw Error('contractAddresses is empty');
    }
    if (contractAddressesLength > MAX_USER_DECRYPT_CONTRACT_ADDRESSES) {
      throw Error(`contractAddresses max length of ${MAX_USER_DECRYPT_CONTRACT_ADDRESSES} exceeded`);
    }

    // ACL checking
    const aclFactory = await hre.ethers.getContractFactory('ACL');
    const acl = aclFactory.attach(aclContractAddress) as ethers.Contract;
    const verifications = handleContractPairs.map(async ({ handle, contractAddress }) => {
      const userAllowed = await acl.persistAllowed(handle, userAddress);
      const contractAllowed = await acl.persistAllowed(handle, contractAddress);
      if (!userAllowed) {
        throw new Error('User is not authorized to reencrypt this handle!');
      }
      if (!contractAllowed) {
        throw new Error('dApp contract is not authorized to reencrypt this handle!');
      }
      if (userAddress === contractAddress) {
        throw new Error('userAddress should not be equal to contractAddress when requesting reencryption!');
      }
    });

    await Promise.all(verifications).catch((e) => {
      throw e;
    });

    await processAllPastFHEVMExecutorEvents(coprocessorContractAddress);

    const listBigIntDecryptions: bigint[] = await Promise.all(
      handleContractPairs.map(async (handleContractPair) => BigInt(await getClearText(handleContractPair.handle))),
    );

    const results: UserDecryptResults = buildUserDecryptResults(handles, listBigIntDecryptions);

    return results;
  };

////////////////////////////////////////////////////////////////////////////////
// publicDecrypt
////////////////////////////////////////////////////////////////////////////////

const publicDecryptRequestMocked =
  (params: {
    hostKMSSigners: `0x${string}`[];
    thresholdSigners: number;
    aclContractAddress: `0x${string}`;
    coprocessorContractAddress: `0x${string}`;
    gatewayChainId: number;
    verifyingContractAddress: `0x${string}`;
  }) =>
  async (_handles: (string | Uint8Array<ArrayBufferLike>)[]): Promise<PublicDecryptResults> => {
    const {
      hostKMSSigners,
      thresholdSigners,
      aclContractAddress,
      coprocessorContractAddress,
      gatewayChainId,
      verifyingContractAddress,
    } = params;
    const extraData: `0x${string}` = '0x00';
    const aclFactory = await hre.ethers.getContractFactory('ACL');
    const acl = aclFactory.attach(aclContractAddress) as ethers.Contract;

    let handlesBytes32Hex: `0x${string}`[];
    try {
      handlesBytes32Hex = await Promise.all(
        _handles.map(async (_handle) => {
          const handle = typeof _handle === 'string' ? toHexString(fromHexString(_handle)) : toHexString(_handle);

          const isAllowedForDecryption = await acl.isAllowedForDecryption(handle);
          if (!isAllowedForDecryption) {
            throw new Error(`Handle ${handle} is not allowed for public decryption!`);
          }
          return handle;
        }),
      );
    } catch (e) {
      throw e;
    }

    // check 2048 bits limit
    checkEncryptedBits(handlesBytes32Hex);

    const payloadForRequest = {
      ciphertextHandles: handlesBytes32Hex,
      extraData,
    };

    const json = await _handleFhevmRelayerV1PublicDecrypt(coprocessorContractAddress, payloadForRequest);

    // verify signatures on decryption:
    const domain = {
      name: 'Decryption',
      version: '1',
      chainId: gatewayChainId,
      verifyingContract: verifyingContractAddress,
    };
    const types = {
      PublicDecryptVerification: [
        { name: 'ctHandles', type: 'bytes32[]' },
        { name: 'decryptedResult', type: 'bytes' },
        { name: 'extraData', type: 'bytes' },
      ],
    };
    const result = json.response[0];
    const decryptedResult: `0x${string}` = ensure0x(result.decrypted_value);
    const kmsSignatures: `0x${string}`[] = result.signatures.map(ensure0x);

    // TODO result.extra_data (RelayerPublicDecryptJsonResponse)
    const signedExtraData: `0x${string}` = '0x00';

    const recoveredAddresses: `0x${string}`[] = kmsSignatures.map((kmsSignature: `0x${string}`) => {
      const recoveredAddress = ethers.verifyTypedData(
        domain,
        types,
        { ctHandles: handlesBytes32Hex, decryptedResult, extraData: signedExtraData },
        kmsSignature,
      ) as `0x${string}`;
      return recoveredAddress;
    });

    // use KMS Signers stored on host
    const thresholdReached = isKmsThresholdReached(hostKMSSigners, recoveredAddresses, thresholdSigners);

    if (!thresholdReached) {
      throw Error('KMS signers threshold is not reached');
    }

    const clearValues: ClearValues = deserializeClearValues(handlesBytes32Hex, decryptedResult);

    const abiEnc = abiEncodeClearValues(clearValues);
    const decryptionProof = buildDecryptionProof(kmsSignatures, signedExtraData);

    return {
      clearValues,
      abiEncodedClearValues: abiEnc.abiEncodedClearValues,
      decryptionProof,
    };
  };

async function _handleFhevmRelayerV1PublicDecrypt(
  coprocessorAddress: `0x${string}`,
  payload: {
    ciphertextHandles: `0x${string}`[];
    extraData: `0x${string}`;
  },
) {
  const handlesBytes32Hex = payload.ciphertextHandles;
  const extraData = payload.extraData;

  await processAllPastFHEVMExecutorEvents(coprocessorAddress);

  const listBigIntDecryptions: bigint[] = await Promise.all(
    handlesBytes32Hex.map(async (h) => BigInt(await getClearText(h))),
  );

  const clearValues = buildClearValues(handlesBytes32Hex, listBigIntDecryptions);
  const abiEnc = abiEncodeClearValues(clearValues);

  // Use env KMS Signers. Not host KMS signers. This is needed to simulate KMS signature issues in the test suite
  const kmsSignatures = await envComputeDecryptSignaturesKms(
    handlesBytes32Hex,
    abiEnc.abiEncodedClearValues,
    extraData,
  );

  // Build relayer response
  return {
    response: [
      {
        decrypted_value: abiEnc.abiEncodedClearValues,
        signatures: kmsSignatures,
      },
    ],
  };
}

////////////////////////////////////////////////////////////////////////////////
// createEncryptedInput
////////////////////////////////////////////////////////////////////////////////

const createEncryptedInputMocked = (contractAddress: string, userAddress: string) => {
  if (!isAddress(contractAddress)) {
    throw new Error('Contract address is not a valid address.');
  }

  if (!isAddress(userAddress)) {
    throw new Error('User address is not a valid address.');
  }

  const values: bigint[] = [];
  const bits: EncryptedBits[] = [];
  return {
    addBool(value: boolean | number | bigint) {
      if (value == null) throw new Error('Missing value');
      if (typeof value !== 'boolean' && typeof value !== 'number' && typeof value !== 'bigint')
        throw new Error('The value must be a boolean, a number or a bigint.');

      // Convert to 0n or 1n
      if (typeof value === 'boolean') {
        value = value ? 1n : 0n;
      } else if (typeof value === 'number') {
        if (value !== 0 && value !== 1) {
          throw new Error('The value must be 1 or 0.');
        }
        value = value === 0 ? 0n : 1n;
      } else if (typeof value === 'bigint') {
        if (value !== 0n && value !== 1n) {
          throw new Error('The value must be 1 or 0.');
        }
      }

      values.push(value);
      bits.push(2); // ebool takes 2 bits instead of one: only exception in TFHE-rs
      if (sum(bits) > 2048) throw Error('Packing more than 2048 bits in a single input ciphertext is unsupported');
      if (bits.length > 256) throw Error('Packing more than 256 variables in a single input ciphertext is unsupported');
      return this;
    },
    add8(value: number | bigint) {
      checkEncryptedUint(value, 8);
      values.push(BigInt(value));
      bits.push(8);
      if (sum(bits) > 2048) throw Error('Packing more than 2048 bits in a single input ciphertext is unsupported');
      if (bits.length > 256) throw Error('Packing more than 256 variables in a single input ciphertext is unsupported');
      return this;
    },
    add16(value: number | bigint) {
      checkEncryptedUint(value, 16);
      values.push(BigInt(value));
      bits.push(16);
      if (sum(bits) > 2048) throw Error('Packing more than 2048 bits in a single input ciphertext is unsupported');
      if (bits.length > 256) throw Error('Packing more than 256 variables in a single input ciphertext is unsupported');
      return this;
    },
    add32(value: number | bigint) {
      checkEncryptedUint(value, 32);
      values.push(BigInt(value));
      bits.push(32);
      if (sum(bits) > 2048) throw Error('Packing more than 2048 bits in a single input ciphertext is unsupported');
      if (bits.length > 256) throw Error('Packing more than 256 variables in a single input ciphertext is unsupported');
      return this;
    },
    add64(value: number | bigint) {
      checkEncryptedUint(value, 64);
      values.push(BigInt(value));
      bits.push(64);
      if (sum(bits) > 2048) throw Error('Packing more than 2048 bits in a single input ciphertext is unsupported');
      if (bits.length > 256) throw Error('Packing more than 256 variables in a single input ciphertext is unsupported');
      return this;
    },
    add128(value: number | bigint) {
      checkEncryptedUint(value, 128);
      values.push(BigInt(value));
      bits.push(128);
      if (sum(bits) > 2048) throw Error('Packing more than 2048 bits in a single input ciphertext is unsupported');
      if (bits.length > 256) throw Error('Packing more than 256 variables in a single input ciphertext is unsupported');
      return this;
    },
    addAddress(value: string) {
      if (!isAddress(value)) {
        throw new Error('The value must be a valid address.');
      }
      values.push(BigInt(value));
      bits.push(160);
      if (sum(bits) > 2048) throw Error('Packing more than 2048 bits in a single input ciphertext is unsupported');
      if (bits.length > 256) throw Error('Packing more than 256 variables in a single input ciphertext is unsupported');
      return this;
    },
    add256(value: number | bigint) {
      checkEncryptedUint(value, 256);
      values.push(BigInt(value));
      bits.push(256);
      if (sum(bits) > 2048) throw Error('Packing more than 2048 bits in a single input ciphertext is unsupported');
      if (bits.length > 256) throw Error('Packing more than 256 variables in a single input ciphertext is unsupported');
      return this;
    },
    getValues() {
      return values;
    },
    getBits() {
      return bits;
    },
    resetValues() {
      values.length = 0;
      bits.length = 0;
      return this;
    },
    async encrypt(): Promise<{
      handles: Uint8Array[];
      inputProof: Uint8Array;
    }> {
      let mockEncryptedValues = Buffer.alloc(0);
      for (let i = 0; i < bits.length; ++i) {
        mockEncryptedValues = Buffer.concat([mockEncryptedValues, createRandomEncryptedValue(values[i], bits[i])]);
      }

      const mockEncryptedArray = new Uint8Array(mockEncryptedValues);
      const mockHash = new Keccak(256).update(Buffer.from(mockEncryptedArray)).digest();

      const extraDataV0 = ethers.solidityPacked(['uint8'], [0]);

      const chainId = process.env.SOLIDITY_COVERAGE === 'true' ? 31337 : hre.network.config.chainId;
      if (chainId === undefined) {
        throw new Error('Chain ID is not defined');
      }

      const handles = bits.map((v, i) => {
        const dataWithIndex = new Uint8Array(mockHash.length + 1);
        dataWithIndex.set(mockHash, 0);
        dataWithIndex.set([i], mockHash.length);
        const finalHash = new Keccak(256).update(Buffer.from(dataWithIndex)).digest();
        const dataInput = new Uint8Array(32);
        dataInput.set(finalHash, 0);
        // Put the index at byte21
        dataInput.set([i], 21);

        // Split the chainId over 8 bytes
        const chainIdBuffer = Buffer.alloc(8);
        chainIdBuffer.writeBigUInt64BE(BigInt(chainId), 0);

        // Add the chainId to bytes22-29
        dataInput.set(chainIdBuffer, 22);

        // Add encryption type and handle_version (which is 0) to bytes30-31
        dataInput.set([ENCRYPTION_TYPES[v], 0], 30);
        return dataInput;
      });

      const listHandlesHexNo0x = handles.map((handleAsBytes: Uint8Array) => uint8ArrayToHexStringNo0x(handleAsBytes));
      const listHandlesAsBigInt = listHandlesHexNo0x.map((handleNo0x: string) => BigInt('0x' + handleNo0x));

      const signaturesCoproc = await envComputeInputSignaturesCoproc(
        listHandlesAsBigInt,
        userAddress,
        contractAddress,
        extraDataV0,
      );

      // numHandles + numCoprocessorSigners + list_handles + signatureCoprocessorSigners (total len : 1+1+32+NUM_HANDLES*32+65*numSigners)
      let inputProof = '0x' + numberToHexNo0x(handles.length);
      inputProof += numberToHexNo0x(signaturesCoproc.length);
      listHandlesHexNo0x.map((handleNo0x) => (inputProof += handleNo0x));
      signaturesCoproc.map((sigCopro) => (inputProof += sigCopro.slice(2)));
      listHandlesHexNo0x.map((handleNo0x, i) => insertSQL('0x' + handleNo0x, values[i]));

      // Append the extra data to the input proof
      inputProof = ethers.concat([inputProof, extraDataV0]);

      return {
        handles,
        inputProof: hexStringToUint8Array(inputProof),
      };
    },
  };
};

////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////

// Add type checking
function getAddress(value: string): `0x${string}` {
  return ethers.getAddress(value) as `0x${string}`;
}
// Add type checking
function isAddress(value: unknown): value is `0x${string}` {
  return ethers.isAddress(value);
}

function ensure0x(s: string): `0x${string}` {
  return !s.startsWith('0x') ? `0x${s}` : (s as `0x${string}`);
}

function toHexString(bytes: Uint8Array): `0x${string}` {
  return `0x${bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '')}`;
}

function fromHexString(hexString: string): Uint8Array {
  const arr = hexString.replace(/^(0x)/, '').match(/.{1,2}/g);
  if (!arr) return new Uint8Array();
  return Uint8Array.from(arr.map((byte) => parseInt(byte, 16)));
}

function toBytes32HexString(value: EthersT.BigNumberish): `0x${string}` {
  return ethers.zeroPadValue(ethers.toBeHex(value), 32) as `0x${string}`;
}

function hexStringToUint8Array(hexString: string) {
  return ethers.getBytes(hexString);
}

function uint8ArrayToHexStringNo0x(uint8Array: Uint8Array): string {
  return Array.from(uint8Array)
    .map((byte) => byte.toString(16).padStart(2, '0'))
    .join('');
}

function numberToHexNo0x(num: number): string {
  let hex = num.toString(16);
  return hex.length % 2 ? '0' + hex : hex;
}

const sum = (arr: number[]) => arr.reduce((acc, val) => acc + val, 0);

////////////////////////////////////////////////////////////////////////////////
// Encrypted values
////////////////////////////////////////////////////////////////////////////////

function createRandomEncryptedValue(valueAsBigInt: bigint, numBits: EncryptedBits) {
  const numBytes = Math.ceil(numBits / 8);
  // Format: type + value + random 32
  // concatenate 32 random bytes at the end of buffer to simulate encryption noise
  const bufferValue = toBufferBE(valueAsBigInt, numBytes);
  const combinedBuffer = Buffer.concat([bufferValue, crypto.randomBytes(32)]);
  const typeBuffer = Buffer.from([ENCRYPTION_TYPES[numBits]]);
  const totalBuffer = Buffer.concat([typeBuffer, combinedBuffer]);
  return totalBuffer;
}

function checkEncryptedUint(value: number | bigint, bits: number) {
  if (value == null) throw new Error('Missing value');
  let limit;
  if (bits >= 8) {
    limit = BigInt(`0x${new Array(bits / 8).fill(null).reduce((v) => `${v}ff`, '')}`);
  } else {
    limit = BigInt(2 ** bits - 1);
  }
  if (typeof value !== 'number' && typeof value !== 'bigint') throw new Error('Value must be a number or a bigint.');
  if (typeof value === 'number') {
    if (!Number.isInteger(value)) {
      throw new Error('Value must be an unsigned integer.');
    }
    value = BigInt(value);
  }
  if (value < 0) {
    throw new Error('Value must be an unsigned integer.');
  }
  if (value > limit) {
    throw new Error(`The value exceeds the limit for ${bits}bits integer (${limit.toString()}).`);
  }
}

////////////////////////////////////////////////////////////////////////////////
// Dynamic Signers (using env variables)
////////////////////////////////////////////////////////////////////////////////

// Not dynamic
async function getHostKMSSigners(kmsContractAddress: `0x${string}`): Promise<`0x${string}`[]> {
  const abiKmsVerifier = ['function getKmsSigners() view returns (address[])'];
  const kmsContract = new ethers.Contract(kmsContractAddress, abiKmsVerifier, hre.ethers.provider);
  const signers: `0x${string}`[] = await kmsContract.getKmsSigners();
  return signers;
}

// dynamic function for testing
async function getEnvCoprocessorSigners() {
  const signers = [];
  const numSigners = getRequiredEnvVar('NUM_COPROCESSORS');
  for (let idx = 0; idx < +numSigners; idx++) {
    const signer = await hre.ethers.getSigner(getRequiredEnvVar(`COPROCESSOR_SIGNER_ADDRESS_${idx}`));
    await checkIsHardhatSigner(signer);
    signers.push(signer);
  }
  return signers;
}

// dynamic function for testing
async function getEnvKMSSigners() {
  const signers = [];
  const numSigners = getRequiredEnvVar('NUM_KMS_NODES');
  for (let idx = 0; idx < +numSigners; idx++) {
    const signer = await hre.ethers.getSigner(getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`));
    await checkIsHardhatSigner(signer);
    signers.push(signer);
  }
  return signers;
}

// Coprocessor Signers are dynamically determined using env variables
async function envComputeInputSignaturesCoproc(
  handlesList: EthersT.BigNumberish[],
  userAddress: string,
  contractAddress: string,
  extraData: string,
): Promise<`0x${string}`[]> {
  const signatures: `0x${string}`[] = [];
  const coprocSigners = await getEnvCoprocessorSigners();
  for (let idx = 0; idx < coprocSigners.length; idx++) {
    const coprocSigner = coprocSigners[idx];
    const signature = await envCoprocSign(handlesList, userAddress, contractAddress, extraData, coprocSigner);
    signatures.push(signature);
  }
  return signatures;
}

// Coprocessor Signers are dynamically determined using env variables
async function envCoprocSign(
  handlesList: EthersT.BigNumberish[],
  userAddress: string,
  contractAddress: string,
  extraData: string,
  signer: EthersT.Signer,
): Promise<`0x${string}`> {
  const inputVerificationAdd = process.env.INPUT_VERIFICATION_ADDRESS;
  const gatewayChainId = process.env.CHAIN_ID_GATEWAY;
  const hostChainId = process.env.SOLIDITY_COVERAGE === 'true' ? 31337 : hre.network.config.chainId;

  const domain = {
    name: 'InputVerification',
    version: '1',
    chainId: gatewayChainId,
    verifyingContract: inputVerificationAdd,
  };

  const types = {
    CiphertextVerification: [
      {
        name: 'ctHandles',
        type: 'bytes32[]',
      },
      {
        name: 'userAddress',
        type: 'address',
      },
      {
        name: 'contractAddress',
        type: 'address',
      },
      {
        name: 'contractChainId',
        type: 'uint256',
      },
      {
        name: 'extraData',
        type: 'bytes',
      },
    ],
  };

  const message = {
    ctHandles: handlesList.map(toBytes32HexString),
    userAddress: userAddress,
    contractAddress: contractAddress,
    contractChainId: hostChainId,
    extraData,
  };

  const signature = await signer.signTypedData(domain, types, message);
  const sigRSV = ethers.Signature.from(signature);
  const v = 27 + sigRSV.yParity;
  const r = sigRSV.r;
  const s = sigRSV.s;

  const result = r + s.substring(2) + v.toString(16);

  return result as `0x${string}`;
}

// KMS Signers are dynamically determined using env variables
async function envComputeDecryptSignaturesKms(
  handlesList: EthersT.BigNumberish[],
  decryptedResult: `0x${string}`,
  extraData: `0x${string}`,
): Promise<`0x${string}`[]> {
  const signatures: `0x${string}`[] = [];
  const kmsSigners = await getEnvKMSSigners();

  for (let i = 0; i < kmsSigners.length; i++) {
    const kmsSigner = kmsSigners[i];
    const signature = await envKmsSign(handlesList, decryptedResult, extraData, kmsSigner);
    signatures.push(signature);
  }
  return signatures;
}

// KMS Signers are dynamically determined using env variables
async function envKmsSign(
  handlesList: EthersT.BigNumberish[],
  decryptedResult: `0x${string}`,
  extraData: `0x${string}`,
  kmsSigner: EthersT.Signer,
): Promise<`0x${string}`> {
  // always keep dynamic values for testing
  const decryptionAddress = process.env.DECRYPTION_ADDRESS;
  const gatewayChainId = process.env.CHAIN_ID_GATEWAY;

  const domain = {
    name: 'Decryption',
    version: '1',
    chainId: gatewayChainId,
    verifyingContract: decryptionAddress,
  };

  const types = {
    PublicDecryptVerification: [
      {
        name: 'ctHandles',
        type: 'bytes32[]',
      },
      {
        name: 'decryptedResult',
        type: 'bytes',
      },
      {
        name: 'extraData',
        type: 'bytes',
      },
    ],
  };
  const message = {
    ctHandles: handlesList.map(toBytes32HexString),
    decryptedResult,
    extraData,
  };

  const signature = await kmsSigner.signTypedData(domain, types, message);
  const sigRSV = ethers.Signature.from(signature);
  const v = 27 + sigRSV.yParity;
  const r = sigRSV.r;
  const s = sigRSV.s;

  const result = r + s.substring(2) + v.toString(16);

  return result as `0x${string}`;
}

// Copy/paste from relayer-sdk
function toClearValueType(clearValueAsBigInt: bigint, type: number): ClearValueType {
  if (type === 0) {
    // ebool
    return clearValueAsBigInt === BigInt(1);
  } else if (type === 7) {
    // eaddress
    return getAddress('0x' + clearValueAsBigInt.toString(16).padStart(40, '0'));
  } else if (type > 8 || type == 1) {
    // type == 1 : euint4 (not supported)
    throw new Error(`Unsupported handle type ${type}`);
  }
  // euintXXX
  return clearValueAsBigInt;
}

// Copy/paste from relayer-sdk
function buildUserDecryptResults(
  handlesBytes32Hex: `0x${string}`[],
  listBigIntDecryptions: bigint[],
): UserDecryptResults {
  return buildClearValues(handlesBytes32Hex, listBigIntDecryptions);
}

function buildClearValues(handlesBytes32Hex: `0x${string}`[], listBigIntDecryptions: bigint[]): ClearValues {
  const typesList = getHandlesTypes(handlesBytes32Hex);
  const results: ClearValues = {};
  handlesBytes32Hex.forEach(
    (handle, idx) => (results[handle] = toClearValueType(listBigIntDecryptions[idx], typesList[idx])),
  );

  return results;
}

// Copy/paste from relayer-sdk
function checkDeadlineValidity(startTimestamp: bigint, durationDays: bigint) {
  if (durationDays === BigInt(0)) {
    throw Error('durationDays is null');
  }

  if (durationDays > MAX_USER_DECRYPT_DURATION_DAYS) {
    throw Error(`durationDays is above max duration of ${MAX_USER_DECRYPT_DURATION_DAYS}`);
  }

  const currentTimestamp = BigInt(Math.floor(Date.now() / 1000));
  if (startTimestamp > currentTimestamp) {
    throw Error('startTimestamp is set in the future');
  }

  const durationInSeconds = durationDays * BigInt(86400);
  if (startTimestamp + durationInSeconds < currentTimestamp) {
    throw Error('User decrypt request has expired');
  }
}

// Copy/paste from relayer-sdk
function checkEncryptedBits(handlesBytes32Hex: `0x${string}`[]): number {
  let total = 0;

  for (const handleBytes32Hex of handlesBytes32Hex) {
    const typeDiscriminant = getHandleType(handleBytes32Hex);
    total += NumEncryptedBits[typeDiscriminant as keyof typeof NumEncryptedBits];
    // enforce 2048‑bit limit
    if (total > 2048) {
      throw new Error('Cannot decrypt more than 2048 encrypted bits in a single request');
    }
  }
  return total;
}

// Copy/paste from relayer-sdk
function isKmsThresholdReached(kmsSigners: string[], recoveredAddresses: string[], threshold: number): boolean {
  if (typeof threshold !== 'number') {
    throw new Error('INTERNAL ERROR');
  }
  const uniq = new Map<string, number>();
  recoveredAddresses.forEach((address, index) => {
    if (uniq.has(address)) {
      throw new Error(`Duplicate KMS signer address found: ${address} appears multiple times in recovered addresses`);
    }
    uniq.set(address, index);
  });

  for (const address of recoveredAddresses) {
    if (!kmsSigners.includes(address)) {
      throw new Error(`Invalid address found: ${address} is not in the list of KMS signers`);
    }
  }
  return uniq.size >= threshold;
}

// Copy/paste from relayer-sdk
function abiEncodeClearValues(clearValues: ClearValues) {
  const handlesBytes32Hex = Object.keys(clearValues) as `0x${string}`[];

  const abiTypes: string[] = [];
  const abiValues: (string | bigint)[] = [];

  for (let i = 0; i < handlesBytes32Hex.length; ++i) {
    const handle = handlesBytes32Hex[i];
    const handleType: EncryptedType = getHandleType(handle);

    let clearTextValue: ClearValueType = clearValues[handle as keyof typeof clearValues];
    if (typeof clearTextValue === 'boolean') {
      clearTextValue = clearTextValue ? '0x01' : '0x00';
    }

    const clearTextValueBigInt = BigInt(clearTextValue);

    //abiTypes.push(fhevmTypeInfo.solidityTypeName);
    abiTypes.push('uint256');

    switch (handleType) {
      // eaddress
      case 7: {
        // string
        abiValues.push(`0x${clearTextValueBigInt.toString(16).padStart(40, '0')}`);
        break;
      }
      // ebool
      case 0: {
        // bigint (0 or 1)
        if (clearTextValueBigInt !== BigInt(0) && clearTextValueBigInt !== BigInt(1)) {
          throw new Error(`Invalid ebool clear text value ${clearTextValueBigInt}. Expecting 0 or 1.`);
        }
        abiValues.push(clearTextValueBigInt);
        break;
      }
      case 2: //euint8
      case 3: //euint16
      case 4: //euint32
      case 5: //euint64
      case 6: //euint128
      case 7: {
        //euint256
        // bigint
        abiValues.push(clearTextValueBigInt);
        break;
      }
      default: {
        throw new Error(`Unsupported Fhevm primitive type id: ${handleType}`);
      }
    }
  }

  const abiCoder = ethers.AbiCoder.defaultAbiCoder();

  // ABI encode the decryptedResult as done in the KMS, since all decrypted values
  // are native static types, thay have same abi-encoding as uint256:
  const abiEncodedClearValues: `0x${string}` = abiCoder.encode(abiTypes, abiValues) as `0x${string}`;

  return {
    abiTypes,
    abiValues,
    abiEncodedClearValues,
  };
}

// Copy/paste from relayer-sdk
function buildDecryptionProof(kmsSignatures: `0x${string}`[], extraData: `0x${string}`): `0x${string}` {
  // Build the decryptionProof as numSigners + KMS signatures + extraData
  const packedNumSigners = ethers.solidityPacked(['uint8'], [kmsSignatures.length]);
  const packedSignatures = ethers.solidityPacked(Array(kmsSignatures.length).fill('bytes'), kmsSignatures);
  const decryptionProof: `0x${string}` = ethers.concat([
    packedNumSigners,
    packedSignatures,
    extraData,
  ]) as `0x${string}`;
  return decryptionProof;
}

// Copy/paste from relayer-sdk
function getHandleType(handleBytes32Hex: `0x${string}`): EncryptedType {
  if (handleBytes32Hex.length !== 66) {
    throw new Error(`Handle ${handleBytes32Hex} is not of valid length`);
  }
  const hexPair = handleBytes32Hex.slice(-4, -2).toLowerCase();
  const typeDiscriminant = parseInt(hexPair, 16);

  if (!(typeDiscriminant in NumEncryptedBits)) {
    throw new Error(`Handle ${handleBytes32Hex} is not of valid type`);
  }
  return typeDiscriminant as EncryptedType;
}

function getHandlesTypes(handlesBytes32Hex: `0x${string}`[]): EncryptedType[] {
  return handlesBytes32Hex.map(getHandleType);
}

// Copy/paste from relayer-sdk
function deserializeClearValues(handlesBytes32Hex: `0x${string}`[], abiEncodedClearValues: `0x${string}`): ClearValues {
  const typesList: EncryptedType[] = getHandlesTypes(handlesBytes32Hex);

  // TODO: dummy stuff must be removed!
  const restoredEncoded =
    '0x' +
    '00'.repeat(32) + // dummy requestID (ignored)
    abiEncodedClearValues.slice(2) +
    '00'.repeat(32); // dummy empty bytes[] length (ignored)

  const abiTypes = typesList.map((t) => {
    const abiType = CiphertextAbiType[t]; // all types are valid because this was supposedly checked already inside the `checkEncryptedBits` function
    return abiType;
  });

  const coder = new ethers.AbiCoder();
  const decoded = coder.decode(['uint256', ...abiTypes, 'bytes[]'], restoredEncoded);

  // strip dummy first/last element
  const rawValues = decoded.slice(1, 1 + typesList.length);

  const results: ClearValues = {};
  handlesBytes32Hex.forEach((handle, idx) => (results[handle] = rawValues[idx]));

  return results;
}
