import { ethers } from "hardhat";
import type { Provider } from "ethers";

const currentTime = (): string => {
  const now = new Date();
  return now.toLocaleTimeString("en-US", {
    hour12: true,
    hour: "numeric",
    minute: "numeric",
    second: "numeric",
  });
};

const POLLING_INTERVAL = 1100; // Blocktime is 1s, so we use a slightly bigger polling time

let lastProcessedBlock = 0;
const pendingDecryptionRequestCounters = new Set<bigint>();
const pendingDecryptionRequestParameters = new Map<bigint, [string, bigint]>();

// Event definitions
const decryptionRequestEventFragment =
  "event DecryptionRequest(uint256 indexed counter, uint256 requestID, bytes32[] cts, address contractCaller, bytes4 callbackSelector)";
const ifaceRequest = new ethers.Interface([decryptionRequestEventFragment]);
const topicHashRequest = ifaceRequest.getEvent("DecryptionRequest")!.topicHash;

const decryptionFulfillEventFragment =
  "event DecryptionFulfilled(uint256 indexed requestID)";
const ifaceFulfill = new ethers.Interface([decryptionFulfillEventFragment]);
const topicHashFulfill = ifaceFulfill.getEvent(
  "DecryptionFulfilled"
)!.topicHash;

// Initialize by starting the polling of eth_getLogs (more robust solution than using websocket to avoid missng events on anvil)
export const initDecryptionOracle = async (): Promise<void> => {
  lastProcessedBlock = await ethers.provider.getBlockNumber();
  setInterval(pollEvents, POLLING_INTERVAL);
};

async function pollEvents(): Promise<void> {
  const currentBlock = await ethers.provider.getBlockNumber();

  // If no new blocks, skip this polling cycle
  if (currentBlock <= lastProcessedBlock) {
    return;
  }

  const requestLogs = await getDecryptionRequestLogs(
    lastProcessedBlock + 1,
    currentBlock
  );
  const fulfillLogs = await getDecryptionFulfillLogs(
    lastProcessedBlock + 1,
    currentBlock
  );

  processDecryptionRequests(requestLogs);

  processDecryptionFulfillments(fulfillLogs);

  lastProcessedBlock = currentBlock;
}

async function getDecryptionRequestLogs(
  fromBlock: number,
  toBlock: number
): Promise<ethers.Log[]> {
  const filterOracle = {
    address: process.env.DECRYPTION_ORACLE_ADDRESS!,
    topics: [topicHashRequest],
    fromBlock,
    toBlock,
  };

  return ethers.provider.getLogs(filterOracle);
}

async function getDecryptionFulfillLogs(
  fromBlock: number,
  toBlock: number
): Promise<ethers.Log[]> {
  const filterFulfill = {
    topics: [topicHashFulfill],
    fromBlock,
    toBlock,
  };

  return ethers.provider.getLogs(filterFulfill);
}

function processDecryptionRequests(logs: ethers.Log[]): void {
  for (const log of logs) {
    const parsed = ifaceRequest.parseLog({
      topics: log.topics as string[],
      data: log.data,
    });

    if (!parsed) continue;

    const { counter, requestID, cts, contractCaller, callbackSelector } =
      parsed.args;

    console.log(
      `${currentTime()} - Requested public decryption on block ${
        log.blockNumber
      } ` + ` (counter ${counter} - requestID ${requestID})`
    );

    pendingDecryptionRequestCounters.add(counter);
    pendingDecryptionRequestParameters.set(counter, [
      contractCaller,
      requestID,
    ]);
  }
}

function processDecryptionFulfillments(logs: ethers.Log[]): void {
  for (const log of logs) {
    const parsed = ifaceFulfill.parseLog({
      topics: log.topics as string[],
      data: log.data,
    });

    if (!parsed) continue;

    const { requestID } = parsed.args;
    const emitterAddress = log.address;

    // find which counter(s) this maps to
    for (const [counter, [caller, id]] of pendingDecryptionRequestParameters) {
      if (id === requestID && caller === emitterAddress) {
        pendingDecryptionRequestCounters.delete(counter);
        pendingDecryptionRequestParameters.delete(counter);
        console.log(
          `${currentTime()} - Fulfilled public decryption on block ${
            log.blockNumber
          } ` + ` (counter ${counter} - requestID ${requestID})`
        );
      }
    }
  }
}

export const awaitAllDecryptionResults = async (): Promise<void> => {
  // WARNING: if the callback reverts, this function will timeout
  // TODO: to avoid this issue, a solution is to add an http endpoint on the relayer to know if the callback reverted,
  // since this cannot be detected onchain with new oracle design (fulfill now happens by calling directly the dapp contract)

  // Force one last poll to ensure we have the latest data
  await waitOneBlock(ethers.provider); // wait one block to avoid race condition if some previous request was not confirmed
  await pollEvents();

  // if nothing pending, return immediately
  if (pendingDecryptionRequestCounters.size === 0) {
    console.log(`${currentTime()} - No pending decryption requests.`);
    return;
  }

  // otherwise poll every 100ms until the Set is emptied by the event-listener
  console.log(
    `${currentTime()} - Waiting for ${pendingDecryptionRequestCounters.size}` +
      ` pending decryption request(s) to be fulfilled...`
  );
  while (pendingDecryptionRequestCounters.size > 0) {
    await new Promise((resolve) => setTimeout(resolve, 100));
  }

  if (pendingDecryptionRequestCounters.size === 0) {
    console.log(`${currentTime()} - All decryption requests fulfilled.`);
  }
};

async function waitOneBlock(provider: Provider): Promise<void> {
  await new Promise<void>((resolve) => {
    provider.once("block", () => {
      resolve();
    });
  });
}
