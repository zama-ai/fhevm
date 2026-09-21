# Governance Proposal Builder

Helpers to build, validate, and fill DAO Governance proposals.

## Prerequisites

- Node.js v18+
- npm

## Installation

```
npm install
```

## Configuration

Create a `.env` file based on `.env.example`:
```
cp .env.example .env
```

## Destinations

Cross-chain proposals target an EVM **destination** chain. Each destination has
its own `GovernanceOAppSender` (on Ethereum/Sepolia), `GovernanceOAppReceiver`,
`AdminModule`, and multisig on the destination. They are defined in
[`destinations.js`](./destinations.js):

```bash
npm run list-destinations   # prints the registry (ids, addresses, RPC vars)
```

To add a new EVM destination, append an entry to `destinations.js` and add its
RPC var to `.env.example` — no script code changes and no per-destination
template are required (the single minimal input file works for every
destination; the script fills the per-destination `to`).

## Available scripts

Currently available scripts are:
```
[*] fill-options-remote-proposal
[*] decode-options-remote-proposal
[*] list-destinations
[*] aragon-proposal-inspector
```

### fillOptionsRemoteProposal

#### Workflow

1. Describe the calls in a **minimal** input file — you provide **only** three
   equal-length arrays; the script fills everything else (`to`, `method`,
   `values`=0, `operations`=0, `options`):

   ```bash
   cp remote-proposal-temp.example.json remote-proposal-temp.json
   ```
   ```json
   { "targets": ["0x…"], "functionSignatures": ["addOwnerWithThreshold(address,uint256)"], "datas": ["0x…"] }
   ```
   (`functionSignatures[i]` is **required** by default — never empty; the script
   derives the 4-byte selector from it and `datas[i]` carries the ABI-encoded
   args **without** the selector. Pass `--allowEmptyFunctionSignatures` to
   override this: an empty `functionSignatures[i]` then means `datas[i]` is used
   verbatim as the raw on-chain calldata — selector included — and is **not**
   decoded in the sanity check. This is off by default because it sacrifices
   auditability; use it only for calls that have no human-readable ABI signature)

   > **Out of scope (by design):** every governance proposal to date is
   > `value` `0` / `Call`, so the tool only builds that shape. Proposals that
   > need a non-zero native `value` or a `delegatecall` (`operation` `1`) are
   > **not** supported here and must be hand-crafted; a `--custom` escape hatch
   > will be re-added if/when a concrete need appears.

2. Run the fill script for the target destination:

   ```bash
   npm run fill-options-remote-proposal -- --destination gateway-mainnet
   # or --destination gateway-testnet, gateway-devnet, polygon-amoy-testnet, polygon-amoy-devnet, …
   ```

3. The script writes two files in current directory:
   - `remote-proposal-filled.json` — the full filled proposal
     (`to`/`method`/`arguments` with `options` populated). Human-readable record.
   - `aragonProposal.json` — the same call rendered as a single Aragon
     transaction (`[{ to, value, data }]`) where `data` is the ABI-encoded
     `sendRemoteProposal(...)` calldata. **This is the file to upload via the
     Aragon DAO front-end.**

#### What the script checks

- **Input:** exactly the keys `targets`, `functionSignatures`, `datas`;
  equal-length string arrays; `targets[i]` are valid addresses. (An old
  full-shape `{ to, method, arguments }` file is rejected with a hint to convert
  it to the minimal shape.)
- **Sanity check:** decodes each `datas[i]` against `functionSignatures[i]`,
  prints the resolved call + arguments, and **aborts** on a `datas`/signature
  mismatch. (With `--allowEmptyFunctionSignatures`, entries whose signature is
  empty are printed as raw calldata and skipped in this decode step.)
- each `targets[i]` has non-empty bytecode on the destination chain.

#### How `options` is computed

The script uses `@layerzerolabs/lz-v2-utilities` to build a LayerZero
option containing a single executor `lzReceive` action. To do this it runs
`eth_estimateGas` against the destination chain (RPC from the destination's
`rpcEnvVar` in `.env`, falling back to the registry default) with the
destination multisig as the (unsigned) `from` to estimate the gas needed, and
adds some constant and proportional buffers.

#### Output

The script writes two files next to the input:

```
./remote-proposal-filled.json
./aragonProposal.json
```

It **refuses to overwrite** either file if it already exists, and exits
with a non-zero status before writing anything. Delete the file(s) first if
you want to regenerate.

#### OPTIONAL CROSS-CHECK: Decoding individual `datas` entries

The script already decodes and prints every `arguments.datas[i]` (against the
matching `arguments.functionSignatures[i]`) and aborts on a mismatch. As an
independent cross-check you can also decode them yourself — note datas are
encoded **without** the 4-byte selector, so use `cast abi-decode` and treat the
bytes as the "return-value" tuple, for example:

```bash
DATA=0x00000000000000000000000012345678901234567890123456789012345678900000000000000000000000000000000000000000000000000000000000000002

cast abi-decode 'f()(address,uint256)' "$DATA"
# 0x1234567890123456789012345678901234567890
# 2
```

The `f()` is just a placeholder; only the parameter-types tuple matters. Pass
the same types as the matching `functionSignatures[i]`.

### decodeOptionsRemoteProposal

Reverse of the `computeLZOptions` step inside `fillOptionsRemoteProposal`:
takes a LayerZero options hex string
and prints the decoded `gasLimit` and `nativeValue`. Useful to sanity-check
what's in `arguments.options` of a remote proposal and to use before voting on a pending DAO cross-chain proposal.

#### Usage

```bash
npm run decode-options-remote-proposal -- --options 0x000301001101000000000000000000000000000493e0
```

The leading `--` is required so npm forwards the flag to the script instead
of consuming it itself. `--options` is the only flag and is required.

#### Output

Prints (to stdout) the raw hex and the decoded fields:

```
Options hex:   0x000301001101000000000000000000000000000493e0
gasLimit:      300000
nativeValue:   0
```

### aragonProposalInspector

Independent, RPC-only viewer for a pending proposal on an Aragon
**Multisig** plugin. Useful as a sanity check before voting in case the
Aragon front-end has been compromised: this script bypasses
all Aragon-hosted infrastructure.

#### Trust path

For the proposal content itself: only the chosen RPC endpoint and ethers js library. No Aragon subgraph or
hosted API is consulted. Point `RPC_ETHEREUM` at an endpoint
you trust — ideally your own node.

For the optional abi-decoding of calldata and contract names: also Etherscan. 
The abi-decoding is still done and checked locally, we use Etherscan only for fetching 
the ABIs and contrat names, so we don't actually trust Etherscan for the security of decoded data.

#### Usage

```bash
# Reads RPC from .env (RPC_ETHEREUM)
npm run aragon-proposal-inspector -- --plugin 0xPLUGIN --id 5

# Or override the RPC inline (e.g. to point at Sepolia, your own node, ...)
npm run aragon-proposal-inspector -- --plugin 0xPLUGIN --id 5 --rpc https://your.rpc

# Machine-readable output
npm run aragon-proposal-inspector -- --plugin 0xPLUGIN --id 5 --json
```

The leading `--` is required so npm forwards flags to the script. Required
flags are `--plugin` (the Multisig plugin address — **not** the DAO address;
in Aragon OSx, proposals live on the plugin) and `--id` (decimal or
0x-hex non-negative integer).

#### Optional: Etherscan enrichment

If `ETHERSCAN_API_KEY` is set in `.env`, every action's `to` address is
additionally looked up via the Etherscan v2 API
(single key across all supported chains). For
each address, the inspector prints:

- the contract `name` and verification status,
- if the contract is flagged as a proxy and `Implementation` is set, the
  implementation's name + verification status,
- the abi-decoded `function:` line + each argument, decoded against the
  contract's own ABI when verified, falling back to the implementation ABI
  for verified proxies.

#### What it prints

For the human-readable mode:

- `plugin`: the plugin address,
- `chainId`: the chain id,
- `latestBlock`: the latest block fetched from the RPC,
- `proposalId`: the proposal id,
- `executed`: if the proposal has been executed,
- `approvals`: the number of approvals received vs the minimum number of approvals required,
- `startDate`: the proposal's start date,
- `endDate`: the proposal's end date,
- `windowStatus`: not yet open, open, or closed.
- `etherscanEnabled`: if Etherscan is enabled,
- `actions`: the number and details of all actions in the proposal. For each:
  - `to`: the proxy contract address
  - (optional) `name`: Etherscan information (when enabled), including: name, current implementation address, verification status,
  - `value`: in wei, 
  - `data`: the full raw calldata, 
  - (optional) `function`: the decoded signature and arguments (when Etherscan is enabled and
  the calldata can be decoded).