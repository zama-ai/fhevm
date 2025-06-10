# KMS Connector Architecture

## Introduction

The role of the KMS Connector is to forward Gateway's events to the KMS Core and the responses of
the KMS Core to the Gateway.

The ambition of `fhevm` is to be able to handle thousands of decryptions per second. If the KMS
Connector does not play its role, it would break the whole `fhevm` flow, so we must ensure that:
- it never misses any Gateway events
- it never misses any KMS Core responses
- it is able to catch up events/responses in case it has been down for some time

## Architecture overview

In order to achieve this, the KMS Connector has been divided into 3 components:
- **GatewayListener**
  - Multiple listeners, so we do not miss Gateway events if one is down
  - Each listener listens to a RPC node of the Gateway
    - Each listener can have backup RPC nodes URL in case the connection with the first one is lost
  - Each listener tries to write the events it catches in a Postgres DB (only 1 will succeed, the other will ignore the duplicate key error)
  - The number of listeners should be able to scale up to avoid missing events but also to scale down to not overspend resources when it is not required
 
- **KmsWorker**
  - One or multiple workers
    - In the first place, we will probably start with only one worker
    - But we should be able to scale the number of worker to handle more events if required
  - Get notified by the Postgres DB when new events are stored
  - Forward the events' requests to the KMS Core, and store its responses to DB
  - Remove the events from the DB once handled

- **TransactionSender**
  - Only one tx sender
  - Gets notified by the Postgres DB when new KMS Core responses are stored
  - Forwards the KMS Core response to the Gateway by submitting transactions
  - Removes the responses from the DB once forwarded

Here is an overview of the architecture:

```mermaid
block-beta
    columns 5

    block:gateway:5
        columns 3
        space title1("Gateway L3") space
        r1["RPC node 1"] r2["RPC node 2"] r3["RPC node 3"]
    end

    space:5
    block:connector:5
        columns 5
        space:2 title2("KMS Connector") space:2
        l1["GatewayListener"]
        l2["GatewayListener"]
        l3["GatewayListener"]
        w["KmsWorker"]
        txs["TransactionSender"]
    end
    space:5

    l1 -- "Listen events" --> r1
    l2 -- "Listen events" --> r2
    l3 -- "Listen events" --> r3

    db[("DB")]:4 kms["KMS Core"]
    l1 -- "Put \n events" --> db
    l2 -- "Put \n events" --> db
    l3 -- "Put \n events" --> db

    w -- "Pick events \n & \n Put tx" --> db
    w -- "GRPC" --> kms
    kms -- "GRPC" --> w
    
    txs -- "\n Pick \n tx" --> db
    txs -- "Send tx" --> r3

    class title1 BT1
    classDef BT1 stroke:transparent,fill:transparent
    class title2 BT2
    classDef BT2 stroke:transparent,fill:transparent
```

## KMS Connector flow

Here is an overview of the KMS Connector flow from the Gateway request event emitted to the
transaction response:

```mermaid
flowchart LR
    Gw[Gateway L3] -->|emit event| L[GatewayListener]
    L -->|insert event| DB[(Postgres)]
    DB -->|trigger notification| W[KmsWorker]
    W -->|GRPC request| C[KmsCore]
    C -->|GRPC response| W
    W -->|insert response| DB
    DB -->|trigger notification| T[TransactionSender]
    T -->|tx response| Gw
```

## Database design

Each event emitted by the Gateway or response received from the KMS Core has its table
representation in the DB, with a notification triggered when data is inserted within this table.

The `KmsWorker` and `TransactionSender` listen to these notifications and perform their jobs when
a notification is received.

Example with `PublicDecryptionResponse` received from the KMS Core:

```sql
CREATE TABLE IF NOT EXISTS public_decryption_responses (
    decryption_id BYTEA NOT NULL,
    decrypted_result BYTEA NOT NULL,
    signatures BYTEA NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (decryption_id)
);

CREATE OR REPLACE FUNCTION notify_public_decryption_response()
    RETURNS trigger AS $$
BEGIN
    NOTIFY public_decryption_response_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_from_public_decryption_responses_insertions
    AFTER INSERT
    ON public_decryption_responses
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_public_decryption_response();
```

## Reliability

### Never miss Gateway events

As we will run multiple `GatewayListener` instances, we assume that they will not crash all
simultaneously, thus that all events emitted by the Gateway would be written in the DB.

So even if we run only one `KmsWorker` which crashes, it will have access to unhandled events when
restarted.

### Never miss KMS Core responses

Until the `KmsWorker` has received a response from the KMS Core, the associated event request will
stay in the DB. So if the `KmsWorker` or KMS Core crashes before the KMS Core responds, the
`KmsWorker` will be able to re-pick the event from the DB and re-send the request when the
connection with the KMS Core is re-established. Thus, no KMS Core responses should be missed.

### Catch up events/responses after downtime

The DB notifications are used to handle events/responses while the KMS Connector's components are
running. But what about events/responses that happened while the `KmsWorker` or the
`TransactionSender` was down?

For the `KmsWorker`, when the connections to the DB and to the KMS Core are both (re-)established,
it will check in the DB if there are previous events to handle, and will process them if so.

For the `TransactionSender`, when the connections to the DB and to a RPC node of the Gateway are
both (re-)established, it will check in the DB if there are previous responses to submit via
transactions, and executes them if so.
