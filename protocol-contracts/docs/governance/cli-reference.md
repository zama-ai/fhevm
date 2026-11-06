# CLI Reference: Governance Tools

All tools live in the [`protocol-contracts/scripts/governance-proposal-builder`](https://github.com/zama-ai/fhevm/tree/main/protocol-contracts/scripts/governance-proposal-builder) directory of the [fhevm](https://github.com/zama-ai/fhevm) repo.

---

## Installation

```bash
git clone https://github.com/zama-ai/fhevm.git
cd fhevm/protocol-contracts/scripts/governance-proposal-builder
npm install
cp .env.example .env
```

Edit `.env` with the required values:

| Variable | Required | Description |
|---|---|---|
| `RPC_ETHEREUM` | Yes | Ethereum RPC URL (mainnet or Sepolia). Use your own node when possible. |
| `RPC_<DESTINATION>` | No | RPC for each cross-chain destination (e.g. `RPC_GATEWAY_MAINNET`, `RPC_GATEWAY_TESTNET`, `RPC_POLYGON_AMOY_TESTNET`). Falls back to the registry default in `destinations.js` when unset. See [Destinations](destinations.md). |
| `ETHERSCAN_API_KEY` | Yes (verification step) | Etherscan API key. Enables ABI fetching for human-readable logs in the inspector. |

---

## `aragon-proposal-inspector`

Independently verifies an Aragon Multisig proposal using only an RPC endpoint. Bypasses all Aragon-hosted infrastructure (subgraph, API, frontend).

**Used by:** [Reviewers](reviewing-proposals.md) (independent proposal verification)

### Usage

```bash
npm run aragon-proposal-inspector -- --plugin <PLUGIN_ADDRESS> --id <PROPOSAL_ID>
```

Optional flags:
- `--rpc <URL>`: override `RPC_ETHEREUM` from `.env`
- `--json`: output raw JSON instead of formatted logs

### Expected inputs

| Input | Description | Where to find it |
|---|---|---|
| `--plugin` | Multisig Plugin contract address (**not** the DAO address) | [protocol-registry repo](https://github.com/zama-ai/protocol-registry) |
| `--id` | Proposal ID (`uint256`) | Aragon frontend → click "Published: \<DATE\>" → Etherscan Logs tab → `proposalId` field |

### Expected output

Formatted log showing for each action:
- `to`: target contract address (with name if Etherscan API key is set)
- `function`: decoded function name and arguments
- `value`: ETH value

### Trust model

- **Trusted:** RPC endpoint, ethers.js library
- **Untrusted:** Aragon subgraph, Aragon API, Aragon frontend, Etherscan (used only for ABI fetching)
- If the ABI from Etherscan is wrong, decoding fails (the script panics) rather than silently producing incorrect output.

### Common errors

| Error | Cause | Fix |
|---|---|---|
| Cannot fetch proposal | Wrong plugin address or proposal ID | Verify `--plugin` is the Multisig Plugin (not the DAO address). Verify `--id` from Etherscan Logs. |
| ABI fetch failed | Etherscan API key missing or invalid | Set `ETHERSCAN_API_KEY` in `.env`, or the script falls back to raw calldata display. |
| RPC connection error | `RPC_ETHEREUM` not set or unreachable | Set a valid RPC URL in `.env` or pass `--rpc`. |
| Decoding error | Contract ABI mismatch | Check that the contract is verified on Etherscan. The script may still show raw calldata. |

---

## Utility: `cast abi-decode`

Not part of this repo, but frequently used alongside these tools for manual verification. Part of [Foundry](https://book.getfoundry.sh/).

### Usage

```bash
# Decode ABI-encoded data (without selector)
cast abi-decode 'f()(address,uint256)' "0x00000000..."

# Encode a function call (to verify bytes inputs)
cast calldata 'reinitializeV2()'
```

### Installation

```bash
curl -L https://foundry.paradigm.xyz | bash
foundryup
```

## `fill-options-remote-proposal`

Computes LayerZero gas options for a cross-chain (remote) proposal to an EVM destination and produces an Aragon-uploadable JSON file.

**Used by:** [Creators](creating-proposals-remote.md) (building proposals), [Reviewers](reviewing-proposals.md) (verifying proposals)

### Usage

```bash
npm run fill-options-remote-proposal -- --destination <id>
# e.g. --destination gateway-mainnet | gateway-testnet | gateway-devnet | polygon-amoy-testnet | polygon-amoy-devnet
# run `npm run list-destinations` to see all ids (see destinations.md)
```

### Expected input

A `remote-proposal-temp.json` file with **only** three equal-length arrays. The
script fills `to` (the destination's `GovernanceOAppSender`), `method`, `values`
(all `0`), `operations` (all `0`) and `options`:

```bash
cp remote-proposal-temp.example.json remote-proposal-temp.json
```
```json
{
  "targets": ["0x…"],
  "functionSignatures": ["addOwnerWithThreshold(address,uint256)"],
  "datas": ["0x…"]
}
```
- `functionSignatures[i]` is **required by default** (never empty): the script builds the 4-byte selector from it, and it keeps every call auditable. `datas[i]` is the ABI-encoded arguments **without** the selector.
- Override the file path with `--input <file>`.
- `--allowEmptyFunctionSignatures` (off by default): allow empty `functionSignatures[i]` entries. When set, an empty entry means the matching `datas[i]` is used **verbatim** as the raw on-chain calldata (**selector included**) and is **not** decoded in the sanity check. Only needed for calls without a human-readable ABI signature (e.g. this is useful in combination with LayerZero wiring transactions produced via the LayerZero cli tool); it trades auditability for flexibility, so keep it off unless required.
- **Out of scope (by design):** every governance proposal is `value` `0` / `Call`, so no other shape is supported. Proposals needing a non-zero native `value` or a `delegatecall` (`operation` `1`) must be hand-crafted; a `--custom` escape hatch will be re-added if/when a concrete need appears.

### Built-in sanity check

For every call, the script decodes `datas[i]` against `functionSignatures[i]`
and prints the resolved function + arguments, then **aborts** if a
`datas`/signature pair does not match. This replaces the manual `cast abi-decode`
step (still usable as an independent cross-check). With
`--allowEmptyFunctionSignatures`, entries whose signature is empty are printed as
raw calldata and skipped in this decode step.

### Expected output

Two files:

| File | Purpose |
|---|---|
| `aragonProposal.json` | Upload to the Aragon DAO frontend. Contains the ABI-encoded `sendRemoteProposal(...)` calldata as a single Aragon transaction. |
| `remote-proposal-filled.json` | Human-readable record: the full filled proposal (`to`/`method`/`arguments` with `options` populated). |

### Common errors

| Error | Cause | Fix |
|---|---|---|
| Unknown destination | `--destination` id not in the registry | Run `npm run list-destinations`; use a known id (see [Destinations](destinations.md)). |
| `datas[i]` does not match `functionSignatures[i]` | Wrong types, or `datas` includes the selector | `datas` must be ABI-encoded **without** the 4-byte selector; ensure types match the signature. |
| Looks like an old full proposal file | A `{ to, method, arguments }` file was passed to `--input` | Convert it to the minimal `{ targets, functionSignatures, datas }` shape; the script fills the rest. |
| Output file already exists | Script refuses to overwrite | Delete `aragonProposal.json` and `remote-proposal-filled.json`, then retry. |
| Target has no bytecode | `targets[i]` address has no contract on the destination | Verify the address is correct and on the right chain. |
| RPC connection error | The destination RPC is unset/unreachable | Set `RPC_<DESTINATION>` in `.env` (see [Destinations](destinations.md)). |

---

## `decode-options-remote-proposal`

Decodes a LayerZero `options` hex string into human-readable `gasLimit` and `nativeValue`.

**Used by:** [Reviewers](reviewing-proposals.md) (comparing gas options)

### Usage

```bash
npm run decode-options-remote-proposal -- --options <OPTIONS_HEX>
```

> The leading `--` after the script name is required so npm forwards the flag to the script.

### Expected input

- `--options`: the hex string from `arguments.options` in a proposal or from the Aragon frontend.

### Expected output

```
Options hex:   0x000301001101000000000000000000000000000493e0
gasLimit:      300000
nativeValue:   0
```

### Common errors

| Error | Cause | Fix |
|---|---|---|
| Missing `--options` flag | Flag not provided or `--` separator missing | Use `-- --options <HEX>` (note the double `--`). |
| Invalid hex string | Malformed options value | Verify the hex string is copied correctly from the proposal. |

---