import dotenv from 'dotenv';
import fs from 'fs';
import { ethers } from 'hardhat';

import { OraclePredeploy } from '../../types';
import { waitNBlocks } from './utils';

const network = process.env.HARDHAT_NETWORK;

const currentTime = (): string => {
  const now = new Date();
  return now.toLocaleTimeString('en-US', { hour12: true, hour: 'numeric', minute: 'numeric', second: 'numeric' });
};

const parsedEnv = dotenv.parse(fs.readFileSync('oracle/.env.oracle'));
const privKeyRelayer = process.env.PRIVATE_KEY_ORACLE_RELAYER;
const relayer = new ethers.Wallet(privKeyRelayer!, ethers.provider);

const argEvents =
  '(uint256 indexed requestID, uint256[] cts, address contractCaller, bytes4 callbackSelector, uint256 msgValue, uint256 maxTimestamp)';
const ifaceEventDecryptionEBool = new ethers.Interface(['event EventDecryptionEBool' + argEvents]);
const ifaceEventDecryptionEUint4 = new ethers.Interface(['event EventDecryptionEUint4' + argEvents]);
const ifaceEventDecryptionEUint8 = new ethers.Interface(['event EventDecryptionEUint8' + argEvents]);
const ifaceEventDecryptionEUint16 = new ethers.Interface(['event EventDecryptionEUint16' + argEvents]);
const ifaceEventDecryptionEUint32 = new ethers.Interface(['event EventDecryptionEUint32' + argEvents]);
const ifaceEventDecryptionEUint64 = new ethers.Interface(['event EventDecryptionEUint64' + argEvents]);

const argEvents2 = '(uint256 indexed requestID, bool success, bytes result)';
const ifaceResultCallbackBool = new ethers.Interface(['event ResultCallbackBool' + argEvents2]);
const ifaceResultCallbackUint4 = new ethers.Interface(['event ResultCallbackUint4' + argEvents2]);
const ifaceResultCallbackUint8 = new ethers.Interface(['event ResultCallbackUint8' + argEvents2]);
const ifaceResultCallbackUint16 = new ethers.Interface(['event ResultCallbackUint16' + argEvents2]);
const ifaceResultCallbackUint32 = new ethers.Interface(['event ResultCallbackUint32' + argEvents2]);
const ifaceResultCallbackUint64 = new ethers.Interface(['event ResultCallbackUint64' + argEvents2]);

let oracle: OraclePredeploy;
let firstBlockListening: number;

export const asyncDecrypt = async (): Promise<void> => {
  firstBlockListening = await ethers.provider.getBlockNumber();
  // this function will emit logs for every request and fulfilment of a decryption
  oracle = await ethers.getContractAt('OraclePredeploy', parsedEnv.ORACLE_CONTRACT_PREDEPLOY_ADDRESS);
  oracle.on(
    'EventDecryptionEBool',
    async (requestID, cts, contractCaller, callbackSelector, msgValue, maxTimestamp, eventData) => {
      const blockNumber = eventData.log.blockNumber;
      console.log(`${await currentTime()} - Requested ebool decrypt on block ${blockNumber} (requestID ${requestID})`);
    },
  );
  oracle.on('ResultCallbackBool', async (requestID, success, result, eventData) => {
    const blockNumber = eventData.log.blockNumber;
    console.log(`${await currentTime()} - Fulfilled ebool decrypt on block ${blockNumber} (requestID ${requestID})`);
  });
  oracle.on(
    'EventDecryptionEUint4',
    async (requestID, cts, contractCaller, callbackSelector, msgValue, maxTimestamp, eventData) => {
      const blockNumber = eventData.log.blockNumber;
      console.log(`${await currentTime()} - Requested euint4 decrypt on block ${blockNumber} (requestID ${requestID})`);
    },
  );
  oracle.on('ResultCallbackUint4', async (requestID, success, result, eventData) => {
    const blockNumber = eventData.log.blockNumber;
    console.log(`${await currentTime()} - Fulfilled euint4 decrypt on block ${blockNumber} (requestID ${requestID})`);
  });
  oracle.on(
    'EventDecryptionEUint8',
    async (requestID, cts, contractCaller, callbackSelector, msgValue, maxTimestamp, eventData) => {
      const blockNumber = eventData.log.blockNumber;
      console.log(`${await currentTime()} - Requested euint8 decrypt on block ${blockNumber} (requestID ${requestID})`);
    },
  );
  oracle.on('ResultCallbackUint8', async (requestID, success, result, eventData) => {
    const blockNumber = eventData.log.blockNumber;
    console.log(`${await currentTime()} - Fulfilled euint8 decrypt on block ${blockNumber} (requestID ${requestID})`);
  });
  oracle.on(
    'EventDecryptionEUint16',
    async (requestID, cts, contractCaller, callbackSelector, msgValue, maxTimestamp, eventData) => {
      const blockNumber = eventData.log.blockNumber;
      console.log(
        `${await currentTime()} - Requested euint16 decrypt on block ${blockNumber} (requestID ${requestID})`,
      );
    },
  );
  oracle.on('ResultCallbackUint16', async (requestID, success, result, eventData) => {
    const blockNumber = eventData.log.blockNumber;
    console.log(`${await currentTime()} - Fulfilled euint16 decrypt on block ${blockNumber} (requestID ${requestID})`);
  });
  oracle.on(
    'EventDecryptionEUint32',
    async (requestID, cts, contractCaller, callbackSelector, msgValue, maxTimestamp, eventData) => {
      const blockNumber = eventData.log.blockNumber;
      console.log(
        `${await currentTime()} - Requested euint32 decrypt on block ${blockNumber} (requestID ${requestID})`,
      );
    },
  );
  oracle.on('ResultCallbackUint32', async (requestID, success, result, eventData) => {
    const blockNumber = eventData.log.blockNumber;
    console.log(`${await currentTime()} - Fulfilled euint32 decrypt on block ${blockNumber} (requestID ${requestID})`);
  });
  oracle.on(
    'EventDecryptionEUint64',
    async (requestID, cts, contractCaller, callbackSelector, msgValue, maxTimestamp, eventData) => {
      const blockNumber = eventData.log.blockNumber;
      console.log(
        `${await currentTime()} - Requested euint64 decrypt on block ${blockNumber} (requestID ${requestID})`,
      );
    },
  );
  oracle.on('ResultCallbackUint64', async (requestID, success, result, eventData) => {
    const blockNumber = eventData.log.blockNumber;
    console.log(`${await currentTime()} - Fulfilled euint64 decrypt on block ${blockNumber} (requestID ${requestID})`);
  });
};

export const awaitAllDecryptionResults = async (): Promise<void> => {
  oracle = await ethers.getContractAt('OraclePredeploy', parsedEnv.ORACLE_CONTRACT_PREDEPLOY_ADDRESS);
  await fulfillAllPastRequestsIds(network === 'hardhat');
  firstBlockListening = await ethers.provider.getBlockNumber();
};

const getAlreadyFulfilledDecryptions = async (): Promise<[bigint]> => {
  let results = [];
  const eventDecryptionResultBool = await oracle.filters.ResultCallbackBool().getTopicFilter();
  const filterDecryptionResultBool = {
    address: process.env.ORACLE_CONTRACT_PREDEPLOY_ADDRESS,
    fromBlock: firstBlockListening,
    toBlock: 'latest',
    topics: eventDecryptionResultBool,
  };
  const pastResultsEbool = await ethers.provider.getLogs(filterDecryptionResultBool);
  results = results.concat(pastResultsEbool.map((result) => ifaceResultCallbackBool.parseLog(result).args[0]));

  const eventDecryptionResultEUint4 = await oracle.filters.ResultCallbackUint4().getTopicFilter();
  const filterDecryptionResultUint4 = {
    address: process.env.ORACLE_CONTRACT_PREDEPLOY_ADDRESS,
    fromBlock: firstBlockListening,
    toBlock: 'latest',
    topics: eventDecryptionResultEUint4,
  };
  const pastResultsEuint4 = await ethers.provider.getLogs(filterDecryptionResultUint4);
  results = results.concat(pastResultsEuint4.map((result) => ifaceResultCallbackUint4.parseLog(result).args[0]));

  const eventDecryptionResultEUint8 = await oracle.filters.ResultCallbackUint8().getTopicFilter();
  const filterDecryptionResultUint8 = {
    address: process.env.ORACLE_CONTRACT_PREDEPLOY_ADDRESS,
    fromBlock: firstBlockListening,
    toBlock: 'latest',
    topics: eventDecryptionResultEUint8,
  };
  const pastResultsEuint8 = await ethers.provider.getLogs(filterDecryptionResultUint8);
  results = results.concat(pastResultsEuint8.map((result) => ifaceResultCallbackUint8.parseLog(result).args[0]));

  const eventDecryptionResultEUint16 = await oracle.filters.ResultCallbackUint16().getTopicFilter();
  const filterDecryptionResultUint16 = {
    address: process.env.ORACLE_CONTRACT_PREDEPLOY_ADDRESS,
    fromBlock: firstBlockListening,
    toBlock: 'latest',
    topics: eventDecryptionResultEUint16,
  };
  const pastResultsEuint16 = await ethers.provider.getLogs(filterDecryptionResultUint16);
  results = results.concat(pastResultsEuint16.map((result) => ifaceResultCallbackUint16.parseLog(result).args[0]));

  const eventDecryptionResultEUint32 = await oracle.filters.ResultCallbackUint32().getTopicFilter();
  const filterDecryptionResultUint32 = {
    address: process.env.ORACLE_CONTRACT_PREDEPLOY_ADDRESS,
    fromBlock: firstBlockListening,
    toBlock: 'latest',
    topics: eventDecryptionResultEUint32,
  };
  const pastResultsEuint32 = await ethers.provider.getLogs(filterDecryptionResultUint32);
  results = results.concat(pastResultsEuint32.map((result) => ifaceResultCallbackUint32.parseLog(result).args[0]));

  const eventDecryptionResultEUint64 = await oracle.filters.ResultCallbackUint64().getTopicFilter();
  const filterDecryptionResultUint64 = {
    address: process.env.ORACLE_CONTRACT_PREDEPLOY_ADDRESS,
    fromBlock: firstBlockListening,
    toBlock: 'latest',
    topics: eventDecryptionResultEUint64,
  };
  const pastResultsEuint64 = await ethers.provider.getLogs(filterDecryptionResultUint64);
  results = results.concat(pastResultsEuint64.map((result) => ifaceResultCallbackUint64.parseLog(result).args[0]));

  return results;
};

const fulfillAllPastRequestsIds = async (mocked: boolean) => {
  const eventDecryptionEBool = await oracle.filters.EventDecryptionEBool().getTopicFilter();
  const results = await getAlreadyFulfilledDecryptions();
  const filterDecryptionEBool = {
    address: process.env.ORACLE_CONTRACT_PREDEPLOY_ADDRESS,
    fromBlock: firstBlockListening,
    toBlock: 'latest',
    topics: eventDecryptionEBool,
  };
  const pastRequestsEbool = await ethers.provider.getLogs(filterDecryptionEBool);
  for (const request of pastRequestsEbool) {
    const event = ifaceEventDecryptionEBool.parseLog(request);
    const requestID = event.args[0];
    const cts = event.args[1];
    const msgValue = event.args[4];
    if (!results.includes(requestID)) {
      // if request is not already fulfilled
      if (mocked) {
        // in mocked mode, we trigger the decryption fulfillment manually
        const tx = await oracle.connect(relayer).fulfillRequestBool(
          requestID,
          cts.map((value) => value === 1n),
          { value: msgValue },
        );
        await tx.wait();
      } else {
        // in fhEVM mode we must wait until the oracle service relayer submits the decryption fulfillment tx
        await waitNBlocks(1);
        await fulfillAllPastRequestsIds(mocked);
      }
    }
  }

  const eventDecryptionEUint4 = await oracle.filters.EventDecryptionEUint4().getTopicFilter();
  const filterDecryptionEUint4 = {
    address: process.env.ORACLE_CONTRACT_PREDEPLOY_ADDRESS,
    fromBlock: firstBlockListening,
    toBlock: 'latest',
    topics: eventDecryptionEUint4,
  };
  const pastRequestsEUint4 = await ethers.provider.getLogs(filterDecryptionEUint4);
  for (const request of pastRequestsEUint4) {
    const event = ifaceEventDecryptionEUint4.parseLog(request);
    const requestID = event.args[0];
    const cts = event.args[1];
    const msgValue = event.args[4];
    if (!results.includes(requestID)) {
      // if request is not already fulfilled
      if (mocked) {
        // in mocked mode, we trigger the decryption fulfillment manually
        const tx = await oracle.connect(relayer).fulfillRequestUint4(requestID, [...cts], { value: msgValue });
        await tx.wait();
      } else {
        // in fhEVM mode we must wait until the oracle service relayer submits the decryption fulfillment tx
        await waitNBlocks(1);
        await fulfillAllPastRequestsIds(mocked);
      }
    }
  }

  const eventDecryptionEUint8 = await oracle.filters.EventDecryptionEUint8().getTopicFilter();
  const filterDecryptionEUint8 = {
    address: process.env.ORACLE_CONTRACT_PREDEPLOY_ADDRESS,
    fromBlock: firstBlockListening,
    toBlock: 'latest',
    topics: eventDecryptionEUint8,
  };
  const pastRequestsEUint8 = await ethers.provider.getLogs(filterDecryptionEUint8);
  for (const request of pastRequestsEUint8) {
    const event = ifaceEventDecryptionEUint8.parseLog(request);
    const requestID = event.args[0];
    const cts = event.args[1];
    const msgValue = event.args[4];
    if (!results.includes(requestID)) {
      // if request is not already fulfilled
      if (mocked) {
        // in mocked mode, we trigger the decryption fulfillment manually
        const tx = await oracle.connect(relayer).fulfillRequestUint8(requestID, [...cts], { value: msgValue });
        await tx.wait();
      } else {
        // in fhEVM mode we must wait until the oracle service relayer submits the decryption fulfillment tx
        await waitNBlocks(1);
        await fulfillAllPastRequestsIds(mocked);
      }
    }
  }

  const eventDecryptionEUint16 = await oracle.filters.EventDecryptionEUint16().getTopicFilter();
  const filterDecryptionEUint16 = {
    address: process.env.ORACLE_CONTRACT_PREDEPLOY_ADDRESS,
    fromBlock: firstBlockListening,
    toBlock: 'latest',
    topics: eventDecryptionEUint16,
  };
  const pastRequestsEUint16 = await ethers.provider.getLogs(filterDecryptionEUint16);
  for (const request of pastRequestsEUint16) {
    const event = ifaceEventDecryptionEUint16.parseLog(request);
    const requestID = event.args[0];
    const cts = event.args[1];
    const msgValue = event.args[4];
    if (!results.includes(requestID)) {
      // if request is not already fulfilled
      if (mocked) {
        // in mocked mode, we trigger the decryption fulfillment manually
        const tx = await oracle.connect(relayer).fulfillRequestUint16(requestID, [...cts], { value: msgValue });
        await tx.wait();
      } else {
        // in fhEVM mode we must wait until the oracle service relayer submits the decryption fulfillment tx
        await waitNBlocks(1);
        await fulfillAllPastRequestsIds(mocked);
      }
    }
  }

  const eventDecryptionEUint32 = await oracle.filters.EventDecryptionEUint32().getTopicFilter();
  const filterDecryptionEUint32 = {
    address: process.env.ORACLE_CONTRACT_PREDEPLOY_ADDRESS,
    fromBlock: firstBlockListening,
    toBlock: 'latest',
    topics: eventDecryptionEUint32,
  };
  const pastRequestsEUint32 = await ethers.provider.getLogs(filterDecryptionEUint32);
  for (const request of pastRequestsEUint32) {
    const event = ifaceEventDecryptionEUint32.parseLog(request);
    const requestID = event.args[0];
    const cts = event.args[1];
    const msgValue = event.args[4];
    if (!results.includes(requestID)) {
      // if request is not already fulfilled
      if (mocked) {
        // in mocked mode, we trigger the decryption fulfillment manually
        const tx = await oracle.connect(relayer).fulfillRequestUint32(requestID, [...cts], { value: msgValue });
        await tx.wait();
      } else {
        // in fhEVM mode we must wait until the oracle service relayer submits the decryption fulfillment tx
        await waitNBlocks(1);
        await fulfillAllPastRequestsIds(mocked);
      }
    }
  }

  const eventDecryptionEUint64 = await oracle.filters.EventDecryptionEUint64().getTopicFilter();
  const filterDecryptionEUint64 = {
    address: process.env.ORACLE_CONTRACT_PREDEPLOY_ADDRESS,
    fromBlock: firstBlockListening,
    toBlock: 'latest',
    topics: eventDecryptionEUint64,
  };
  const pastRequestsEUint64 = await ethers.provider.getLogs(filterDecryptionEUint64);
  for (const request of pastRequestsEUint64) {
    const event = ifaceEventDecryptionEUint64.parseLog(request);
    const requestID = event.args[0];
    const cts = event.args[1];
    const msgValue = event.args[4];
    if (!results.includes(requestID)) {
      // if request is not already fulfilled
      if (mocked) {
        // in mocked mode, we trigger the decryption fulfillment manually
        const tx = await oracle.connect(relayer).fulfillRequestUint64(requestID, [...cts], { value: msgValue });
        await tx.wait();
      } else {
        // in fhEVM mode we must wait until the oracle service relayer submits the decryption fulfillment tx
        await waitNBlocks(1);
        await fulfillAllPastRequestsIds(mocked);
      }
    }
  }
};
