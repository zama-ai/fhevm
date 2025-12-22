# SQL Scheme for relayer

## Overview

This document describes the rationale behind SQL Scheme for the relayer.

## Description

### Query Patterns

- For Public / User Decryption Requests

  1. **M:** Store _Internal Public/User Decryption Indexer ID_ to _Public / User Decrypt Indexer Table_.

  2. **M:** Store _Internal Public/User Decryption Indexer ID_ to _Public / User Decrypt Requests Table_.
  3. **M:** Store public/user decrypt result by _Public / User Decrypt Indexer Table_
  4. **H: ** Retrieve _User Decrypt Indexer ID_ by _Gateway User Decrypt ID_ from Indexer Table.
  5. **H:** Store user decrypt share by _User Decrypt Indexer ID_ to User Decrypt Shares Table

  6. **H**: Retrieve _Internal Public/User Decryption Indexer ID_ by _External Request ID_

  7. **H:** Retrieve Result by _Internal Public/User Decryption Indexer ID_.

- Tables

  - User Decrypt Requests

    - External Reference ID: `UUID V4`
    - Internal Decryption ID: `UUID V7`
    - Gateway Decryption ID: `Number`
    - Response: binary ? Will be filled ones all shares are available
    - Request Transaction Hash
    - Status: `queued` / `processing` / `tx_in_flight` / `receipt_received` / `completed` / `timed_out` / `failure`
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
    - Status: `queued` / `processing` / `tx_in_flight` / `receipt_received` / `completed` / `timed_out` / `failure`
    - Created at
    - Completed at

  - Input Proof Requests

    - External Reference ID: `UUID v4`
    - Internal Request ID `UUID v7`
    - Gateway Input Proof ID: `Number`
    - Response: binary ?
    - Request Transaction Hash
    - Status: `queued` / `processing` / `tx_in_flight` / `receipt_received` / `completed` / `timed_out` / `failure`
    - Created at
    - Completed at
