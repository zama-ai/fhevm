# Config Checker

Utilities for checking FHEVM protocol contract configurations.

Returns the current set of active pausers for PauserSet contracts on Ethereum and Gateway chains by analyzing on-chain events.

## Prerequisites

- Node.js (v18+)
- npm

## Installation

```bash
npm install
```

## Configuration

Create a `.env` file based on `.env.example`:

```bash
cp .env.example .env
```

## Available scripts

Currently, most useful scripts are:

```
[*] get-current-pausers
```
### getCurrentPausers

#### Usage

```bash
npm run get-current-pausers
```

The script will:
1. Query both Ethereum and Gateway chains (if configured)
2. Find the deployment block for each PauserSet contract
3. Fetch all `AddPauser`, `RemovePauser`, and `SwapPauser` events
4. Compute the current set of active pausers
5. Display a summary comparing pausers across chains

#### Example Output

```
[Ethereum]
  Finding deployment block for 0xbBfE1680b4a63ED05f7F80CE330BED7C992A586C...
  Deployment block: 23832655
  Current block: 23900000
  Fetching pauser events...
    AddPauser: 100% - found 2 events
    RemovePauser: 100% - found 0 events
    SwapPauser: 100% - found 0 events

[Gateway]
  Finding deployment block for 0x571ecb596fCc5c840DA35CbeCA175580db50ac1b...
  Deployment block: 1000000
  Current block: 1050000
  Fetching pauser events...
    AddPauser: 100% - found 2 events
    RemovePauser: 100% - found 0 events
    SwapPauser: 100% - found 0 events

==================================================
SUMMARY
==================================================

Ethereum pausers:
  1. 0x1234...abcd
  2. 0x5678...efgh
  Total: 2 pauser(s)

Gateway pausers:
  1. 0x1234...abcd
  2. 0x5678...efgh
  Total: 2 pauser(s)

--------------------------------------------------
Pausers are IDENTICAL on both chains.
```

If pausers differ between chains, the script will show which addresses exist only on one chain.
