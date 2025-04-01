# Public Decryption

Public decryption is when a contract calls the ZWS Public Decryption Oracle to get a ciphertext decrypted publicly on-chain.

<!-- TODO: Add transaction receipt flow from tx-manager to relayer to console to payment -->

[HTTPZ Flow](https://github.com/zama-ai/tech-spec/blob/main/architecture/public_decryption.md)

## Flow


- Some Contract calls the ZWS Decryption Oracle Contract
    - The Oracle contract checks if said caller is allowed to call the endpoint otherwise reverts
    - If ok it emits a decryption-request event
- The L1-event-listener catches the decryption-request events emitted by the Decryption Oracle Contract
    - Potential errors:
        - The event is missed because the service is down
            - Need some block number persistent state to recover (that could be set manually potentially)
        - The software is out-of-date with the contract event spec
            - Need to keep track of failed events if possible
        - Re-orgs
            - ?
- The L1-event-listener sends a message to the SNS queue
    - Potential errors:
        - Communication error with the SNS queue
            - Retry mechanism and cache system for failed messages?
- The orchestrator gets the message from the SNS queue
    - Potential errors
        - Communication error with the SNS queue
            - Retry mechanism
        - Message parsing error
            - Make sure that we have at least the tx hash to be able to recover?
- The orchestrator checks if the ciphertext is available on the L2 (probably through the L2-listener ?)
    - Should probably be done with a specific service?
    - Should we also check if the caller as access or is this done on the L1?
    - Potential errors
        - The orchestrator can’t reach the L2
- The orchestrator checks if the ciphertext is plublicly decryptable
    - allow was called on the acl of the l1
        - if not then we end the flow here
    - i.e. all coprocessors reached consensus on the acl of the L2
        - if not then we retry until it’s done
        - potential problem: faulty coprocessors never reach consensus
- The orchestrator calls the L2-tx-maker(/factory in the case the tx isn’t emitted from there) to create the payload associated with the request
    - Potential problems:
        - Communication
- The orchestrator calls the wallet-manager to sign the payload with a HTTPZ ZWS wallet (and optionally make the transaction or then forward the payload to the L2-tx-maker)
    - Potential problems:
        - Nonce management
        - Gas management
        - A mix of both (i.e. tx with not enough gas stuck blocking following transactions)
        - Communication with the L2
- The L2-event-listener catches the result event and pushes the result to the orchestrator via SNS
    - Potential problems
        - The result event never happens because of an issue with the KMS
        - The L2 misses a block thus the result event (implement recoverability)
        - SNS communication issue
        - Mostly same problem as the other listener
- The orchestrator calls the L1-tx-maker(/factory) to make the payload of the tx with the result + the callback info from the L1 event
- The orchestrator calls the wallet-manager to sign the payload with L1 ZWS wallet (and optionally make the transaction or then forward the payload to the L1-tx-maker)
    - That means from my understanding that we’ll need to make sure that the contract that called the public decryption MUST have an endpoint where to push the decrypted result.
