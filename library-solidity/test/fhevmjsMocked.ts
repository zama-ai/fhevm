import { toBufferBE } from 'bigint-buffer';
import dotenv from 'dotenv';
import { type BigNumberish, type Signer, ethers } from 'ethers';
import crypto from 'node:crypto';
import * as fs from 'node:fs';

import { getRequiredEnvVar } from '../tasks/env.js';
import { coprocDbWriteCleartext } from './coprocessorUtils.js';
import { awaitCoprocessor, coprocDbReadCleartext } from './coprocessorUtils.js';
import { chainId as networkChainId, ethers as hardhatEthers } from './hardhat.js';
import {
  checkIsHardhatSigner,
  fromHexString,
  HOST_ADDRESSES_ENV_PATH,
  numberToHexNo0x,
  sum,
  toHexString,
  uint8ArrayToHexStringNo0x,
} from './utils.js';
import { ENCRYPTION_TYPES } from './legacyRelayerSdk/types.js';
import type { HandleContractPair, RelayerEncryptedInput, UserDecryptResults } from './legacyRelayerSdk/types.js';

async function getCoprocessorSigners(): Promise<Array<Signer & { address: string }>> {
  const coprocessorSigners = [];
  const numKMSSigners = getRequiredEnvVar('NUM_COPROCESSORS');
  for (let idx = 0; idx < +numKMSSigners; idx++) {
    const coprocessorSigner = await hardhatEthers.getSigner(getRequiredEnvVar(`COPROCESSOR_SIGNER_ADDRESS_${idx}`));
    await checkIsHardhatSigner(coprocessorSigner);
    coprocessorSigners.push(coprocessorSigner);
  }
  return coprocessorSigners;
}

const aclAdd = dotenv.parse(fs.readFileSync(HOST_ADDRESSES_ENV_PATH)).ACL_CONTRACT_ADDRESS;

enum Types {
  // eslint-disable-next-line @typescript-eslint/naming-convention
  ebool = 0,
  // eslint-disable-next-line @typescript-eslint/naming-convention
  euint8 = 2,
  // eslint-disable-next-line @typescript-eslint/naming-convention
  euint16 = 3,
  // eslint-disable-next-line @typescript-eslint/naming-convention
  euint32 = 4,
  // eslint-disable-next-line @typescript-eslint/naming-convention
  euint64 = 5,
  // eslint-disable-next-line @typescript-eslint/naming-convention
  euint128 = 6,
  // eslint-disable-next-line @typescript-eslint/naming-convention
  eaddress = 7,
  // eslint-disable-next-line @typescript-eslint/naming-convention
  euint256 = 8,
}

function createUintToUint8ArrayFunction(numBits: number) {
  const numBytes = Math.ceil(numBits / 8);
  return function (uint: number | bigint | boolean) {
    const buffer = toBufferBE(BigInt(uint), numBytes);

    // concatenate 32 random bytes at the end of buffer to simulate encryption noise
    const randomBytes = crypto.randomBytes(32);
    const combinedBuffer = Buffer.concat([buffer, randomBytes]);

    let byteBuffer;
    let totalBuffer;

    switch (numBits) {
      case 2: // ebool takes 2 bits
        byteBuffer = Buffer.from([Types.ebool]);
        totalBuffer = Buffer.concat([byteBuffer, combinedBuffer]);
        break;
      case 8:
        byteBuffer = Buffer.from([Types.euint8]);
        totalBuffer = Buffer.concat([byteBuffer, combinedBuffer]);
        break;
      case 16:
        byteBuffer = Buffer.from([Types.euint16]);
        totalBuffer = Buffer.concat([byteBuffer, combinedBuffer]);
        break;
      case 32:
        byteBuffer = Buffer.from([Types.euint32]);
        totalBuffer = Buffer.concat([byteBuffer, combinedBuffer]);
        break;
      case 64:
        byteBuffer = Buffer.from([Types.euint64]);
        totalBuffer = Buffer.concat([byteBuffer, combinedBuffer]);
        break;
      case 128:
        byteBuffer = Buffer.from([Types.euint128]);
        totalBuffer = Buffer.concat([byteBuffer, combinedBuffer]);
        break;
      case 160:
        byteBuffer = Buffer.from([Types.eaddress]);
        totalBuffer = Buffer.concat([byteBuffer, combinedBuffer]);
        break;
      case 256:
        byteBuffer = Buffer.from([Types.euint256]);
        totalBuffer = Buffer.concat([byteBuffer, combinedBuffer]);
        break;
      default:
        throw Error('Non-supported numBits');
    }

    return totalBuffer;
  };
}

export const userDecryptRequestMocked = async (
  _handles: HandleContractPair[],
  _privateKey: string,
  publicKey: string,
  signature: string,
  contractAddresses: string[],
  userAddress: string,
  startTimestamp: string | number,
  durationDays: string | number,
): Promise<UserDecryptResults> => {
  // Casting handles if string
  const handles = _handles.map((h) => ({
    handle: typeof h.handle === 'string' ? toHexString(fromHexString(h.handle), true) : toHexString(h.handle, true),
    contractAddress: h.contractAddress,
  }));

  // Signature checking:
  const domain = {
    name: 'Decryption',
    version: '1',
    chainId: networkChainId,
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

  // ACL checking
  const aclFactory = await hardhatEthers.getContractFactory('ACL');
  const acl = aclFactory.attach(aclAdd);
  const verifications = handles.map(async ({ handle, contractAddress }) => {
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

  await Promise.all(verifications);
  await awaitCoprocessor();

  return Object.fromEntries(
    await Promise.all(
      handles.map(async ({ handle }) => [handle, BigInt(await coprocDbReadCleartext(handle))] as const),
    ),
  );
};

// Returns the 32-byte external handle of the `index`-th value of an encrypted input whose ciphertext hash is `inputHash`.
function computeExternalHandle(
  inputHash: Uint8Array,
  index: number,
  chainId: number | bigint,
  fheTypeId: number,
): Uint8Array {
  // Append length to inputHash
  const dataWithIndex = new Uint8Array(inputHash.length + 1);
  dataWithIndex.set(inputHash, 0);
  dataWithIndex.set([index], inputHash.length);

  const finalHash = ethers.getBytes(ethers.keccak256(dataWithIndex));
  const handle = new Uint8Array(32);
  handle.set(finalHash, 0);
  // Put the index at byte21
  handle.set([index], 21);

  // Split the chainId over 8 bytes
  const chainIdBuffer = Buffer.alloc(8);
  chainIdBuffer.writeBigUInt64BE(BigInt(chainId), 0);

  // Add the chainId to bytes22-29
  handle.set(chainIdBuffer, 22);

  // Add encryption type and handle_version (which is 0) to bytes30-31
  handle.set([fheTypeId, 0], 30);
  return handle;
}

// Number of coprocessor signers, read from the NUM_COPROCESSORS env variable.
function getNumCoprocessors(): number {
  const raw = getRequiredEnvVar('NUM_COPROCESSORS');
  const numCoprocessors = Number(raw);
  if (!Number.isInteger(numCoprocessors) || numCoprocessors < 0) {
    throw new Error(`"NUM_COPROCESSORS" env variable must be a non-negative integer, got "${raw}"`);
  }
  return numCoprocessors;
}

// Throws if `value` is null or undefined. Takes `unknown` so the check also guards untyped (JS or `any`) callers.
function assertIsNotNullish(value: unknown): void {
  if (value === null || value === undefined) {
    throw new Error('Missing value');
  }
}

export function createEncryptedInputMocked(contractAddress: string, userAddress: string): RelayerEncryptedInput {
  if (!ethers.isAddress(contractAddress)) {
    throw new Error('Contract address is not a valid address.');
  }

  if (!ethers.isAddress(userAddress)) {
    throw new Error('User address is not a valid address.');
  }

  const values: bigint[] = [];
  const bits: Array<keyof typeof ENCRYPTION_TYPES> = [];
  return {
    addBool(value: boolean | number | bigint) {
      assertIsNotNullish(value);
      if (typeof value !== 'boolean' && typeof value !== 'number' && typeof value !== 'bigint')
        throw new Error('The value must be a boolean, a number or a bigint.');
      if ((typeof value !== 'bigint' || typeof value !== 'number') && Number(value) > 1)
        throw new Error('The value must be 1 or 0.');
      values.push(BigInt(value));
      bits.push(2); // ebool takes 2 bits instead of one: only exception in TFHE-rs
      if (sum(bits) > 2048) throw Error('Packing more than 2048 bits in a single input ciphertext is unsupported');
      if (bits.length > 256) throw Error('Packing more than 256 variables in a single input ciphertext is unsupported');
      return this;
    },
    add8(value: number | bigint) {
      checkEncryptedValue(value, 8);
      values.push(BigInt(value));
      bits.push(8);
      if (sum(bits) > 2048) throw Error('Packing more than 2048 bits in a single input ciphertext is unsupported');
      if (bits.length > 256) throw Error('Packing more than 256 variables in a single input ciphertext is unsupported');
      return this;
    },
    add16(value: number | bigint) {
      checkEncryptedValue(value, 16);
      values.push(BigInt(value));
      bits.push(16);
      if (sum(bits) > 2048) throw Error('Packing more than 2048 bits in a single input ciphertext is unsupported');
      if (bits.length > 256) throw Error('Packing more than 256 variables in a single input ciphertext is unsupported');
      return this;
    },
    add32(value: number | bigint) {
      checkEncryptedValue(value, 32);
      values.push(BigInt(value));
      bits.push(32);
      if (sum(bits) > 2048) throw Error('Packing more than 2048 bits in a single input ciphertext is unsupported');
      if (bits.length > 256) throw Error('Packing more than 256 variables in a single input ciphertext is unsupported');
      return this;
    },
    add64(value: number | bigint) {
      checkEncryptedValue(value, 64);
      values.push(BigInt(value));
      bits.push(64);
      if (sum(bits) > 2048) throw Error('Packing more than 2048 bits in a single input ciphertext is unsupported');
      if (bits.length > 256) throw Error('Packing more than 256 variables in a single input ciphertext is unsupported');
      return this;
    },
    add128(value: number | bigint) {
      checkEncryptedValue(value, 128);
      values.push(BigInt(value));
      bits.push(128);
      if (sum(bits) > 2048) throw Error('Packing more than 2048 bits in a single input ciphertext is unsupported');
      if (bits.length > 256) throw Error('Packing more than 256 variables in a single input ciphertext is unsupported');
      return this;
    },
    addAddress(value: string) {
      if (!ethers.isAddress(value)) {
        throw new Error('The value must be a valid address.');
      }
      values.push(BigInt(value));
      bits.push(160);
      if (sum(bits) > 2048) throw Error('Packing more than 2048 bits in a single input ciphertext is unsupported');
      if (bits.length > 256) throw Error('Packing more than 256 variables in a single input ciphertext is unsupported');
      return this;
    },
    add256(value: number | bigint) {
      checkEncryptedValue(value, 256);
      values.push(BigInt(value));
      bits.push(256);
      if (sum(bits) > 2048) throw Error('Packing more than 2048 bits in a single input ciphertext is unsupported');
      if (bits.length > 256) throw Error('Packing more than 256 variables in a single input ciphertext is unsupported');
      return this;
    },
    getBits() {
      return bits;
    },
    async encrypt() {
      let encrypted = Buffer.alloc(0);

      bits.map((v, i) => {
        encrypted = Buffer.concat([encrypted, createUintToUint8ArrayFunction(v)(values[i])]);
      });

      const encryptedArray = new Uint8Array(encrypted);
      const hash = ethers.getBytes(ethers.keccak256(encryptedArray));
      const extraDataV0 = ethers.solidityPacked(['uint8'], [0]);

      const chainId = networkChainId;
      if (chainId === undefined) {
        throw new Error('Chain ID is not defined');
      }

      const handles = bits.map((v, i) => computeExternalHandle(hash, i, chainId, ENCRYPTION_TYPES[v]));

      let inputProofHex = '0x' + numberToHexNo0x(handles.length); // numHandles + numCoprocessorSigners + list_handles + signatureCoprocessorSigners (total len : 1+1+32+NUM_HANDLES*32+65*numSigners)
      const numSigners = getNumCoprocessors();
      inputProofHex += numberToHexNo0x(numSigners);

      const listHandlesStr = handles.map((i) => uint8ArrayToHexStringNo0x(i));
      listHandlesStr.map((handle) => (inputProofHex += handle));
      const listHandles = listHandlesStr.map((i) => BigInt('0x' + i));
      const signaturesCoproc = await computeInputSignaturesCopro(
        listHandles,
        userAddress,
        contractAddress,
        extraDataV0,
      );
      signaturesCoproc.map((sigCopro) => (inputProofHex += sigCopro.slice(2)));
      listHandlesStr.forEach((handle, i) => {
        coprocDbWriteCleartext('0x' + handle, values[i]);
      });

      // Append the extra data to the input proof
      inputProofHex = ethers.concat([inputProofHex, extraDataV0]);

      return {
        handles,
        inputProof: ethers.getBytes(inputProofHex),
      };
    },
  };
}

function checkEncryptedValue(value: number | bigint, bits: number): void {
  assertIsNotNullish(value);
  let limit;
  if (bits >= 8) {
    limit = BigInt(`0x${new Array(bits / 8).fill(null).reduce((v) => `${v}ff`, '')}`);
  } else {
    limit = BigInt(2 ** bits - 1);
  }
  if (typeof value !== 'number' && typeof value !== 'bigint') throw new Error('Value must be a number or a bigint.');
  if (value > limit) {
    throw new Error(`The value exceeds the limit for ${bits}bits integer (${limit.toString()}).`);
  }
}

async function computeInputSignaturesCopro(
  handlesList: BigNumberish[],
  userAddress: string,
  contractAddress: string,
  extraData: string,
): Promise<string[]> {
  const signatures: string[] = [];
  const numSigners = getNumCoprocessors();
  const signers = await getCoprocessorSigners();

  for (let idx = 0; idx < numSigners; idx++) {
    const coprocSigner = signers[idx];
    const signature = await coprocSign(handlesList, userAddress, contractAddress, extraData, coprocSigner);
    signatures.push(signature);
  }
  return signatures;
}

async function coprocSign(
  handlesList: BigNumberish[],
  userAddress: string,
  contractAddress: string,
  extraData: string,
  signer: Signer,
): Promise<string> {
  const inputVerificationAdd = process.env.INPUT_VERIFICATION_ADDRESS;
  const chainId = process.env.CHAIN_ID_GATEWAY;
  const hostChainId = networkChainId;

  const domain = {
    name: 'InputVerification',
    version: '1',
    chainId: chainId,
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
    ctHandles: handlesList.map((handle) => ethers.zeroPadValue(ethers.toBeHex(handle), 32)),
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
  return result;
}
