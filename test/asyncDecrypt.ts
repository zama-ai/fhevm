import dotenv from 'dotenv';
import { Wallet } from 'ethers';
import fs from 'fs';
import { ethers, network } from 'hardhat';

import { GatewayContract } from '../types';
import { awaitCoprocessor, getClearText } from './coprocessorUtils';
import { waitNBlocks } from './utils';

const networkName = network.name;

const parsedEnvACL = dotenv.parse(fs.readFileSync('lib/.env.acl'));
const aclAdd = parsedEnvACL.ACL_CONTRACT_ADDRESS;

const CiphertextType = {
  0: 'bool',
  1: 'uint8', // corresponding to euint4
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

const currentTime = (): string => {
  const now = new Date();
  return now.toLocaleTimeString('en-US', { hour12: true, hour: 'numeric', minute: 'numeric', second: 'numeric' });
};

const parsedEnv = dotenv.parse(fs.readFileSync('gateway/.env.gateway'));
let relayer: Wallet;
if (networkName === 'hardhat') {
  const privKeyRelayer = process.env.PRIVATE_KEY_GATEWAY_RELAYER;
  relayer = new ethers.Wallet(privKeyRelayer!, ethers.provider);
}

const argEvents =
  '(uint256 indexed requestID, uint256[] cts, address contractCaller, bytes4 callbackSelector, uint256 msgValue, uint256 maxTimestamp, bool passSignaturesToCaller)';
const ifaceEventDecryption = new ethers.Interface(['event EventDecryption' + argEvents]);

const argEvents2 = '(uint256 indexed requestID, bool success, bytes result)';
const ifaceResultCallback = new ethers.Interface(['event ResultCallback' + argEvents2]);

let gateway: GatewayContract;
let firstBlockListening: number;
let lastBlockSnapshotForDecrypt: number;

export const initGateway = async (): Promise<void> => {
  firstBlockListening = await ethers.provider.getBlockNumber();
  if (networkName === 'hardhat' && hre.__SOLIDITY_COVERAGE_RUNNING !== true) {
    // evm_snapshot is not supported in coverage mode
    await ethers.provider.send('set_lastBlockSnapshotForDecrypt', [firstBlockListening]);
  }
  // this function will emit logs for every request and fulfilment of a decryption
  gateway = await ethers.getContractAt('GatewayContract', parsedEnv.GATEWAY_CONTRACT_PREDEPLOY_ADDRESS);
  gateway.on(
    'EventDecryption',
    async (requestID, cts, contractCaller, callbackSelector, msgValue, maxTimestamp, eventData) => {
      const blockNumber = eventData.log.blockNumber;
      console.log(`${await currentTime()} - Requested decrypt on block ${blockNumber} (requestID ${requestID})`);
    },
  );
  gateway.on('ResultCallback', async (requestID, success, result, eventData) => {
    const blockNumber = eventData.log.blockNumber;
    console.log(`${await currentTime()} - Fulfilled decrypt on block ${blockNumber} (requestID ${requestID})`);
  });
};

export const awaitAllDecryptionResults = async (): Promise<void> => {
  gateway = await ethers.getContractAt('GatewayContract', parsedEnv.GATEWAY_CONTRACT_PREDEPLOY_ADDRESS);
  const provider = ethers.provider;
  if (networkName === 'hardhat' && hre.__SOLIDITY_COVERAGE_RUNNING !== true) {
    // evm_snapshot is not supported in coverage mode
    lastBlockSnapshotForDecrypt = await provider.send('get_lastBlockSnapshotForDecrypt');
    if (lastBlockSnapshotForDecrypt < firstBlockListening) {
      firstBlockListening = lastBlockSnapshotForDecrypt + 1;
    }
  }
  await fulfillAllPastRequestsIds(networkName === 'hardhat');
  firstBlockListening = (await ethers.provider.getBlockNumber()) + 1;
  if (networkName === 'hardhat' && hre.__SOLIDITY_COVERAGE_RUNNING !== true) {
    // evm_snapshot is not supported in coverage mode
    await provider.send('set_lastBlockSnapshotForDecrypt', [firstBlockListening]);
  }
};

const getAlreadyFulfilledDecryptions = async (): Promise<[bigint]> => {
  let results = [];
  const eventDecryptionResult = await gateway.filters.ResultCallback().getTopicFilter();
  const filterDecryptionResult = {
    address: parsedEnv.GATEWAY_CONTRACT_PREDEPLOY_ADDRESS,
    fromBlock: firstBlockListening,
    toBlock: 'latest',
    topics: eventDecryptionResult,
  };
  const pastResults = await ethers.provider.getLogs(filterDecryptionResult);
  results = results.concat(pastResults.map((result) => ifaceResultCallback.parseLog(result).args[0]));

  return results;
};

const allTrue = (arr: boolean[], fn = Boolean) => arr.every(fn);

const fulfillAllPastRequestsIds = async (mocked: boolean) => {
  const eventDecryption = await gateway.filters.EventDecryption().getTopicFilter();
  const results = await getAlreadyFulfilledDecryptions();
  const filterDecryption = {
    address: parsedEnv.GATEWAY_CONTRACT_PREDEPLOY_ADDRESS,
    fromBlock: firstBlockListening,
    toBlock: 'latest',
    topics: eventDecryption,
  };
  const pastRequests = await ethers.provider.getLogs(filterDecryption);
  for (const request of pastRequests) {
    const event = ifaceEventDecryption.parseLog(request);
    const requestID = event.args[0];
    const handles = event.args[1];
    const typesList = handles.map((handle) => parseInt(handle.toString(16).slice(-4, -2), 16));
    const msgValue = event.args[4];
    const passSignaturesToCaller = event.args[6];
    if (!results.includes(requestID)) {
      // if request is not already fulfilled
      if (mocked) {
        // in mocked mode, we trigger the decryption fulfillment manually
        await awaitCoprocessor();

        // first check tat all handles are allowed for decryption
        const aclFactory = await ethers.getContractFactory('ACL');
        const acl = aclFactory.attach(aclAdd);
        const isAllowedForDec = await Promise.all(handles.map(async (handle) => acl.isAllowedForDecryption(handle)));
        if (!allTrue(isAllowedForDec)) {
          throw new Error('Some handle is not authorized for decryption');
        }
        const types = typesList.map((num) => CiphertextType[num]);
        const values = await Promise.all(handles.map(async (handle) => BigInt(await getClearText(handle))));
        const valuesFormatted = values.map((value, index) =>
          types[index] === 'address' ? '0x' + value.toString(16).padStart(40, '0') : value,
        );
        const valuesFormatted2 = valuesFormatted.map((value, index) =>
          typesList[index] === 9 ? '0x' + value.toString(16).padStart(128, '0') : value,
        );
        const valuesFormatted3 = valuesFormatted2.map((value, index) =>
          typesList[index] === 10 ? '0x' + value.toString(16).padStart(256, '0') : value,
        );
        const valuesFormatted4 = valuesFormatted3.map((value, index) =>
          typesList[index] === 11 ? '0x' + value.toString(16).padStart(512, '0') : value,
        );

        const abiCoder = new ethers.AbiCoder();
        let encodedData;
        let calldata;
        if (!passSignaturesToCaller) {
          encodedData = abiCoder.encode(['uint256', ...types], [31, ...valuesFormatted4]); // 31 is just a dummy uint256 requestID to get correct abi encoding for the remaining arguments (i.e everything except the requestID)
          calldata = '0x' + encodedData.slice(66); // we just pop the dummy requestID to get the correct value to pass for `decryptedCts`
        } else {
          encodedData = abiCoder.encode(['uint256', ...types, 'bytes[]'], [31, ...valuesFormatted4, []]); // adding also a dummy empty array of bytes for correct abi-encoding when used with signatures
          calldata = '0x' + encodedData.slice(66).slice(0, -64); // we also pop the last 32 bytes (empty bytes[])
        }

        const numSigners = +process.env.NUM_KMS_SIGNERS!;
        const decryptResultsEIP712signatures = await computeDecryptSignatures(handles, calldata, numSigners);
        const tx = await gateway
          .connect(relayer)
          .fulfillRequest(requestID, calldata, decryptResultsEIP712signatures, { value: msgValue });
        await tx.wait();
      } else {
        // in fhEVM mode we must wait until the gateway service relayer submits the decryption fulfillment tx
        await waitNBlocks(1);
        await fulfillAllPastRequestsIds(mocked);
      }
    }
  }
};

async function computeDecryptSignatures(
  handlesList: bigint[],
  decryptedResult: string,
  numSigners: number,
): Promise<string[]> {
  const signatures: string[] = [];

  for (let idx = 0; idx < numSigners; idx++) {
    const privKeySigner = process.env[`PRIVATE_KEY_KMS_SIGNER_${idx}`];
    if (privKeySigner) {
      const kmsSigner = new ethers.Wallet(privKeySigner).connect(ethers.provider);
      const signature = await kmsSign(handlesList, decryptedResult, kmsSigner);
      signatures.push(signature);
    } else {
      throw new Error(`Private key for signer ${idx} not found in environment variables`);
    }
  }
  return signatures;
}

async function kmsSign(handlesList: bigint[], decryptedResult: string, kmsSigner: Wallet) {
  const kmsAdd = dotenv.parse(fs.readFileSync('lib/.env.kmsverifier')).KMS_VERIFIER_CONTRACT_ADDRESS;
  const chainId = (await ethers.provider.getNetwork()).chainId;

  const domain = {
    name: 'KMSVerifier',
    version: '1',
    chainId: chainId,
    verifyingContract: kmsAdd,
  };

  const types = {
    DecryptionResult: [
      {
        name: 'aclAddress',
        type: 'address',
      },
      {
        name: 'handlesList',
        type: 'uint256[]',
      },
      {
        name: 'decryptedResult',
        type: 'bytes',
      },
    ],
  };
  const message = {
    aclAddress: aclAdd,
    handlesList: handlesList,
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
