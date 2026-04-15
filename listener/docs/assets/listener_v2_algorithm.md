# Listener Core - Algorithm V2 (Cursor Algorithm)

## Overview

The Cursor Algorithm solves the block production speed problem by parallelizing block fetching while maintaining sequential validation through a cursor.

## Architecture Diagram

```mermaid
flowchart TB
    subgraph INIT["Initialization"]
        DB[(Database)] -->|"Last canonical block hash"| TIP[Chain Tip Hash]
        RPC[RPC Node] -->|"eth_blockNumber"| HEIGHT[Current Height]
        TIP --> RANGE[Calculate Block Range]
        HEIGHT --> RANGE
    end

    subgraph TASK1["Task 1: Parallel Poller"]
        RANGE -->|"spawn N tasks"| P1[Poller 1]
        RANGE --> P2[Poller 2]
        RANGE --> P3[Poller 3]
        RANGE --> POLLER_N[Poller N]

        P1 -->|"fetch block by number + receipts + compute"| RPC1[RPC Call]
        P2 -->|"fetch block by number + receipts + compute"| RPC2[RPC Call]
        P3 -->|"fetch block by number + receipts + compute"| RPC3[RPC Call]
        POLLER_N -->|"fetch block by number + receipts + compute"| RPCN[RPC Call]

        RPC1 -->|"set_once(0)"| BUFFER
        RPC2 -->|"set_once(1)"| BUFFER
        RPC3 -->|"set_once(2)"| BUFFER
        RPCN -->|"set_once(N)"| BUFFER
    end

    subgraph BUFFER_ZONE["AsyncSlotBuffer (In-Memory)"]
        BUFFER[/"Slot 0 | Slot 1 | Slot 2 | ... | Slot N"/]
    end

    subgraph TASK2["Task 2: Cursor (Sequential Validator)"]
        BUFFER -->|"get(i) - waits if empty"| CURSOR[Cursor]
        CURSOR -->|"compare hashes"| CHECK{parent_hash == prev_hash?}

        CHECK -->|"YES"| BROADCAST[Broadcast to Message Broker]
        BROADCAST --> NEXT[Move to next slot]
        NEXT -->|"i++"| CURSOR

        CHECK -->|"NO - REORG!"| REORG[Reorg Handler]
    end

    subgraph REORG_HANDLING["Reorg Handling"]
        REORG -->|"1. Stop cursor"| STOP[Halt Progression]
        STOP -->|"2. Backtrack by hash"| BACKTRACK[Fetch blocks + receipt + compute by hash]
        BACKTRACK -->|"3. Mark old as UNCLE"| UNCLE[Update Status]
        UNCLE -->|"4. Restart from block N"| RESTART[New Iteration]
        RESTART --> RANGE
    end

    subgraph OUTPUT["Output"]
        BROADCAST -->|"blocks"| QUEUE1[Block Queue]
        BROADCAST -->|"transactions + receipts"| QUEUE2[Transaction Queue]
    end

    NEXT -->|"buffer exhausted"| NEWITER[Start Next Iteration]
    NEWITER -->|"keep last block as tip"| RANGE
```

## Sequence Diagram

```mermaid
sequenceDiagram
    participant DB as Database
    participant Main as Main Loop
    participant Pollers as Parallel Pollers
    participant Buffer as AsyncSlotBuffer
    participant Cursor as Cursor
    participant Broker as Message Broker

    Main->>DB: Get last canonical block hash
    DB-->>Main: chain_tip_hash
    Main->>Main: Calculate range [start, start+batch_size]

    par Parallel Block Fetching
        Main->>Pollers: Spawn N parallel tasks
        Pollers->>Buffer: set_once(0, block_0) [arrives T+500ms]
        Pollers->>Buffer: set_once(1, block_1) [arrives T+100ms]
        Pollers->>Buffer: set_once(2, block_2) [arrives T+800ms]
        Note over Pollers,Buffer: Blocks arrive in random order
    end

    loop Sequential Validation
        Cursor->>Buffer: get(i) - blocks until filled
        Buffer-->>Cursor: block_i

        alt parent_hash matches
            Cursor->>Cursor: Validate hash chain
            Cursor->>Broker: Broadcast block + txs
            Cursor->>Cursor: i++, prev_hash = block.hash
        else REORG DETECTED
            Cursor->>Main: Signal reorg at block N
            Main->>Main: Backtrack by hash
            Main->>DB: Mark old blocks as UNCLE
            Main->>Main: Restart iteration from N
        end
    end

    Cursor->>Main: Buffer exhausted
    Main->>Main: Next iteration (keep last block)
```

## Component State Diagram

```mermaid
stateDiagram-v2
    [*] --> Initializing: Start

    Initializing --> Polling: Calculate range

    Polling --> Filling: Spawn parallel fetchers
    Filling --> Filling: Blocks arriving (random order)

    Filling --> Validating: Cursor starts reading

    Validating --> Validating: Hash matches, broadcast
    Validating --> ReorgDetected: Hash mismatch!

    ReorgDetected --> Backtracking: Stop cursor
    Backtracking --> Backtracking: Fetch by hash
    Backtracking --> Polling: Canonical chain rebuilt

    Validating --> Polling: Buffer exhausted, next batch
```

## Key Components

| Component           | Status         | Description                                            |
| ------------------- | -------------- | ------------------------------------------------------ |
| `AsyncSlotBuffer`   | ✅ Implemented | Thread-safe buffer for parallel write, sequential read |
| `EvmBlockFetcher`   | ✅ Implemented | 5 strategies for fetching blocks + receipts            |
| `EvmBlockComputer`  | ✅ Implemented | Block hash verification (receiptRoot, txRoot)          |
| `SemEvmRpcProvider` | ✅ Implemented | Semaphore-controlled RPC provider                      |
| Cursor Loop         | ⚠️ Simulated   | `slot_buffer_sim_flow.rs` demo                         |
| Reorg Handler       | ❌ Not Yet     | Backtracking logic                                     |
| Message Broker      | ❌ Not Yet     | Redis Streams / RabbitMQ integration                   |
| Database Layer      | ❌ Not Yet     | Block status persistence (CANONICAL/UNCLE)             |

## Algorithm Properties

- **Parallel Fetching**: Overcomes HTTP latency for fast chains (Arbitrum, Monad)
- **Sequential Validation**: Cursor ensures hash chain integrity
- **Reorg Safe**: Detects reorgs via parent_hash comparison
- **Cancellation Support**: CancellationToken propagates through all tasks
- **Rate Limit Friendly**: Semaphore controls concurrent RPC calls
