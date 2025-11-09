# SQL Scheme for relayer

## Overview

This document describes the rationale behind SQL Scheme for the relayer.



## Description

### Query Patterns

- For Public / User Decryption Requests

  1. **M:** Store *Internal Public/User Decryption Indexer ID* to *Public / User Decrypt Indexer Table*.

  2. **M:** Store *Internal Public/User Decryption Indexer ID* to *Public / User Decrypt Requests Table*.
  3. **M:** Store public/user decrypt result by *Public / User Decrypt Indexer Table*
  4. **H: ** Retrieve *User Decrypt Indexer ID* by *Gateway User Decrypt ID* from Indexer Table. 
  5. **H:** Store user decrypt share by *User Decrypt Indexer ID* to User Decrypt Shares Table

  3. **H**: Retrieve *Internal Public/User Decryption Indexer ID* by *External Request ID*

  4. **H:** Retrieve Result by *Internal Public/User Decryption Indexer ID*.





- Tables

  - User Decrypt Requests

    - External Reference ID: `UUID V4`

    - Internal Decryption ID: `UUID V7` 

    - Gateway Decryption ID: `Number`

    - Response: binary ? Will be filled ones all shares are available

    - Request Transaction Hash

    - Status: Queued / Transaction Sent / Completed

    - Created at 

    - Completed at

      

  - User Decrypt Shares

    - Gateway Decryption ID: `Number`

    - Share: binary ?

    - Signature binary ?

    - Share Index ?

    - Created at

      

  - Public Decrypt Requests\

    - External Reference ID: `UUID V4`

    - Internal Decryption ID: `UUID V7` 

    - Gateway Decryption ID: `Number`

    - Response: binary ?

    - Request Transaction Hash

    - Status: Queued / Transaction Sent / Completed

    - Created at

    - Completed at

       

  - Input Proof Requests

    - External Reference ID: `UUID v4`
    - Internal Request ID `UUID v7`
    - Gateway Input Proof ID:  `Number`
    - Response: binary ?
    - Request Transaction Hash
    - Status: Queued / Transaction Sent / Completed
    - Created at
    - Completed at



