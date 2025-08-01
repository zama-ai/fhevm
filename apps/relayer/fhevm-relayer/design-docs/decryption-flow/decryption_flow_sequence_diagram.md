# Decryption flow

In this document, we try to describe the decryption flow within the code and among all the different event types. 

```mermaid
sequenceDiagram
    participant L1 as Ethereum L1
    participant EL as ethereum_listener
    participant EH as ethereum_host_l1_handlers
    participant AGH as arbitrum_gw_l2_handler
    participant RL as rollup_listener

    Note over L1,L2: Decryption Flow
    
    L1->>EL: Emit Decryption request Event
    Note over EL: Emit relayerEvent:PubDecryptEventLogRcvdFromHostL1
    EL->>EH: PubDecryptEventLogRcvdFromHostL1
    Note over EH: Decode event
    Note over EH: Store (l1_req_id, contractCaller, selector)
    Note over EH: Create new request ID 0x123
    Note over EH: Emit relayerEvent:DecryptRequestRcvd
    
    EH->>AGH: DecryptRequestRcvd<br/>(ct_handles, operation_type)
    AGH->>L2: Submit Transaction
    Note over AGH: Extract public_decryption_id 0xAA from receipt
    Note over AGH: Insert (public_decryption_id, original_event_request_id)
    Note over AGH: Insert (0xAA, 0x123)
    Note over AGH: Emit relayerEvent:???<br/>relayerEvent:DecryptionReqToGwL2Sent
    
    L2->>RL: Emit Decryption Response Event
    Note over RL: Retrieve public_decryption_id (0xAA)
    Note over RL: Read associated event request id, read_uuid(0xAA) = 0x123
    Note over RL: Emit relayerEvent:DecryptResponseEventLogRcvdFromGwL2
    RL->>EH: DecryptResponseEventLogRcvdFromGwL2<br/>(ptx, sig)<br/>(event.request_id: 0xAA)
    Note over EH: Retrieve from contextual data
    Note over EH: req.contract_caller, req.selector
    EH->>L1: Submit callback transaction to contract caller
    Note over L1: Read plaintext and check signatures
    Note over L1,L2: End of Flow
```
