// Max block range for eth_getLogs (most providers limit to 50k)
const MAX_BLOCK_RANGE = 49999;

async function queryEventsInChunks(contract, filter, fromBlock, toBlock, label) {
  const events = [];
  let currentFrom = fromBlock;

  while (currentFrom <= toBlock) {
    const currentTo = Math.min(currentFrom + MAX_BLOCK_RANGE, toBlock);
    const progress = Math.round(((currentFrom - fromBlock) / (toBlock - fromBlock)) * 100) || 0;
    process.stdout.write(`\r    ${label}: ${progress}% (block ${currentFrom})...`);

    const chunk = await contract.queryFilter(filter, currentFrom, currentTo);
    events.push(...chunk);

    currentFrom = currentTo + 1;
  }

  console.log(`\r    ${label}: 100% - found ${events.length} events`);
  return events;
}

module.exports = { MAX_BLOCK_RANGE, queryEventsInChunks };
