import dotenv from 'dotenv';
import { Signer } from 'ethers';
import fs from 'fs';
import { ethers, hardhatArguments as hre, network } from 'hardhat';

import { getRequiredEnvVar } from '../tasks/utils/loadVariables';
import { DecryptionOracle } from '../types';
import { awaitCoprocessor, getClearText } from './coprocessorUtils';
import { checkIsHardhatSigner } from './utils';

const networkName = network.name;

const parsedEnvACL = dotenv.parse(fs.readFileSync('addresses/.env.acl'));
const aclAdd = parsedEnvACL.ACL_CONTRACT_ADDRESS;

async function getKMSSigners() {
  const kmsSigners = [];
  const numKMSSigners = getRequiredEnvVar('NUM_KMS_NODES');
  for (let idx = 0; idx < +numKMSSigners; idx++) {
    const kmsSigner = await ethers.getSigner(getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`));
    await checkIsHardhatSigner(kmsSigner);
    kmsSigners.push(kmsSigner);
  }
  return kmsSigners;
}

/**
 * An object that maps numeric keys to their corresponding ciphertext types.
 * The keys represent different types of ciphertexts, and the values are their
 * respective type names as strings.
 */
const CiphertextType = {
  0: 'bool',
  2: 'uint8', // corresponding to euint8
  3: 'uint16',
  4: 'uint32',
  5: 'uint64',
  6: 'uint128',
  7: 'address',
  8: 'uint256',
  9: 'bytes',
  10: 'bytes',
  11: 'bytes',
};

let toSkip: BigInt[] = [];

const currentTime = (): string => {
  const now = new Date();
  return now.toLocaleTimeString('en-US', { hour12: true, hour: 'numeric', minute: 'numeric', second: 'numeric' });
};

const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.decryptionoracle'));
let relayer: Signer;
if (networkName === 'hardhat') {
  ethers.getSigners().then((signers) => (relayer = signers[6]));
}

const argEvents =
  '(uint256 indexed counter, uint256 requestID, bytes32[] cts, address contractCaller, bytes4 callbackSelector)';
const ifaceEventDecryption = new ethers.Interface(['event DecryptionRequest' + argEvents]);

let decryptionOracle: DecryptionOracle;
let firstBlockListening: number;
let lastBlockSnapshotForDecrypt: number;

export const initDecryptionOracle = async (): Promise<void> => {
  firstBlockListening = await ethers.provider.getBlockNumber();
  if (networkName === 'hardhat' && process.env.SOLIDITY_COVERAGE !== 'true') {
    // evm_snapshot is not supported in coverage mode
    await ethers.provider.send('set_lastBlockSnapshotForDecrypt', [firstBlockListening]);
  }
  // this function will emit logs for every request and fulfilment of a decryption
  decryptionOracle = await ethers.getContractAt(
    'decryptionOracle/DecryptionOracle.sol:DecryptionOracle',
    parsedEnv.DECRYPTION_ORACLE_ADDRESS,
  );
  decryptionOracle.on(
    'DecryptionRequest',
    async (counter, requestID, cts, contractCaller, callbackSelector, eventData) => {
      const blockNumber = eventData.log.blockNumber;
      console.log(
        `${await currentTime()} - Requested decrypt on block ${blockNumber} (counter ${counter} - requestID ${requestID})`,
      );
    },
  );
};

export const awaitAllDecryptionResults = async (): Promise<void> => {
  decryptionOracle = await ethers.getContractAt(
    'decryptionOracle/DecryptionOracle.sol:DecryptionOracle',
    parsedEnv.DECRYPTION_ORACLE_ADDRESS,
  );
  const provider = ethers.provider;
  if (networkName === 'hardhat' && process.env.SOLIDITY_COVERAGE !== 'true') {
    // evm_snapshot is not supported in coverage mode
    lastBlockSnapshotForDecrypt = await provider.send('get_lastBlockSnapshotForDecrypt');
    if (lastBlockSnapshotForDecrypt < firstBlockListening) {
      firstBlockListening = lastBlockSnapshotForDecrypt + 1;
    }
  }
  await fulfillAllPastRequestsIds(networkName === 'hardhat');
  firstBlockListening = (await ethers.provider.getBlockNumber()) + 1;
  if (networkName === 'hardhat' && process.env.SOLIDITY_COVERAGE !== 'true') {
    // evm_snapshot is not supported in coverage mode
    await provider.send('set_lastBlockSnapshotForDecrypt', [firstBlockListening]);
  }
};

const allTrue = (arr: boolean[], fn = Boolean) => arr.every(fn);

const fulfillAllPastRequestsIds = async (mocked: boolean) => {
  const eventDecryption = await decryptionOracle.filters.DecryptionRequest().getTopicFilter();
  const filterDecryption = {
    address: parsedEnv.DECRYPTION_ORACLE_ADDRESS,
    fromBlock: firstBlockListening,
    toBlock: 'latest',
    topics: eventDecryption,
  };
  const pastRequests = await ethers.provider.getLogs(filterDecryption);

  for (const request of pastRequests) {
    const event = ifaceEventDecryption.parseLog(request);
    if (!event) {
      throw new Error('Event is null');
    }
    const requestID = event.args[1];
    const handles = event.args[2];
    const contractCaller = event.args[3];
    const callbackSelector = event.args[4];
    const typesList = handles.map((handle) => parseInt(handle.toString(16).slice(-4, -2), 16));
    // if request is not already fulfilled
    if (mocked && !toSkip.includes(requestID)) {
      // in mocked mode, we trigger the decryption fulfillment manually
      await awaitCoprocessor();

      // first check tat all handles are allowed for decryption
      const aclFactory = await ethers.getContractFactory('ACL');
      const acl = aclFactory.attach(aclAdd) as ACL;
      const isAllowedForDec = await Promise.all(
        handles.map(async (handle: string) => acl.isAllowedForDecryption(handle)),
      );
      if (!allTrue(isAllowedForDec)) {
        throw new Error('Some handle is not authorized for decryption');
      }
      const types = typesList.map((num: string | number) => CiphertextType[num]);
      const values = await Promise.all(handles.map(async (handle: string) => await getClearText(handle)));

      const valuesFormatted = values.map((value, index) =>
        types[index] === 'address' ? '0x' + BigInt(value).toString(16).padStart(40, '0') : value,
      );

      const valuesFormatted2 = valuesFormatted.map((value, index) =>
        typesList[index] === 9 ? '0x' + BigInt(value).toString(16).padStart(128, '0') : value,
      );
      const valuesFormatted3 = valuesFormatted2.map((value, index) =>
        typesList[index] === 10 ? '0x' + BigInt(value).toString(16).padStart(256, '0') : value,
      );
      const valuesFormatted4 = valuesFormatted3.map((value, index) =>
        typesList[index] === 11 ? '0x' + BigInt(value).toString(16).padStart(512, '0') : value,
      );

      const abiCoder = new ethers.AbiCoder();
      let encodedData;
      let decryptedResult;

      encodedData = abiCoder.encode(['uint256', ...types, 'bytes[]'], [31, ...valuesFormatted4, []]); // 31 is just a dummy uint256 requestID to get correct abi encoding for the remaining arguments (i.e everything except the requestID)
      // + adding also a dummy empty array of bytes for correct abi-encoding when used with signatures
      decryptedResult = '0x' + encodedData.slice(66).slice(0, -64); // we pop the dummy requestID to get the correct value to pass for `decryptedCts` + we also pop the last 32 bytes (empty bytes[])

      const decryptResultsEIP712signatures = await computeDecryptSignatures(handles, decryptedResult);

      const calldata =
        callbackSelector +
        abiCoder
          .encode(['uint256', ...types, 'bytes[]'], [requestID, ...valuesFormatted4, decryptResultsEIP712signatures])
          .slice(2);

      const txData = {
        to: contractCaller,
        data: calldata,
      };
      try {
        const tx = await relayer.sendTransaction(txData);
        await tx.wait();
      } catch (error) {
        if (error instanceof Error) {
          console.log('Gateway fulfillment tx failed with the following error:', error.message);
        } else {
          console.log('Gateway fulfillment tx failed with an unknown error');
        }
        toSkip.push(requestID);
        throw error;
      }
    }
  }
};

async function computeDecryptSignatures(handlesList: string[], decryptedResult: string): Promise<string[]> {
  const signatures: string[] = [];

  let signers = await getKMSSigners();

  for (let idx = 0; idx < signers.length; idx++) {
    const kmsSigner = signers[idx];
    const signature = await kmsSign(handlesList, decryptedResult, kmsSigner);
    signatures.push(signature);
  }
  return signatures;
}

async function kmsSign(handlesList: string[], decryptedResult: string, kmsSigner: Wallet) {
  const decAdd = process.env.DECRYPTION_ADDRESS;
  const chainId = process.env.CHAIN_ID_GATEWAY;

  const domain = {
    name: 'Decryption',
    version: '1',
    chainId: chainId,
    verifyingContract: decAdd,
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
    ],
  };
  const message = {
    ctHandles: handlesList,
    decryptedResult: decryptedResult,
  };

  const signature = await kmsSigner.signTypedData(domain, types, message);
  const sigRSV = ethers.Signature.from(signature);
  const v = 27 + sigRSV.yParity;
  const r = sigRSV.r;
  const s = sigRSV.s;

  const result = r + s.substring(2) + v.toString(16);
  return result;
}
