import dotenv from 'dotenv';
import fs from 'fs';
import { ethers, network } from 'hardhat';

import { GatewayContract } from '../types';
import { awaitCoprocessor, getClearText } from './coprocessorUtils';
import { waitNBlocks } from './utils';

const networkName = network.name;

const parsedEnvACL = dotenv.parse(fs.readFileSync('lib/.env.acl'));
const aclAdd = parsedEnvACL.ACL_CONTRACT_ADDRESS.replace(/^0x/, '').replace(/^0+/, '').toLowerCase();

const CiphertextType = {
  0: 'bool',
  1: 'uint8', // corresponding to euint4
  2: 'uint8', // corresponding to euint8
  3: 'uint16',
  4: 'uint32',
  5: 'uint64',
  6: 'uint128',
  7: 'address',
  11: 'bytes',
};

const currentTime = (): string => {
  const now = new Date();
  return now.toLocaleTimeString('en-US', { hour12: true, hour: 'numeric', minute: 'numeric', second: 'numeric' });
};

const parsedEnv = dotenv.parse(fs.readFileSync('gateway/.env.gateway'));
const privKeyRelayer = process.env.PRIVATE_KEY_GATEWAY_RELAYER;
const relayer = new ethers.Wallet(privKeyRelayer!, ethers.provider);

const argEvents =
  '(uint256 indexed requestID, uint256[] cts, address contractCaller, bytes4 callbackSelector, uint256 msgValue, uint256 maxTimestamp, bool passSignaturesToCaller)';
const ifaceEventDecryption = new ethers.Interface(['event EventDecryption' + argEvents]);

const argEvents2 = '(uint256 indexed requestID, bool success, bytes result)';
const ifaceResultCallback = new ethers.Interface(['event ResultCallback' + argEvents2]);

let gateway: GatewayContract;
let firstBlockListening: number;
let lastBlockSnapshotForDecrypt: number;

export const asyncDecrypt = async (): Promise<void> => {
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
    address: process.env.GATEWAY_CONTRACT_PREDEPLOY_ADDRESS,
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
    address: process.env.GATEWAY_CONTRACT_PREDEPLOY_ADDRESS,
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
    if (!results.includes(requestID)) {
      // if request is not already fulfilled
      if (mocked) {
        // in mocked mode, we trigger the decryption fulfillment manually
        await awaitCoprocessor();

        // first check tat all handles are allowed for decryption
        const aclFactory = await ethers.getContractFactory('ACL');
        const acl = aclFactory.attach(`0x${aclAdd}`);
        const isAllowedForDec = await Promise.all(handles.map(async (handle) => acl.allowedForDecryption(handle)));
        if (!allTrue(isAllowedForDec)) {
          throw new Error('Some handle is not authorized for decryption');
        }

        const types = typesList.map((num) => CiphertextType[num]);
        const values = await Promise.all(handles.map(async (handle) => BigInt(await getClearText(handle))));
        const valuesFormatted = values.map((value, index) =>
          types[index] === 'address' ? '0x' + value.toString(16).padStart(40, '0') : value,
        );
        const valuesFormatted2 = valuesFormatted.map((value, index) =>
          types[index] === 'bytes' ? '0x' + value.toString(16).padStart(512, '0') : value,
        );

        const abiCoder = new ethers.AbiCoder();
        const encodedData = abiCoder.encode(['uint256', ...types], [31, ...valuesFormatted2]); // 31 is just a dummy uint256 requestID to get correct abi encoding for the remaining arguments (i.e everything except the requestID)
        const calldata = '0x' + encodedData.slice(66); // we just pop the dummy requestID to get the correct value to pass for `decryptedCts`

        const tx = await gateway.connect(relayer).fulfillRequest(requestID, calldata, [], { value: msgValue });
        await tx.wait();
      } else {
        // in fhEVM mode we must wait until the gateway service relayer submits the decryption fulfillment tx
        await waitNBlocks(1);
        await fulfillAllPastRequestsIds(mocked);
      }
    }
  }
};
