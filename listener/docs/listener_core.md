# Listener Core

## Problematics

- TOP PRIORITY: Miss zero events.
- Simple Reorg: Parent hash different previous hash
- Back and forth reorgs: reorged branch becomes canonical again
- Multi branching reorgs: Multiple reorgs can be detected at the same time, and only one will become canonical.
- Must be trustless as much as possible regarding the RPC node we are hitting on, or the load balancer we are hitting on that serves multiple nodes
- Caching problem, corrupted data etc...

## Common knowledge:

- Counter-intuitively, it is always the fresher information that will bring us the truth, especially regarding the past.
- Transaction receipts contains all the logs
- ReceiptRoot calculation, and block hash calculation ensure there is no missing logs for a given block.
- Zero websocket, not resilient.

## Goal

The goal of this core algorithm is to fetch (http polling), blocks, transactions, receipt, by polling, handle reorogs properly by checking hash and parent hash, fetch new information if needed to be consistent and aware of canonical chain and broadcast blocks, and transaction to the message broker for the library to be aware of new events.

## Logic / Algorithms

### Algorithm v1: Sequential poller and reorg checker

This is a descriptive of a basic algorithm, which could be sufficient with chains that produces blocks in more time than an http call duration.
This algorithm is sequential, and is just referred here for knowledge.

If you need access to an existing implementation of this algorithm, ask and I will share you the implementation.

This algorithm leverages mostly on database, to perform checks, states updates, and branching.

1. polling loop for getting the next block
2. we register block, transactions of this block, and associated receipt.
   1. The receipt contains all the logs.
   2. We broadcast the block, and the transactions with receipts to given queues with chainId, to get almost real time performance, for being consumed and filtered by the library notifier over abi filter and contract address
3. we compare current block parent hash, and previous block parent hash to detect if a reorg occurred.
   1. if it matches, we go back to the beginning of the algorithm.
   2. if it didn't match: Reorg is detected.
      1. we fetch one by one all the previous blocks by hash, we broadcast events in the same fashion we did previously. (BACKTRACKING)
      2. pass the other ones to UNCLES status.
      3. Optionally, we broadcast cancelation events for old blocks for the library, but its not needed
      4. Then we go to 1. with the next iteration to fetch new blocks.

### Algorithm v2: Cursor Algorithm

The major flaw with the v1 iterative poller algorithm, is the block production time for faster chains, such as Arbitrum, Monad, or even Solana later could be faster than a single http response call, database operations, and network time calls if levraging on rabbitmq to trigger block fetch and polling operations, cumulated operations could be more than 100/200ms only in average time. It does not keep up with chain with a smaller block time duration.
Also, if later a full chain indexer is needed, this is impossible to leverage on the first algorithm.

Here is the proposed algorithm to address this flaw.

#### task one: parallel poller

Resolving the http latency, and ensure no event is missed.

1. We calculate a block range:
   1. `min(blockHeight - currentRegisteredBlock, maxParallelBlockFetch + currentRegisteredBlock)`
   2. or range given from an order to fetch the next block.
2. We spawn parallel task to fetch blocks (http polling), and register them in an in memory datastructure (slots for new blocks). And we fetch receipt for those blocks (strategy pattern could be required for diffrernt chains implems (`eth_getBlockReceipts` or `eth_getTransactionReceipt`for each transaction))
3. Optional: we recompute block hash: The rationale behind this, calculate receiptRoot and then block hash from receipt root and all other headers: this ensure that there is no inconsistency in receipts, hence logs contained in the receipts.

#### task two: cursor, reorg check and event broadcaster

1. The cursor, check the data structure up to its length, comparing current parent hash and previous hash, and wait if there is no block yet available at a certain slot to continue its progression.
   1. we broadcast blocks and transaction to the message broker.
2. if no reorg, continue and check the next block with strategy described in 1.
3. if a reorg is detected
   1. the cursor stops its progression.
   2. we backtrack from block `n` to handle the reorg and get matching blocks by hash sequentially until the canonical chain is build again, we pass previous data to uncle (refer to algo 1) in a new data structure.
   3. and we start a new task one to get fresher information from block `n`, and a new cursor strategy on top of this one.

4. When the cursor arrives to the end of the data structure, it launches the next iteration for the task one parallel poller, and keep at least the latest block from the previous iteration to compare hash and parent hash.

## Features

- Data structure for getting in memory blocks
- Event driven system to react to multiple events for the algorithm described above.
- strategy pattern (handling chains that doesn't support `eth_getBlockReceipts` method, and solana later)
- algorithm v2 implementation with eth_getBlockReceipt first.
- tables to store minimal metadata (blocks, transactions and receipts) with status (CANONICAL, UNCLE).
- Sql cleaner feature.
- OPTIONAL: Finalized status.
- Block computer.
- OPTIONAL: data storage layer no-sql or S3.
- push rabbitmq messages.

## Additional: 



## Schemes
