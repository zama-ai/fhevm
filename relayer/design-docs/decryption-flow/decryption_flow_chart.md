# Decryption flow chart

```mermaid
flowchart TD
    start[Incoming Event] --> eventHandler[handle_event]
    
    subgraph Event Processing
        eventHandler --> eventMatch{Event Type?}
        
        eventMatch -->|DecryptRequestRcvd| decryptReq[send_decryption_request_to_rollup]
        eventMatch -->|DecryptResponseEventLog| responseLog[handle_decrypt_response_event_log]
        eventMatch -->|DecryptionRequestSent| requestSent[handle_decrypt_request_sent]
        eventMatch -->|Other| noop[noop_handle_decrypt_response_event_log]
    end
    
    subgraph Decryption Request Flow
        decryptReq --> prepHandles[Convert handles to Uint256]
        prepHandles --> spawnTask[Spawn async task]
        spawnTask --> processReq[process_decryption_request]
        
        processReq --> txHelper[TransactionHelper.send_transaction]
        txHelper --> receipt{Transaction Success?}
        
        receipt -->|Yes| extractId[Extract decryption_id]
        receipt -->|No| handleFail[handle_failed_request]
        
        extractId --> handleSuccess[handle_successful_request]
        
        handleSuccess --> storeMapping[Store decryption_id mapping]
        storeMapping --> dispatchNext[Dispatch DecryptionRequestSentToGwL2]
    end
    
    subgraph Response Processing
        responseLog --> sleep[Artificial delay]
        sleep --> extractEventId[extract_decryption_id_from_event]
        extractEventId --> lookupId{Found Request ID?}
        
        lookupId -->|Yes| createResponse[Create Response Event]
        lookupId -->|No| logError[Log Error]
        
        createResponse --> dispatchResponse[Dispatch DecryptionResponseRcvdFromGwL2]
    end
    
    subgraph State Management
        storeMapping -.->|Update| idMap[(decryption_id_to_request_id)]
        lookupId -.->|Read| idMap
    end

    classDef error fill:#4f1919,color:#ff9999    %% Darker red with lighter text
    classDef success fill:#1a3d1a,color:#90EE90   %% Darker green with lighter text
    classDef process fill:#1a1a3d,color:#9999ff   %% Darker blue with lighter text
    classDef state fill:#3d3d1a,color:#ffff99    %% Darker yellow with lighter text
    
    class handleFail,logError error
    class handleSuccess,dispatchNext,dispatchResponse success
    class processReq,txHelper,extractId process
    class idMap state
    ```
