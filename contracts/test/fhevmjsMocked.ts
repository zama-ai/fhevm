import { toBigIntBE } from 'bigint-buffer';
import { toBufferBE } from 'bigint-buffer';
import crypto from 'crypto';
import dotenv from 'dotenv';
import { Wallet, ethers } from 'ethers';
import * as fs from 'fs';
import { Keccak } from 'sha3';
import { isAddress } from 'web3-validator';

import { insertSQL } from './coprocessorUtils';
import { awaitCoprocessor, getClearText } from './coprocessorUtils';

const hre = require('hardhat');

const parsedEnvACL = dotenv.parse(fs.readFileSync('addresses/.env.acl'));
const aclAdd = parsedEnvACL.ACL_CONTRACT_ADDRESS;

enum Types {
  ebool = 0,
  euint4,
  euint8,
  euint16,
  euint32,
  euint64,
  euint128,
  eaddress,
  euint256,
  ebytes64,
  ebytes128,
  ebytes256,
}

const sum = (arr: number[]) => arr.reduce((acc, val) => acc + val, 0);

function bytesToBigInt(byteArray: Uint8Array): bigint {
  if (!byteArray || byteArray?.length === 0) {
    return BigInt(0);
  }
  const buffer = Buffer.from(byteArray);
  const result = toBigIntBE(buffer);
  return result;
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
      case 4:
        byteBuffer = Buffer.from([Types.euint4]);
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
      case 512:
        byteBuffer = Buffer.from([Types.ebytes64]);
        totalBuffer = Buffer.concat([byteBuffer, combinedBuffer]);
        break;
      case 1024:
        byteBuffer = Buffer.from([Types.ebytes128]);
        totalBuffer = Buffer.concat([byteBuffer, combinedBuffer]);
        break;
      case 2048:
        byteBuffer = Buffer.from([Types.ebytes256]);
        totalBuffer = Buffer.concat([byteBuffer, combinedBuffer]);
        break;
      default:
        throw Error('Non-supported numBits');
    }

    return totalBuffer;
  };
}

export const reencryptRequestMocked = async (
  handle: bigint,
  privateKey: string,
  publicKey: string,
  signature: string,
  contractAddress: string,
  userAddress: string,
) => {
  // Signature checking:
  const domain = {
    name: 'Authorization token',
    version: '1',
    chainId: hre.network.config.chainId,
    verifyingContract: contractAddress,
  };
  const types = {
    Reencrypt: [{ name: 'publicKey', type: 'bytes' }],
  };
  const value = {
    publicKey: `0x${publicKey}`,
  };
  const signerAddress = ethers.verifyTypedData(domain, types, value, `0x${signature}`);
  const normalizedSignerAddress = ethers.getAddress(signerAddress);
  const normalizedUserAddress = ethers.getAddress(userAddress);
  if (normalizedSignerAddress !== normalizedUserAddress) {
    throw new Error('Invalid EIP-712 signature!');
  }

  // ACL checking
  const aclFactory = await hre.ethers.getContractFactory('ACL');
  const acl = aclFactory.attach(aclAdd);
  const userAllowed = await acl.persistAllowed(handle, userAddress);
  const contractAllowed = await acl.persistAllowed(handle, contractAddress);
  const isAllowed = userAllowed && contractAllowed;
  if (!isAllowed) {
    throw new Error('User is not authorized to reencrypt this handle!');
  }
  if (userAddress === contractAddress) {
    throw new Error('userAddress should not be equal to contractAddress when requesting reencryption!');
  }
  await awaitCoprocessor();
  return BigInt(await getClearText(handle));
};

export const createEncryptedInputMocked = (contractAddress: string, userAddress: string) => {
  if (!isAddress(contractAddress)) {
    throw new Error('Contract address is not a valid address.');
  }

  if (!isAddress(userAddress)) {
    throw new Error('User address is not a valid address.');
  }

  const values: bigint[] = [];
  const bits: (keyof typeof ENCRYPTION_TYPES)[] = [];
  return {
    addBool(value: boolean | number | bigint) {
      if (value == null) throw new Error('Missing value');
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
    add4(value: number | bigint) {
      checkEncryptedValue(value, 4);
      values.push(BigInt(value));
      bits.push(4);
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
      checkEncryptedValue(value, 256);
      values.push(BigInt(value));
      bits.push(256);
      if (sum(bits) > 2048) throw Error('Packing more than 2048 bits in a single input ciphertext is unsupported');
      if (bits.length > 256) throw Error('Packing more than 256 variables in a single input ciphertext is unsupported');
      return this;
    },
    addBytes64(value: Uint8Array) {
      if (value.length !== 64) throw Error('Uncorrect length of input Uint8Array, should be 64 for an ebytes64');
      const bigIntValue = bytesToBigInt(value);
      checkEncryptedValue(bigIntValue, 512);
      values.push(bigIntValue);
      bits.push(512);
      if (sum(bits) > 2048) throw Error('Packing more than 2048 bits in a single input ciphertext is unsupported');
      if (bits.length > 256) throw Error('Packing more than 256 variables in a single input ciphertext is unsupported');
      return this;
    },
    addBytes128(value: Uint8Array) {
      if (value.length !== 128) throw Error('Uncorrect length of input Uint8Array, should be 128 for an ebytes128');
      const bigIntValue = bytesToBigInt(value);
      checkEncryptedValue(bigIntValue, 1024);
      values.push(bigIntValue);
      bits.push(1024);
      if (sum(bits) > 2048) throw Error('Packing more than 2048 bits in a single input ciphertext is unsupported');
      if (bits.length > 256) throw Error('Packing more than 256 variables in a single input ciphertext is unsupported');
      return this;
    },
    addBytes256(value: Uint8Array) {
      if (value.length !== 256) throw Error('Uncorrect length of input Uint8Array, should be 256 for an ebytes256');
      const bigIntValue = bytesToBigInt(value);
      checkEncryptedValue(bigIntValue, 2048);
      values.push(bigIntValue);
      bits.push(2048);
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
    async encrypt() {
      let encrypted = Buffer.alloc(0);

      bits.map((v, i) => {
        encrypted = Buffer.concat([encrypted, createUintToUint8ArrayFunction(v)(values[i])]);
      });

      const encryptedArray = new Uint8Array(encrypted);
      const hash = new Keccak(256).update(Buffer.from(encryptedArray)).digest();

      const handles = bits.map((v, i) => {
        const dataWithIndex = new Uint8Array(hash.length + 1);
        dataWithIndex.set(hash, 0);
        dataWithIndex.set([i], hash.length);
        const finalHash = new Keccak(256).update(Buffer.from(dataWithIndex)).digest();
        const dataInput = new Uint8Array(32);
        dataInput.set(finalHash, 0);
        dataInput.set([i, ENCRYPTION_TYPES[v], 0], 29);
        return dataInput;
      });
      let inputProof = '0x' + numberToHex(handles.length); // numHandles + numCoprocessorSigners + list_handles + signatureCoprocessorSigners (total len : 1+1+32+NUM_HANDLES*32+65*numSigners)
      const numSigners = +process.env.NUM_COPROCESSOR_SIGNERS!;
      inputProof += numberToHex(numSigners);

      const listHandlesStr = handles.map((i) => uint8ArrayToHexString(i));
      listHandlesStr.map((handle) => (inputProof += handle));
      const listHandles = listHandlesStr.map((i) => BigInt('0x' + i));
      const signaturesCoproc = await computeInputSignaturesCopro(listHandles, userAddress, contractAddress);
      signaturesCoproc.map((sigCopro) => (inputProof += sigCopro.slice(2)));
      listHandlesStr.map((handle, i) => insertSQL('0x' + handle, values[i]));

      return {
        handles,
        inputProof,
      };
    },
  };
};

function uint8ArrayToHexString(uint8Array: Uint8Array) {
  return Array.from(uint8Array)
    .map((byte) => byte.toString(16).padStart(2, '0'))
    .join('');
}

function numberToHex(num: number) {
  let hex = num.toString(16);
  return hex.length % 2 ? '0' + hex : hex;
}

const checkEncryptedValue = (value: number | bigint, bits: number) => {
  if (value == null) throw new Error('Missing value');
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
};

export const ENCRYPTION_TYPES = {
  2: 0, // ebool takes 2 bits
  4: 1,
  8: 2,
  16: 3,
  32: 4,
  64: 5,
  128: 6,
  160: 7,
  256: 8,
  512: 9,
  1024: 10,
  2048: 11,
};

async function computeInputSignaturesCopro(
  handlesList: bigint[],
  userAddress: string,
  contractAddress: string,
): Promise<string[]> {
  const signatures: string[] = [];
  const numSigners = +process.env.NUM_COPROCESSOR_SIGNERS!;
  for (let idx = 0; idx < numSigners; idx++) {
    const privKeySigner = process.env[`PRIVATE_KEY_COPROCESSOR_ACCOUNT_${idx}`];
    if (privKeySigner) {
      const coprocSigner = new Wallet(privKeySigner).connect(ethers.provider);
      const signature = await coprocSign(handlesList, userAddress, contractAddress, coprocSigner);
      signatures.push(signature);
    } else {
      throw new Error(`Private key for coprocessor signer ${idx} not found in environment variables`);
    }
  }
  return signatures;
}

async function coprocSign(
  handlesList: bigint[],
  userAddress: string,
  contractAddress: string,
  signer: Wallet,
): Promise<string> {
  const zkpokManAdd = dotenv.parse(fs.readFileSync('addressesL2/.env.zkpokmanager')).ZKPOK_MANAGER_ADDRESS;
  const chainId = dotenv.parse(fs.readFileSync('.env')).CHAIN_ID_GATEWAY;
  const hostChainId = hre.__SOLIDITY_COVERAGE_RUNNING ? 31337 : hre.network.config.chainId;

  const domain = {
    name: 'ZKPoKManager',
    version: '1',
    chainId: chainId,
    verifyingContract: zkpokManAdd,
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
    ],
  };

  const message = {
    ctHandles: handlesList.map((handle) => ethers.zeroPadValue(ethers.toBeHex(handle), 32)),
    userAddress: userAddress,
    contractAddress: contractAddress,
    contractChainId: hostChainId,
  };

  const signature = await signer.signTypedData(domain, types, message);
  const sigRSV = ethers.Signature.from(signature);
  const v = 27 + sigRSV.yParity;
  const r = sigRSV.r;
  const s = sigRSV.s;

  const result = r + s.substring(2) + v.toString(16);
  return result;
}
