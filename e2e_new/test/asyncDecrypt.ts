import { ethers } from "hardhat";

const currentTime = (): string => {
  const now = new Date();
  return now.toLocaleTimeString("en-US", {
    hour12: true,
    hour: "numeric",
    minute: "numeric",
    second: "numeric",
  });
};

let firstBlockListening: number;

export const initDecryptionOracle = async (): Promise<void> => {
  firstBlockListening = await ethers.provider.getBlockNumber();

  const decryptionEventFragment =
    "event DecryptionRequest(uint256 indexed counter, uint256 requestID, bytes32[] cts, address contractCaller, bytes4 callbackSelector)";
  const iface = new ethers.Interface([decryptionEventFragment]);

  const topicHash = iface.getEvent("DecryptionRequest")!.topicHash;
  const filter = {
    address: process.env.DECRYPTION_ORACLE_ADDRESS!,
    topics: [topicHash],
  };

  ethers.provider.on(filter, async (log) => {
    const parsed = iface.parseLog(log);
    const { counter, requestID, cts, contractCaller, callbackSelector } =
      parsed!.args;

    console.log(
      `${currentTime()} - Requested decrypt on block ${log.blockNumber} ` +
        `(counter ${counter} - requestID ${requestID})`
    );
  });
};

export const awaitAllDecryptionResults = async (): Promise<void> => {
  firstBlockListening = (await ethers.provider.getBlockNumber()) + 1;
  await new Promise((resolve) => setTimeout(resolve, 20000));
};
