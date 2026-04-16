# Notifier library

## Goal

Integrate event consuming for specific logs and filters for zama components, and mimic the behaviour of a poller or a websocket stream, but from the listener standalone component.

## Logic:

This component consume blocks, transactions, receipts from the different queues declared on rabbitmq, checks into its table to see if there is relevant filters or abi to watch, or even "from" or "to" sources if we need to watch for some transactions, register logs in a table, and forward to the internal logic of the components that need logs.

Basically, the library is receipt parser.
You refer a watcher, and match the current watcher from the receipt if we need to get process an event.

For inspiration regarding this library, There is an existing implementation for logic, ask for access if needed.

## Features:

### General:

- Rust library.
- Resilient to HPA: (Ok by design: consuming from rmq)
- Clear api for subscribe and consume events from the rabbitmq.
- Find the ability to call the consume functions and pass hooks or handlers to trigger zama internal logic (should be easily integrable to existing components).
- Prepare strategy pattern for Solana.

### Queue consumer:

- Use Shared rmq library between listener, and notifier library, with retry queues, etc.

### Data storage:

- RDS database (Could leverage on rds for different components)
- postgres tables for logs and different watchers.

### Blockchain related:

#### Minimal features:

- Persist block height and resilient to failure mode.
- Multichain by design (consuming multiple queues (blocks, transactions with receipts -> e.g logs) for each networks).
- Should consume all events even if they are not used (or rmq memory will grow).
- Declare notifiers for dynamical events ABI
  - Store log watchers types into a postgres.
  - Store watched logs into a postgres.
  - Declare new watchers dynamically.

#### Should have features:

- Declare multiple watcher with a number of block confirmations if a block confirmation number is required (RPC url could be required for this). (e.g finality, safe or n confirmations blocks) based on events.
- Ability to be aware of new available chain from rabbitmq.
- different types of watchers (logs, tx)
- OPTIONAL: Cancel reorged events.
- OPTIONAL: Replay past blocks (should not need this with rmq, since its queuing messages).
- Check altogether if problems with duplicate logs, and how to manage them in the zama internals (could be handled optionally) To get a unicity regarding logs and handle deduplication, we can if needed apply a semantic hash regarding log.
- Metrics
- Alerting.

### Example:

- Should implement a minimal working example on how to use the library.

### Postgres table:

- table 1: watcher
  - uuid, chainId, number of conf block ?, ABI, watcher type (tx, contract)
- table 2: logs
  - uuid, watcher_uuid, block_number, released (TRUE, FALSE), log (deserialised or not), UNCLE? (Not mendatory if leverage on block confirmations)

## Schemes:
