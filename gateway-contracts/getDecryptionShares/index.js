const ethers = require("ethers");
const { writeFile } = require("fs/promises");

async function saveEvents(events) {
  const filePath = `UserDecryptionResponseExample.json`;
  await writeFile(filePath, JSON.stringify(events, null, 2), "utf8");
  console.log(`Saved event(s) to: ${filePath}`);
}

// RPC can be any network where the tx was sent
const RPC_URL = "https://rpc-zama-oft-devnet-d3losclbqm.t.conduit.xyz";
const provider = new ethers.JsonRpcProvider(RPC_URL);

// If you're inside Hardhat, you can use: const provider = ethers.provider;

const ABI = [
  `event UserDecryptionResponse(
      uint256 indexed decryptionId,
      bytes[] userDecryptedShares,
      bytes[] signatures,
      bytes extraData,
   )`,
];

const iface = new ethers.Interface(ABI);
// Topic for quick filtering (keccak256 of the event signature)
const EVENT_TOPIC = ethers.id(
  "UserDecryptionResponse(uint256,bytes[],bytes[],bytes)"
);

async function getUserDecryptionResponses(txHash) {
  const receipt = await provider.getTransactionReceipt(txHash);
  if (!receipt) {
    throw new Error("Transaction not found or not yet mined");
  }

  const results = [];

  for (const log of receipt.logs) {
    // Fast pre-filter by topic[0]
    if (log.topics[0].toLowerCase() !== EVENT_TOPIC.toLowerCase()) continue;

    // Parse the log using the interface
    const parsed = iface.parseLog({ topics: log.topics, data: log.data });
    if (parsed?.name !== "UserDecryptionResponse") continue;

    const { decryptionId, userDecryptedShares, signatures } = parsed.args;

    results.push({
      txHash: txHash,
      emitter: log.address,
      decryptionId: decryptionId.toString(),
      userDecryptedShares,
      signatures,
    });
  }
  await saveEvents(results);
  return results;
}

(async () => {
  const events = await getUserDecryptionResponses(
    "0x6a10b7a2b11ad7f037903a00ac8af99e52a2816a224723f198553537520ede2f"
  );
  console.dir(events, { depth: null });
})().catch((err) => {
  console.error(err);
  process.exit(1);
});
