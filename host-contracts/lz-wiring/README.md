# lz-wiring

Isolated workspace for running the **official** LayerZero V2 wiring tasks
(`lz:oapp:wire`, `lz:oapp:peers:get`, `lz:oapp:config:get`, …) against a
`ConfidentialBridge` already deployed by the parent `host-contracts` project.

## Why a separate workspace?

`@layerzerolabs/toolbox-hardhat` transitively pulls in
`@safe-global/protocol-kit` → `zksync-web3@0.14.4` which is hard‑coded against
ethers v5's `ethers.providers.JsonRpcSigner`. The parent `host-contracts`
project is on ethers v6 (via OZ upgrades / `@nomicfoundation/hardhat-ethers`),
so loading `@layerzerolabs/toolbox-hardhat` into the parent's
`hardhat.config.ts` blows up at config-load time. This workspace pins ethers
v5 (matching the official `npx create-lz-oapp` scaffold), runs in its own
`node_modules`, and avoids the conflict entirely.

The bridge proxy itself is unchanged — `lz:oapp:wire` is just a configurator
that calls `setPeer` / `setSendLibrary` / `setReceiveLibrary` / `setConfig` on
the already-deployed contracts.

## Quickstart

```bash
# 1. Install isolated deps (pnpm is required; npm hoists into the parent
#    workspaces' node_modules and re-introduces the ethers-v6 conflict).
cd lz-wiring
pnpm install --ignore-workspace

# 2. After `task:deployBridge` has run on each chain, export the resulting
#    proxy addresses (read them from each per-chain snapshot of
#    addresses/.env.host → CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS):
export SEPOLIA_BRIDGE_ADDRESS=0x...
export POLYGON_AMOY_BRIDGE_ADDRESS=0x...
# (Or set these in the parent ../.env — this workspace loads it.)

# 3. (Optional) Override the per-chain RPC URLs. Defaults are public endpoints.
export SEPOLIA_RPC_URL=...
export POLYGON_AMOY_RPC_URL=...

# 4. Inspect what wire would do (no transactions):
pnpm wire:dry          # full diff: peer/library/DVN/Executor changes that would be applied
pnpm peers:get         # current peers only
pnpm config:get        # full library + DVN/Executor config

# 5. Run the wiring (will submit txs from DEPLOYER_PRIVATE_KEY on both chains):
pnpm wire
```

`pnpm wire` is equivalent to `npx hardhat lz:oapp:wire --oapp-config layerzero.config.ts`.
It will print a diff of (current on-chain state) → (desired state from
`layerzero.config.ts`) and prompt before submitting transactions.

## Editing the pathway

`layerzero.config.ts` defines a single Sepolia ↔ Amoy pathway with a
**testnet-grade single LZ Labs DVN, 1 confirmation**. For production:

- Replace `[['LayerZero Labs'], []]` with `[['LayerZero Labs', '<SECONDARY_DVN>'], []]`.
- Bump `[1, 1]` to per-chain reorg-safe confirmations (Sepolia typically 15+, Amoy 10+).
- Optionally raise `gas` in `EVM_ENFORCED_OPTIONS` based on profiling.

DVN options for testnet are listed at
<https://docs.layerzero.network/v2/deployments/dvn-addresses?chains=sepolia,amoy>.

## Relation to `task:wireBridge` / `task:wireBridgeProduction`

Three options to wire the bridge, in increasing order of completeness:

1. **`task:wireBridge` (parent project)** — only `setPeer` + `setDstChainId`. Relies on the LZ V2 endpoint's defaults. Fine for a testnet smoke test.
2. **`task:wireBridgeProduction` (parent project)** — adds `setSendLibrary`, `setReceiveLibrary`, `setConfig` (Executor + ULN). Same on-chain effect as `lz:oapp:wire`, no extra workspace. Useful if you don't want a `pnpm install` step.
3. **`lz:oapp:wire` (this workspace)** — the canonical LZ flow. Diffs current vs desired state, supports inspection tasks (`peers:get`, `config:get`), and benefits from upstream improvements as LZ ships them.

Pick whichever fits your operations workflow.
