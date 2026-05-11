# Identifiers used in the relayer

## Overview

This document describes different IDs used in the relayer. There are 4 identifiers

1. Internal Request ID
2. External Request ID
3. Internal Public/User Decryption Indexer IDs
4. External Reference ID
5. Gateway Public/User Decryption ID
6. Gateway Input Proof ID

## Description

### Internal Input Proof ID

- **Purpose**: Track async parts of a HTTP Request internally in the relayer.
- **Format**: `UUID v7` for its time sortable property with strong uniqueness guarantee.
- **Creation**: By HTTP handler in application on every HTTP request (`v2 /POST`, `v2 /GETs`).
- **Relationship**: Mapped `1:1` to *External Request ID* by Relayer.
- **Visibility for user**: :x: No
- **Storage:** Stored in Requests DB for  `input-proof`.
- **Logging:** `Relayer` includes in every log corresponding to a HTTP request

### External Request ID

- **Purpose:** Track a request across client, Cloudflare and Kong and relayer.
- **Format:** `UUID v4` for its opaqueness, fully random property with strong uniqueness guarantee
- **Creation**: By Kong plugin on every HTTP request (`v2 /POST`, `v2 /GETs`)
- **Relationship**: ???
- **Visibility for user**: :white_check_mark: Receives in each HTTP response, in `X-Request-ID` header.
- **Storage:** :x: Not stored in database.
- **Logging:** 
    1. `Kong`  creates on request and includes it in request/response logs.
    2. `Relayer` includes once at entry of HTTP handler along with Internal Request ID.
    3. `Relayer-SDK` includes it logs when response is received.

### Internal Public/User Decryption IDs

- **Purpose:** Deduplicate public/user-decryption requests based on payload content. One for each *public* and *user* decryption.

- **Format:** `XXH3 hash` of payload for its determinism property

    1. For `public-decrypt` , complete payload is considered.

    2. For `user-decrypt`, payload after nullifying `request_validity`  field is used.

        Request Validity field is defined to ensure relayer forwards the request to Gateway chain before this validity expires. However, if it was processed successfully once and indexed, we look up the result from indexer and this field can be safely ignored.

- **Creation:** By HTTP handler in application, on first occurrence of the request

    1. `v2` : `POST /user-decrypt`, `POST /public-decrypt`

- **Relationship**: 

    1. For user/public-decryption requests, mapped `1:1` to *External Reference ID*.

- **Visibility for user**: :x: No

- **Storage:** :white_check_mark: Stored in database

- **Logging**: `Relayer` includes it in logs related to indexing part of the flow.

### External Reference ID

- **Purpose**: Referenced by client for polling results in `v2` API.
- **Format:**  `UUID v4` for its opaqueness, fully random property with strong uniqueness guarantee
- **Creation:** By HTTP handler in application, 
    1. Created on first request for a payload. On subsequent requests, it's fetched from DB using *Internal Public/User Decryption Indexer ID*
        1. `v2` : `POST /user-decrypt`, `POST /public-decrypt`
    2. Created on every request for 
        1. `v2`: `POST /input-proof`
- **Relationship:** 
    1. For user/public-decryption requests, mapped `1:1` to *Internal Public/User Decryption ID*.
    2. For input-proof requests, mapped `1:1` to *Internal Request ID*.
- **Visibility for user**: :white_check_mark: 
    1. Received in Response: `v2` `POST /user-decrypt`, `POST /public-decrypt`, `POST /input-proof`.
    2. Used in Requests: `v2` `GET /user-decrypt`, `GET /user-decrypt`, `GET /input-proof` 
- **Storage**: :white_check_mark: Stored in Indexer DB
- **Logging**: `Relayer` includes once at creation of indexer entry and once in the HTTP handler of GET request.

### Gateway Public/User Decryption ID

- **Purpose**: Referenced on gateway chain to map user / public decryption request to responses.

- **Format:**  `Number`, scheme is opaque to relayer.

- **Creation:** By Decryption Manager contract on Gateway Chain on successful user / public decryption request. 

- **Relationship:** 

  1. For *User/Public-decryption requests*, mapped `1:1` to *Internal Public/User Decryption Indexer ID*.

- **Visibility for user**: :x: Not visible to user.

  1. Could be included in responses. 

- **Storage**: :white_check_mark: Stored in Indexer DB

- **Logging**: `Relayer` includes it in logs related to event listening part of the flow until its mapped to *Internal Public/User Decryption Indexer ID*.

  

### Gateway Input Proof ID

- **Purpose**: Referenced on gateway chain to map input proof request to responses.

- **Format:**  `Number`, scheme is opaque to relayer.

- **Creation:** By Input Proof contract on Gateway Chain on successful user / public decryption request. 

- **Relationship:** 

  1. For *Input proof* requests, mapped `1:1` to *Internal Request ID*.

- **Visibility for user**: :x: Not visible to user.

  1. Could be included in responses. 

- **Storage**: :white_check_mark: Stored in Indexer DB

- **Logging**: `Relayer` includes it in logs related to event listening part of the flow until its mapped to *Internal Request ID*.

  



