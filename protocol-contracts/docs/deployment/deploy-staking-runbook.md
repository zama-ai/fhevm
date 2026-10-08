# Staking deployment runbook

> Deploys the Zama Protocol staking system (see [`staking.md`](../staking.md)) to any EVM
> network: two `ProtocolStaking` roots (KMS + Coprocessor), `N` `OperatorStaking` pools
> (each with an auto-deployed `OperatorRewarder`), registers pool eligibility, wires the
> staking token's minting permission, and hands ownership to the DAO.
>
> All commands run from `contracts/staking/` and take `--network <NETWORK>`.

## Placeholders

| Placeholder | Meaning |
| --- | --- |
| `<NETWORK>` | The Hardhat network name (e.g. `mainnet`, `testnet`, `hoodi`) |
| `<RPC_URL>` / `<CHAIN_ID>` | RPC endpoint and chain id for `<NETWORK>` |
| `PROTOCOL_DEPLOYER_ADDRESS` | Wallet that deploys and initially owns the contracts |
| `DAO_ADDRESS` | Protocol DAO governance — final owner / `MANAGER_ROLE` holder |
| `SETUP_MULTISIG` | Multisig that executes calls on behalf of the DAO (accept transfer, grant minter) |
| `ZAMA_TOKEN_ADDRESS` | The staking/reward ERC-20 (real ZAMA token, or a testnet mock) |
| `N_COPRO` / `N_KMS` | Number of Coprocessor / KMS operator pools |

---

## Requirements

| Input | Where to get it |
| --- | --- |
| `PRIVATE_KEY` for `PROTOCOL_DEPLOYER_ADDRESS` | DFNS / internal secrets |
| RPC URL for `<NETWORK>` | Infura / Alchemy / internal node / public endpoint |
| `ETHERSCAN_API_KEY` | Etherscan dashboard (only for optional source verification) |
| `ZAMA_TOKEN_ADDRESS` | Existing ZAMA token address, or deploy a mock (Phase 1) |
| `DAO_ADDRESS`, `SETUP_MULTISIG` | [Addresses directory](../addresses/README.md) for the target chain |
| Native gas token | Fund `PROTOCOL_DEPLOYER_ADDRESS` before starting |

---

## What gets deployed

| Contract | Kind | Count | Notes |
| --- | --- | --- | --- |
| `ProtocolStaking` | UUPS proxy + impl | 2 | KMS + Coprocessor roots |
| `OperatorStaking` | UUPS proxy + impl | `N_COPRO + N_KMS` | One pool per operator per role |
| `OperatorRewarder` | Immutable | one per pool | Deployed + started inside `OperatorStaking.initialize` (`contracts/OperatorStaking.sol:140-150`) |
| `ERC20Mock` | Token | 0 or 1 | Testnet only — the mock staking/reward token (Phase 1) |

Operator share-token naming (rule from [`staking.md`](../staking.md)): symbol
`stZAMA-<name>-<role>`, name `<name> Staked ZAMA (<role>)`, where `<role>` is `KMS` or
`Coprocessor`.

---

## Network setup

Add `<NETWORK>` to `hardhat.config.ts` under `networks` and, for source verification, add
a matching `etherscan.customChains` entry pointing at the target chain's explorer API.

```ts
// networks
<NETWORK>: { url: process.env.<NETWORK>_RPC_URL || '', accounts, chainId: <CHAIN_ID> },

// etherscan — hardhat-verify picks the customChain by chainId
etherscan: {
  apiKey: process.env.ETHERSCAN_API_KEY!,
  customChains: [
    {
      network: '<NETWORK>',
      chainId: <CHAIN_ID>,
      urls: {
        apiURL:     '<EXPLORER_API_URL>',      // e.g. Etherscan v2: https://api.etherscan.io/v2/api
        browserURL: '<EXPLORER_BROWSER_URL>',  // e.g. https://etherscan.io
      },
    },
  ],
},
```

Notes:
- **Etherscan v2 multichain API** (`https://api.etherscan.io/v2/api`) routes by chainId, so a single Etherscan API key covers every supported chain.
- **Blockscout** is an alternative explorer; if the chain has a Blockscout instance, point `apiURL` at `<blockscout-host>/api` and `browserURL` at `<blockscout-host>`. Any non-empty `apiKey` works — Blockscout ignores it.
- Only one customChain per chainId is honored by `hardhat-verify` (last-in-array wins on collision). To verify on both explorers, do it in two passes, swapping the `urls` between runs.

---

## Phase 0 — Configure `.env`

Copy `.env.example` to `.env` and fill it in for `<NETWORK>`:

```dotenv
PRIVATE_KEY=<key for PROTOCOL_DEPLOYER_ADDRESS>
ETHERSCAN_API_KEY=<optional, for verification>
<NETWORK>_RPC_URL=<RPC_URL>

# Staking / reward token — real ZAMA token, or the mock deployed in Phase 1
ZAMA_TOKEN_ADDRESS=<ZAMA_TOKEN_ADDRESS>
# Final owner / manager
DAO_ADDRESS=<DAO_ADDRESS>

# ── ProtocolStaking (both roots) ──
# Cooldown: 604800 (7 days) on mainnet, 180 (3 min) on testnet
PROTOCOL_STAKING_COPRO_TOKEN_NAME="Staked ZAMA (Coprocessor)"
PROTOCOL_STAKING_COPRO_TOKEN_SYMBOL=stZAMA-Coprocessor
PROTOCOL_STAKING_COPRO_VERSION=1
PROTOCOL_STAKING_COPRO_COOLDOWN_PERIOD=<7d mainnet | 180 testnet>
PROTOCOL_STAKING_COPRO_REWARD_RATE=<tokens per second, 18 decimals>
PROTOCOL_STAKING_KMS_TOKEN_NAME="Staked ZAMA (KMS)"
PROTOCOL_STAKING_KMS_TOKEN_SYMBOL=stZAMA-KMS
PROTOCOL_STAKING_KMS_VERSION=1
PROTOCOL_STAKING_KMS_COOLDOWN_PERIOD=<7d mainnet | 180 testnet>
PROTOCOL_STAKING_KMS_REWARD_RATE=<tokens per second, 18 decimals>

# ── OperatorStaking — one indexed block per operator (0 .. N-1) ──
NUM_OPERATOR_STAKING_COPRO=<N_COPRO>
NUM_OPERATOR_STAKING_KMS=<N_KMS>
# For each Coprocessor operator i in 0..N_COPRO-1:
OPERATOR_STAKING_COPRO_TOKEN_NAME_i="<name> Staked ZAMA (Coprocessor)"
OPERATOR_STAKING_COPRO_TOKEN_SYMBOL_i="stZAMA-<name>-Coprocessor"
OPERATOR_REWARDER_COPRO_BENEFICIARY_i="<operator beneficiary>"
OPERATOR_REWARDER_COPRO_MAX_FEE_i=2000   # 20% max (basis points)
OPERATOR_REWARDER_COPRO_FEE_i=<initial fee bps>
# For each KMS operator j in 0..N_KMS-1: same block with KMS/ names/symbols
```

---

## Phase 1 — Staking token

**Real token (mainnet / production):** set `ZAMA_TOKEN_ADDRESS` in `.env` to the deployed
ZAMA token. Skip to Phase 2. Grant its minter role in Phase 6.

**Testnet mock (optional):** the staking contracts need a token whose `mint` they can
call. `ERC20Mock` exposes a public `mint(address,uint256)` capped at
`publicMintCap = 1_000_000 * 10^decimals` per call, and a `MINTER_ROLE` (bypasses the cap)
mirrored on the real ZAMA token — the deployer receives `DEFAULT_ADMIN_ROLE` and grants
`MINTER_ROLE` to both `ProtocolStaking` contracts in Phase 6 so `claimRewards` can mint
rewards larger than the public cap.

```bash
npx hardhat task:deployERC20MockAndMintDeployer --network <NETWORK>
```
- Copy the printed `ERC20Mock` address into `.env` as `ZAMA_TOKEN_ADDRESS`

> The public per-call cap is a fixed immutable set at construction.
> Read it via `publicMintCap()`. Any address that holds `MINTER_ROLE`
> can mint any amount.

---

## Phase 2 — Local dry run (before touching `<NETWORK>`)

```bash
npm run compile
npx hardhat test          # unit tests
npx hardhat test:tasks    # deploys mock → protocol → operator on in-memory net + task tests
```

---

## Phase 3 — Deploy the ProtocolStaking contracts

```bash
npx hardhat task:deployAllProtocolStakingContracts --network <NETWORK>
```

At deploy the `governor` and `MANAGER_ROLE` are both set to the deployer for
configuration; they move to the DAO in Phase 8.

---

## Phase 4 — Deploy the OperatorStaking contracts

```bash
npx hardhat task:deployAllOperatorStakingContracts --network <NETWORK>
```
- `N_COPRO` Coprocessor pools + `N_KMS` KMS pools deployed (each prints its `OperatorRewarder`)

---

## Phase 5 — Register operators as eligible

```bash
npx hardhat task:addAllOperatorsAsEligible --network <NETWORK>
```
Calls `addEligibleAccount(operatorStaking)` once per pool on that pool's own root: the
Coprocessor pools on the Coprocessor `ProtocolStaking`, the KMS pools on the KMS
`ProtocolStaking` — `N_COPRO + N_KMS` transactions total. Requires `MANAGER_ROLE` (still
held by the deployer at this point).

---

## Phase 6 — Grant the token's `MINTER_ROLE` to the ProtocolStaking contracts

Because `claimRewards` mints rewards, both ProtocolStaking roots must hold the token's
`MINTER_ROLE` (the public 1M-per-call cap on the mock would otherwise brick any user who
accrues ≥1M rewards).

**Testnet mock:** the deployer holds `DEFAULT_ADMIN_ROLE` on the mock — run the task:
```bash
npx hardhat task:grantZamaTokenMinterRoleToProtocolStaking --network <NETWORK>
```

**Real token (mainnet / production):** same `grantRole(role, account)` call on
`ZAMA_TOKEN_ADDRESS`, but the deployer does not hold `DEFAULT_ADMIN_ROLE` on the real
token, so this is executed by `SETUP_MULTISIG` on behalf of `DAO_ADDRESS` via the
[Creating Proposals Runbook](../governance/creating-proposals-ethereum.md):

- `role`: `0x9f2df0fed2c77648de5860a4cc508cd0818c85b8b8a1ab4ceeef8d981c8956a6`
  (`keccak256("MINTER_ROLE")`)
- `account`: the Coprocessor ProtocolStaking, then the KMS ProtocolStaking (two calls)

---

## Phase 6.5 — (Optional, testnet) Pre-stake to keep UI APY realistic

Only run this if a fresh deployment shows non-realistic APYs in the UI because the pools
are empty relative to the reward rate. Requires per-operator `OPERATOR_STAKING_*_INITIAL_DEPOSIT_ASSETS_i`
and `OPERATOR_STAKING_*_INITIAL_DEPOSIT_RECEIVER_i` env vars (defaults: 1M ZAMA into the
deployer as receiver for every pool).

```bash
# Mint enough mock ZAMA to cover every pool (N_COPRO + N_KMS calls of 1M each)
npx hardhat task:mintToDeployer --count <N_COPRO + N_KMS> --network <NETWORK>

# Approve + deposit into every operator pool
npx hardhat task:depositAllOperatorStakingFromDeployer --network <NETWORK>
```

The public per-call cap on the mock is 1M, so `--count` iterates that many public mints —
no `MINTER_ROLE` grant to the deployer needed.

---

## Phase 7 — Configure operator fees

Initial `feeBasisPoints` / `maxFeeBasisPoints` are set at deploy time from the
`OPERATOR_REWARDER_*_FEE_*` / `_MAX_FEE_*` env vars. Afterwards each operator's
beneficiary can adjust their own pool via `OperatorRewarder.setFee(basisPoints)`
(beneficiary-only), capped at that pool's `maxFeeBasisPoints`. That cap is seeded to
`2000` (20%) here from `OPERATOR_REWARDER_*_MAX_FEE_i` and is itself owner-adjustable via
`setMaxFee`, up to the protocol hard limit of `10000`.

---

## Phase 8 — Transfer ownership to the DAO

> For an ephemeral testnet where the deployer intentionally keeps `owner` / `MANAGER_ROLE`
> (e.g. to keep calling `addEligibleAccount`), **skip this phase**.

**Manager role** — grant to the DAO, then renounce the deployer's
(`MANAGER_ROLE` = `keccak256("MANAGER_ROLE")` =
`0x241ecf16d79d0f8dbfb92cbc07fe17840425976cf0667f022fe9877caa831b08`):
```bash
npx hardhat task:grantProtocolStakingManagerRolesToDAO --network <NETWORK>
npx hardhat task:renounceProtocolStakingManagerRolesFromDeployer --network <NETWORK>
```

**Governor role** (`DEFAULT_ADMIN_ROLE`, 2-step) — begin transfer, then the DAO accepts:
```bash
npx hardhat task:beginTransferProtocolStakingGovernorRolesToDAO --network <NETWORK>
```
- [ ] `SETUP_MULTISIG` calls `acceptDefaultAdminTransfer()` on both ProtocolStaking
      contracts on behalf of `DAO_ADDRESS`
- [ ] `owner()` on both roots returns `DAO_ADDRESS`

---

## Phase 9 — Source verification

(Against whichever explorer is configured in `hardhat.config.ts` for `<NETWORK>`.)

```bash
npx hardhat task:verifyERC20Mock --contract-address <MOCK_ADDR> --network <NETWORK>  # testnet mock only
npx hardhat task:verifyAllProtocolStakingContracts --network <NETWORK>
npx hardhat task:verifyAllOperatorStakingContracts --network <NETWORK>
npx hardhat task:verifyOperatorRewarder --network <NETWORK>
```

---

## Phase 10 — Record addresses

Update the `protocol-registry` repo with the deployed addresses (token, both
`ProtocolStaking` proxies, all `OperatorStaking` proxies, all `OperatorRewarder`
contracts) and ping the repo owners to review and merge.

---

## Phase 11 — Deployment checklist review

Before considering the deployment complete, verify on-chain against the saved
`deployments/<NETWORK>/` addresses.

**Staking token**
- [ ] `name()` / `symbol()` match the expected constants
      (`ERC20_MOCK_TOKEN_NAME` / `ERC20_MOCK_TOKEN_SYMBOL` for the mock)
- [ ] `hasRole(MINTER_ROLE, protocolStaking)` is true for both `ProtocolStaking`
      roots (`MINTER_ROLE` =
      `0x9f2df0fed2c77648de5860a4cc508cd0818c85b8b8a1ab4ceeef8d981c8956a6`)

**Each `ProtocolStaking` root (KMS + Coprocessor)**
- [ ] `owner()` matches the expected owner (`DAO_ADDRESS` in prod, deployer on
      an ephemeral testnet that skipped Phase 8)
- [ ] `MANAGER_ROLE`
      (`0x241ecf16d79d0f8dbfb92cbc07fe17840425976cf0667f022fe9877caa831b08`) is
      held only by the expected owner (deployer has renounced in prod)
- [ ] `stakingToken()` returns `ZAMA_TOKEN_ADDRESS` — the ZAMA token deployed for
      this staking suite
- [ ] `name()` / `symbol()` / `rewardRate()` match the `.env` values
- [ ] `unstakeCooldownPeriod()` matches the `.env` value
      (`PROTOCOL_STAKING_*_COOLDOWN_PERIOD` — 604800 on mainnet, 180 on testnet)
- [ ] `isEligibleAccount(pool)` returns true for every `OperatorStaking` pool
      under this domain (Coprocessor pools on the Coprocessor root, KMS pools
      on the KMS root)
- [ ] If Phase 6.5 pre-stake ran: `totalSupply()` == the sum of every
      `OPERATOR_STAKING_*_INITIAL_DEPOSIT_ASSETS_i` configured for each OperatorStaking

**Each `OperatorStaking` pool**
- [ ] `name()` / `symbol()` match the per-operator `.env` values
- [ ] `protocolStaking()` returns the root this pool was deployed for — the
      Coprocessor root for a Coprocessor pool, the KMS root for a KMS pool
- [ ] `asset()` returns `ZAMA_TOKEN_ADDRESS`
- [ ] If Phase 6.5 pre-stake ran: `totalAssets()` == this pool's configured
      `OPERATOR_STAKING_*_INITIAL_DEPOSIT_ASSETS_i`
- [ ] `rewarder()` returns the `OperatorRewarder` deployed for this pool, and on
      that rewarder:
  - [ ] `operatorStaking()` returns this pool (the pool and rewarder are paired 1:1)
  - [ ] `protocolStaking()` returns the same root as the pool's `protocolStaking()`
  - [ ] `token()` returns `ZAMA_TOKEN_ADDRESS`
  - [ ] `beneficiary()` / `feeBasisPoints()` / `maxFeeBasisPoints()` match the `.env` values

**Functional smoke test (at least one pool)**
- [ ] Faucet: `token.mint(deployer, amount)` increases the deployer's balance
- [ ] Deposit: `pool.deposit(amount, deployer)` mints pool shares
- [ ] Rewards accrue: after a short wait, `protocolStaking.earned(pool) > 0`
      and `rewarder.claimRewards(deployer)` transfers accrued rewards to the delegator
- [ ] Redeem: `pool.requestRedeem(shares, deployer, deployer)` followed by
      `pool.redeem(...)` after the `unstakeCooldownPeriod` returns assets to
      the delegator. Note that this step waits out the full cooldown
      (`unstakeCooldownPeriod()` — ~3 min on testnet, 7 days on mainnet)

---
