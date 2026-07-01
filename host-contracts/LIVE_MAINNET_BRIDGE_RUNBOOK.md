# Live mainnet runbook: Ethereum + Polygon

End-to-end procedure for spinning up the full FHEVM host stack on two EVM
mainnets, wiring the `ConfidentialBridge` between them with production-grade
LayerZero V2 settings, running the mock coprocessor daemon, and exercising a
confidential OFT bridging using `ConfidentialOFT` contract instances.

**NOTE:** Last 2 steps (steps 6 and 7) of this runbook are optional and do not involve the confidential OFT.
Step 6 shows how to run the bridge's `lzReceive` gas profiler in order to fit the corresponding default constant gas parameters.
Step 7 shows how to deploy and test another minimalistic confidential OApp: `HandlesListCOnfidentialOApp`.

Target chains used as the running example:

| Chain            | EVM chainId | LZ V2 EID |
| ---------------- | ----------- | --------- |
| Ethereum Mainnet | `1`         | `30101`   |
| Polygon Mainnet  | `137`       | `30109`   |

The procedure generalizes to any chain pair that has a canonical LayerZero V2
endpoint. It just happens that the EndpointV2 contract address is identical on Ethereum and Polygon
mainnet (`0x1a44076050125825900e736c501f859c50fE728c`). Always look up the
EndpointV2 address for each target chain in the
[LayerZero deployments table](https://docs.layerzero.network/v2/deployments/deployed-contracts)
before deploying.

> **Scope.** This runbook covers the host-contract side of the stack plus the
> mock coprocessor needed to make FHE operations observable in plaintext. The
> production coprocessor (signers, KMS endpoints, …) is out of scope — see
> the protocol RFCs for that.

> **`BRIDGE_ENV=mainnet`.** The mock coprocessor daemon
> (`scripts/mock-coprocessor/config.ts`) and the bridge stress test
> (`scripts/stress/handlesListBridgeStress.ts`) are environment-aware: they pick
> their chain pair from the `BRIDGE_ENV` variable, defaulting to `testnet`. This
> runbook always sets `BRIDGE_ENV=mainnet`, which selects the Ethereum/Polygon
> mainnet pair, reads the `ETHEREUM_MAINNET_RPC_URL` / `POLYGON_MAINNET_RPC_URL`
> RPCs, and the `addresses-ethereum/` / `addresses-polygon/` address snapshots.

## Prerequisites

### Wallets and funds

A single deployer key is reused across both chains. Make sure to have pre-funded your deployer account with enough Ethereum mainnet ETH and Polygon mainnet POL to cover the deployments, wiring transactions and LayerZero fees.

First step, after installing dependencies, is to copy the `.env.example` to `.env`:

```bash
cp .env.example .env
```

Keep all the default values inside this newly created `.env` file, with the exception of the following variables which you must replace with your own values:

- `DEPLOYER_PRIVATE_KEY` — your funded deployer key.
- `ETHERSCAN_API_KEY` — an [Etherscan V2 multichain API key](https://docs.etherscan.io/) (a single key verifies on both Ethereum and Polygon).
- `LZ_ENDPOINT_ADDRESS` — **critical on mainnet**: replace the default testnet endpoint with the mainnet EndpointV2 address, which is the same on both target chains:

```bash
LZ_ENDPOINT_ADDRESS="0x1a44076050125825900e736c501f859c50fE728c"
```

### RPC URLs

Export the two mainnet RPC URLs once. Use **paid/dedicated** endpoints — the mock
coprocessor and the optional profiler/stress steps make many `eth_getLogs` calls
that exceed free public rate limits:

```bash
export ETHEREUM_MAINNET_RPC_URL=<YOUR_ETHEREUM_MAINNET_RPC_URL>
export POLYGON_MAINNET_RPC_URL=<YOUR_POLYGON_MAINNET_RPC_URL>
```

---

## Step 1 — Deploy host stack on Ethereum Mainnet

Then deploy all host contracts on Ethereum mainnet with:

```bash
RPC_URL="$ETHEREUM_MAINNET_RPC_URL" npx hardhat --network mainnet task:deployAllHostContracts --with-kms-generation true
```

Then verify all Ethereum mainnet contracts on Etherscan via:

```bash
RPC_URL="$ETHEREUM_MAINNET_RPC_URL" npx hardhat task:verifyAllHostContracts --use-internal-proxy-address true --network mainnet
```

After contract verification, save the Ethereum mainnet addresses in a dedicated folder:

```bash
mkdir -p addresses-ethereum
cp -r addresses/ addresses-ethereum/
```

The `cp` is **critical** — the next chain's deploy overwrites `addresses/`.

## Step 2 — Deploy host stack on Polygon Mainnet

Deploy all host contracts on Polygon mainnet with:

```bash
RPC_URL="$POLYGON_MAINNET_RPC_URL" npx hardhat --network polygon task:deployAllHostContracts --with-kms-generation false
```

Then verify all Polygon mainnet contracts on Etherscan via:

```bash
RPC_URL="$POLYGON_MAINNET_RPC_URL" npx hardhat task:verifyAllHostContracts --use-internal-proxy-address true --network polygon
```

After contract verification, save the Polygon mainnet addresses in a dedicated folder:

```bash
mkdir -p addresses-polygon
cp -r addresses/ addresses-polygon/
```

Note each chain's `CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS` from `addresses-{ethereum,polygon}/.env.host` — you'll use both in step 3.

---

## Step 3 — Wire the ConfidentialBridge instances

`lz:oapp:wire` (in `lz-wiring/`) handles peers, send/receive libraries, DVNs,
Executor config, confirmations and enforced options. It does **not** set the
bridge-specific `dstChainId` mapping (an FHEVM extension to the OApp
interface), so there's an extra call per chain at the end.

### 3.1 — Export the bridge addresses

First run this command to extract the ConfidentialBridge addresses instances on both chains:

```bash
export ETHEREUM_BRIDGE_ADDRESS=$(grep '^CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS=' addresses-ethereum/.env.host | cut -d= -f2)
export POLYGON_BRIDGE_ADDRESS=$(grep '^CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS=' addresses-polygon/.env.host | cut -d= -f2)
echo "Ethereum bridge: $ETHEREUM_BRIDGE_ADDRESS"
echo "Polygon bridge:  $POLYGON_BRIDGE_ADDRESS"
```

Then install with `pnpm` (recommended package manager for LayerZero) the needed dependencies for the LayerZero wiring task:

```bash
cd lz-wiring
pnpm i
```

Now replace the address field values in both `lz-wiring/deployments/{ethereum-mainnet,polygon-mainnet}/ConfidentialBridge.json` files by their actual values logged earlier by their corresponding values, i.e replace the `<ConfidentialBridgeAddress>` by their actual values for both chains:

```
{
  "address": "<ConfidentialBridgeAddress>" <-- replace by actual value on Ethereum/Polygon mainnet
}
```

And finally copy the ABI of the ConfidentialBridge to those json files by running (inside `lz-wiring` directory):

```bash
npx ts-node scripts/copyAbiToDeployments.ts
```

### 3.2 — Wiring the bridges

While still in the `lz-wiring` directory, and after checking the config inside `layerzero.config.mainnet.ts`[./layerzero.config.mainnet.ts] is correct - it is crucial to double check the values for required DVNs and number of confirmations for security, run:

```bash
npx hardhat lz:oapp:wire --oapp-config layerzero.config.mainnet.ts
```

The CLI submits one tx per change, on each chain, signed by `DEPLOYER_PRIVATE_KEY`.

### 3.3 — Set the bridge-specific `dstChainId`

```bash
cd ..   # back to host-contracts root

# Ethereum → Polygon
RPC_URL="$ETHEREUM_MAINNET_RPC_URL" \
  npx hardhat --network mainnet task:setDstChainId \
  --bridge-address "$ETHEREUM_BRIDGE_ADDRESS" \
  --remote-eid 30109 \
  --remote-chain-id 137

# Polygon → Ethereum
RPC_URL="$POLYGON_MAINNET_RPC_URL" \
  npx hardhat --network polygon task:setDstChainId \
  --bridge-address "$POLYGON_BRIDGE_ADDRESS" \
  --remote-eid 30101 \
  --remote-chain-id 1
```

---

## Step 4 — Start the mock coprocessor daemon

**Open a dedicated terminal** and keep it running. The daemon is
**live-events-only**: it seeds its cursor at each chain's current head on
startup and never catches up missed events. Any test transaction submitted
before the daemon's `initialised at chain head N` line will be missed.

`BRIDGE_ENV=mainnet` selects the Ethereum/Polygon mainnet pair; the daemon reads
its RPC URLs from `ETHEREUM_MAINNET_RPC_URL` / `POLYGON_MAINNET_RPC_URL` and the
contract addresses from `addresses-ethereum/.env.host` / `addresses-polygon/.env.host`.

```bash
export ETHEREUM_MAINNET_RPC_URL=<YOUR_ETHEREUM_MAINNET_RPC_URL>
export POLYGON_MAINNET_RPC_URL=<YOUR_POLYGON_MAINNET_RPC_URL>
BRIDGE_ENV=mainnet npm run mock:daemon
```

Wait until you see two `initialised at chain head …` lines (one per chain):

```
[mock-coprocessor:ethereum] initialised at chain head 20950123 — only events from block 20950124 onwards will be processed
[mock-coprocessor:polygon]  initialised at chain head 64012345 — only events from block 64012346 onwards will be processed
```

Only proceed once both have printed. Leave the daemon alive for the rest of
the runbook.

---

## Step 5 — Deploy and test `ConfidentialOFT`

`ConfidentialOFT` is a minimalistic cross-chain confidential token that rides
on top of the bridge: each chain holds encrypted per-user balances, and
`send(...)` burns from the sender on the source chain while the destination
chain's `onConfidentialBridgeReceived(...)` mints to the recipient.

### 5.1 — Deploy on Ethereum Mainnet

```bash
cp addresses-ethereum/{FHEVMHostAddresses.sol,.env.host} addresses/

npx hardhat clean

npx hardhat compile

npx hardhat compile:specific --contract examples

RPC_URL="$ETHEREUM_MAINNET_RPC_URL" npx hardhat --network mainnet task:deployConfidentialOFT

cp addresses/.env.host addresses-ethereum/.env.host
```

The new address is appended to `addresses/.env.host` as
`CONFIDENTIAL_OFT_CONTRACT_ADDRESS`. The constructor wires the OFT to the
local bridge it just read from the same file, and sets the deployer wallet as
the OFT's `Ownable2Step` owner.

Then, if you want to verify the contract on Etherscan (optional):

```bash
RPC_URL="$ETHEREUM_MAINNET_RPC_URL" npx hardhat --network mainnet task:verifyConfidentialOFT --use-internal-proxy-address true
```

### 5.2 — Deploy on Polygon Mainnet

```bash
cp addresses-polygon/{FHEVMHostAddresses.sol,.env.host} addresses/

npx hardhat clean

npx hardhat compile

npx hardhat compile:specific --contract examples

RPC_URL="$POLYGON_MAINNET_RPC_URL" npx hardhat --network polygon task:deployConfidentialOFT

cp addresses/.env.host addresses-polygon/.env.host
```

Then, if you want to verify the contract on PolygonScan (optional):

```bash
RPC_URL="$POLYGON_MAINNET_RPC_URL" npx hardhat --network polygon task:verifyConfidentialOFT --use-internal-proxy-address true
```

Export the two OFT addresses for the next step:

```bash
export ETHEREUM_OFT_ADDRESS=$(grep '^CONFIDENTIAL_OFT_CONTRACT_ADDRESS=' addresses-ethereum/.env.host | cut -d= -f2)
export POLYGON_OFT_ADDRESS=$(grep '^CONFIDENTIAL_OFT_CONTRACT_ADDRESS=' addresses-polygon/.env.host | cut -d= -f2)
```

### 5.3 — Set each remote OFT as the peer

Each `ConfidentialOFT` resolves its destination peer internally from a single
peer-per-eid registry (`setPeer`). The same registry authenticates inbound
mints: `onConfidentialBridgeReceived` rejects any `(srcEid, srcApp)` that doesn't match the
configured peer (`UntrustedPeer`), and `send` reverts with `PeerNotSet` for an
eid with no configured peer. Call `task:wireConfidentialOFT` once per direction.

```bash
# Ethereum → set the Polygon OFT as peer
cp addresses-ethereum/{FHEVMHostAddresses.sol,.env.host} addresses/

RPC_URL="$ETHEREUM_MAINNET_RPC_URL" \
  npx hardhat --network mainnet task:wireConfidentialOFT \
  --remote-eid 30109 \
  --remote-oft "$POLYGON_OFT_ADDRESS"

# Polygon → set the Ethereum OFT as peer
cp addresses-polygon/{FHEVMHostAddresses.sol,.env.host} addresses/

RPC_URL="$POLYGON_MAINNET_RPC_URL" \
  npx hardhat --network polygon task:wireConfidentialOFT \
  --remote-eid 30101 \
  --remote-oft "$ETHEREUM_OFT_ADDRESS"
```

### 5.4 — End-to-end send: seed balance via `mint`, bridge to the other chain

`ConfidentialOFT` exposes an `onlyOwner` `mint(address to, externalEuint64
encryptedAmount, bytes inputProof)` for bootstrapping the initial supply on
each chain. The encrypted `(handle, inputProof)` bundle is
produced off-chain by the mock coprocessor's `mock:encrypt` CLI, which
emulates the relayer / input-verifier path.

#### 5.4.1 — Mint an initial balance on Ethereum Mainnet

```bash
# Resolve a deployer address for convenience.
export DEPLOYER_PRIVATE_KEY=$(grep '^DEPLOYER_PRIVATE_KEY=' .env | sed -E 's/^[^"]*"([^"]*)".*/\1/')
export DEPLOYER_ADDRESS=$(cast wallet address "$DEPLOYER_PRIVATE_KEY")

# Build a single euint64=1000 encrypted input bound to (OFT, deployer) on Ethereum mainnet
# and insert its cleartext into the mock DB. Prints handle=… inputProof=… on stdout.
pnpm mock:encrypt \
  --contract "$ETHEREUM_OFT_ADDRESS" \
  --user    "$DEPLOYER_ADDRESS" \
  --type    euint64 \
  --value   1000 \
  --host-chain-id 1
```

Carefully note the values logged for `handle` and its corresponding `inputProof`.
Now call the `mint` function on the cOFT instance deployed on Ethereum mainnet while reusing those value:

```bash
RPC_URL="$ETHEREUM_MAINNET_RPC_URL" npx hardhat task:mintCOFT --coft "$ETHEREUM_OFT_ADDRESS" --handle <INPUT_HANDLE> --input-proof <INPUT_PROOF> --network mainnet
```

Once the mint transaction is processed, you should see on the daemon's terminal some log such as:

```
[mock-coprocessor:ethereum] processed blocks XXXXX-XXXXX (head=XXXXX): inserted=2 pending=0 skipped=0
```

You should then be able to decrypt your current balance on Ethereum mainnet:

```bash
pnpm mock:query <balance handle from task:mintCOFT logs>
# → 1000
```

If the result isn't 1000, something went wrong; consult the daemon log.

#### 5.4.2 — Bridge it to Polygon Mainnet

Once you have a non-zero balance handle on Ethereum mainnet, bridge your whole balance
to Polygon mainnet with `task:bridgeCOFT`:

```bash
RPC_URL="$ETHEREUM_MAINNET_RPC_URL" npx hardhat task:bridgeCOFT \
  --coft "$ETHEREUM_OFT_ADDRESS" \
  --dst-eid 30109 \
  --dst-oft "$POLYGON_OFT_ADDRESS" \
  --network mainnet
```

LayerZero delivery on mainnet typically takes a few minutes. Track it on
[LayerZero Scan](https://layerzeroscan.com/).

A reliable way to track the cross-chain transfer is to keep watching whenever your balance on Polygon changes — initially
it should be an uninitialized (i.e. zero) handle. Run this command several times if needed, the printed `bytes32` will flip from `0x0000…0000` to a real handle the moment the bridge delivers and the OFT mints on Polygon:

```bash
cast call "$POLYGON_OFT_ADDRESS" "balanceOf(address)(bytes32)" "$DEPLOYER_ADDRESS" --rpc-url "$POLYGON_MAINNET_RPC_URL"
```

Note also that when the destination side fires, the daemon terminal should log something like:

```
[mock-coprocessor:polygon] processed blocks YYYYY-YYYYY (head=YYYYY): inserted=1 pending=0 skipped=0
```

Finally you can decrypt your bridged balance (i.e the non-null bytes32 logged via previous `cast` command) on Polygon via:

```bash
pnpm mock:query <balance handle on Polygon>
# → 1000
```

A printed `1000` on Polygon confirms: the OFT burned on Ethereum, the bridge
delivered the handle association via LayerZero, the OFT minted on Polygon, and
the mock coprocessor propagated the cleartext end-to-end.

### Step 6 (Optional) - Run gas profiler for `lzReceive` parameters

In order to get the recommended `lzReceive`-specific gas parameters from `HandlesSender` (i.e `LZ_RECEIVE_BASE_GAS_DEFAULT`, `LZ_RECEIVE_PER_HANDLE_GAS_DEFAULT` and `LZ_RECEIVE_PER_PAYLOAD_BYTE_DEFAULT`) you can run the following script:

On Ethereum mainnet:

```
PROFILE_RPC_URL="$ETHEREUM_MAINNET_RPC_URL" \
PROFILE_RECEIVER="$ETHEREUM_BRIDGE_ADDRESS" \
PROFILE_SENDER="$POLYGON_BRIDGE_ADDRESS" \
PROFILE_SRC_EID=30109 \
PROFILE_DST_EID=30101 \
PROFILE_ENDPOINT=0x1a44076050125825900e736c501f859c50fE728c \
forge script scripts/HandlesReceiverProfiler.s.sol:HandlesReceiverProfilerExample -vv
```

On Polygon mainnet:

```
PROFILE_RPC_URL="$POLYGON_MAINNET_RPC_URL" \
PROFILE_RECEIVER="$POLYGON_BRIDGE_ADDRESS" \
PROFILE_SENDER="$ETHEREUM_BRIDGE_ADDRESS" \
PROFILE_SRC_EID=30101 \
PROFILE_DST_EID=30109 \
PROFILE_ENDPOINT=0x1a44076050125825900e736c501f859c50fE728c \
forge script scripts/HandlesReceiverProfiler.s.sol:HandlesReceiverProfilerExample -vv
```

#### Verify the fitted parameters cover the whole input domain (WARNING: this can take more than 20 minutes to run until completion)

The profiler fits the three coefficients (`base`, `perHandle`, `perByte`) from a **coarse** grid. To gain confidence that the resulting formula `base + perHandle·nHandles + perByte·payloadLen` is an **upper bound** on the real `lzReceive` gas for _every_ admissible input, use `scripts/verify_lzreceive_budget.sh`. It measures each `(nHandles, payloadLen)` cell through the same real `EndpointV2.lzReceive` path as the profiler and asserts the budget covers the measured gas, sweeping all handle counts values in `1..32` and every single payload length value in `[0, 10000]`.

Pass the wiring exactly as for the profiler (the fork must be the **destination** chain, matching `PROFILE_RECEIVER` / `PROFILE_DST_EID`), plus the three fitted coefficients you want to validate : use the margin-included **RECOMMENDED** values the profiler printed previously for the 3 arguments `<RECOMMENDED_LZ_RECEIVE_BASE_GAS>`, `<RECOMMENDED_LZ_RECEIVE_PER_HANDLE_GAS>` and `<RECOMMENDED_LZ_RECEIVE_PER_PAYLOAD_BYTE_GAS>`:

```bash
# Polygon verifier command
PROFILE_RPC_URL="$POLYGON_MAINNET_RPC_URL" \
PROFILE_ENDPOINT=0x1a44076050125825900e736c501f859c50fE728c \
PROFILE_RECEIVER="$POLYGON_BRIDGE_ADDRESS" \
PROFILE_SENDER="$ETHEREUM_BRIDGE_ADDRESS" \
PROFILE_SRC_EID=30101 PROFILE_DST_EID=30109 \
VERIFY_BASE_GAS=<RECOMMENDED_LZ_RECEIVE_BASE_GAS> \
VERIFY_PER_HANDLE_GAS=<RECOMMENDED_LZ_RECEIVE_PER_HANDLE_GAS> \
VERIFY_PER_PAYLOAD_BYTE_GAS=<RECOMMENDED_LZ_RECEIVE_PER_PAYLOAD_BYTE_GAS> \
bash scripts/verify_lzreceive_budget.sh

# Ethereum verifier command
PROFILE_RPC_URL="$ETHEREUM_MAINNET_RPC_URL" \
PROFILE_ENDPOINT=0x1a44076050125825900e736c501f859c50fE728c \
PROFILE_RECEIVER="$ETHEREUM_BRIDGE_ADDRESS" \
PROFILE_SENDER="$POLYGON_BRIDGE_ADDRESS" \
PROFILE_SRC_EID=30109 PROFILE_DST_EID=30101 \
VERIFY_BASE_GAS=<RECOMMENDED_LZ_RECEIVE_BASE_GAS> \
VERIFY_PER_HANDLE_GAS=<RECOMMENDED_LZ_RECEIVE_PER_HANDLE_GAS> \
VERIFY_PER_PAYLOAD_BYTE_GAS=<RECOMMENDED_LZ_RECEIVE_PER_PAYLOAD_BYTE_GAS> \
bash scripts/verify_lzreceive_budget.sh
```

A clean run ends with `PASS: every cell ... is within budget`. If any cell exceeds the budget, the offending `(nHandles, payloadLen)` is printed and the script exits non-zero.

> **⚠️ Use a paid/dedicated RPC, and expect it to run for a few hours.** The exhaustive sweep performs roughly `32 × 10001 ≈ 320k` real `lzReceive` calls against a fork of the destination chain, each fetching cold state over the network — far beyond the rate limits of free public endpoints. To keep host memory bounded the script runs the sweep in **batches, each in its own short-lived `forge` process** (a single monolithic run gets OOM-killed: `zsh: killed`). When a batch is still OOM-killed it is automatically subdivided and retried, so no manual tuning is needed.

> The script exits with such an error (see below) **only** if it finds a counter-example — a `(nHandles, payloadLen)` pair whose measured gas exceeds the budget, meaning the fitted coefficients are insufficient. In that case it stops at the first offending cell and prints a report of the form below. This should not happen with coefficients obtained from the profiler; if it does, re-run the profiler from the previous step with a finer grid to derive larger parameters, then re-verify.

```
# only this kind of Error output shows that the budget parameters are ill-fitted
     slack: NEGATIVE by 45424
  RESULT: FAIL - first under-budget cell  n / len: 1 0
     measured / budget: 56525 11525
Error: script failed: budget insufficient: see FAIL log above
FAIL: forge exited 1 on payloadLen in [0, 249] -- verifier reverted (under-budget)
```

---

## Step 7 (Optional) — Deploy and test `HandlesListConfidentialOApp`

`HandlesListConfidentialOApp` is a second minimal example app riding on
the same bridge. Instead of tracking balances, it bridges an arbitrary _list_ of
FHE handles: `generateAndSendHandlesList(...)` mints `countHandles` fresh
encrypted `euint32` values on-chain via `FHE.randEuint32`, grants itself (and the
owner) ACL allowance on each, and bridges that list to the peer. On the
destination chain, `onConfidentialBridgeReceived(...)` grants the owner decryption rights on
the derived destination handles and emits them in an `HandlesListConfidentialOAppReceived` event.

Because the values are generated on-chain, there is **no `mint` / `mock:encrypt`
step**: the only handle-related input is `countHandles`. The same deployment embeds both
the send and receive paths, so a single instance per chain bridges in both
directions.

> **Prerequisites.** This reuses everything from steps 1–4: the host stack on
> both chains (steps 1–2), the wired `ConfidentialBridge` instances including the
> `dstChainId` mapping (step 3), and a **running mock coprocessor daemon**
> (step 4). The daemon is live-events-only, so it must be up _before_ you submit
> the send in step 7.4.

### 7.1 — Deploy on Ethereum Mainnet

```bash
cp addresses-ethereum/{FHEVMHostAddresses.sol,.env.host} addresses/

npx hardhat clean

npx hardhat compile

npx hardhat compile:specific --contract examples

RPC_URL="$ETHEREUM_MAINNET_RPC_URL" npx hardhat --network mainnet task:deployHandlesListConfidentialOApp

cp addresses/.env.host addresses-ethereum/.env.host
```

The new address is appended to `addresses/.env.host` as
`HANDLES_LIST_OAPP_CONTRACT_ADDRESS`. The constructor wires the app to the local
bridge it just read from the same file, and sets the deployer wallet as the app's
`Ownable2Step` owner.

Then, if you want to verify the contract on Etherscan (optional):

```bash
RPC_URL="$ETHEREUM_MAINNET_RPC_URL" npx hardhat --network mainnet task:verifyHandlesListConfidentialOApp --use-internal-proxy-address true
```

### 7.2 — Deploy on Polygon Mainnet

```bash
cp addresses-polygon/{FHEVMHostAddresses.sol,.env.host} addresses/

npx hardhat clean

npx hardhat compile

npx hardhat compile:specific --contract examples

RPC_URL="$POLYGON_MAINNET_RPC_URL" npx hardhat --network polygon task:deployHandlesListConfidentialOApp

cp addresses/.env.host addresses-polygon/.env.host
```

Then, if you want to verify the contract on PolygonScan (optional):

```bash
RPC_URL="$POLYGON_MAINNET_RPC_URL" npx hardhat --network polygon task:verifyHandlesListConfidentialOApp --use-internal-proxy-address true
```

Export the two app addresses for the next steps:

```bash
export ETHEREUM_HANDLES_OAPP_ADDRESS=$(grep '^HANDLES_LIST_OAPP_CONTRACT_ADDRESS=' addresses-ethereum/.env.host | cut -d= -f2)
export POLYGON_HANDLES_OAPP_ADDRESS=$(grep '^HANDLES_LIST_OAPP_CONTRACT_ADDRESS=' addresses-polygon/.env.host | cut -d= -f2)
```

### 7.3 — Set each remote app as the peer

Like the OFT, each app resolves its destination peer from a single
peer-per-eid registry (`setPeer`), which also authenticates inbound deliveries:
`onConfidentialBridgeReceived` rejects any `(srcEid, srcApp)` that doesn't match
the configured peer (`UntrustedPeer`), and `generateAndSendHandlesList` reverts
with `PeerNotSet` for an eid with no configured peer. Call
`task:wireHandlesListConfidentialOApp` once per direction.

```bash
# Ethereum → set the Polygon app as peer
cp addresses-ethereum/{FHEVMHostAddresses.sol,.env.host} addresses/

RPC_URL="$ETHEREUM_MAINNET_RPC_URL" \
  npx hardhat --network mainnet task:wireHandlesListConfidentialOApp \
  --remote-eid 30109 \
  --remote-app "$POLYGON_HANDLES_OAPP_ADDRESS"

# Polygon → set the Ethereum app as peer
cp addresses-polygon/{FHEVMHostAddresses.sol,.env.host} addresses/

RPC_URL="$POLYGON_MAINNET_RPC_URL" \
  npx hardhat --network polygon task:wireHandlesListConfidentialOApp \
  --remote-eid 30101 \
  --remote-app "$ETHEREUM_HANDLES_OAPP_ADDRESS"
```

### 7.4 — End-to-end send: generate a handle list on Ethereum Mainnet, bridge to Polygon

`task:sendHandlesList` quotes the LayerZero fee, calls
`generateAndSendHandlesList` (which mints the random handles on-chain and bridges
them), and prints the freshly generated source-side handles parsed from the
`HandlesListConfidentialOAppSent` event. The owner is ACL-allowed on each, so you
can decrypt them on the source chain.

```bash
RPC_URL="$ETHEREUM_MAINNET_RPC_URL" npx hardhat task:sendHandlesList \
  --app "$ETHEREUM_HANDLES_OAPP_ADDRESS" \
  --dst-eid 30109 \
  --dst-app "$POLYGON_HANDLES_OAPP_ADDRESS" \
  --count 2 \
  --payload-length 512 \
  --network mainnet
```

`--count` controls how many handles are generated and bridged (capped by the
bridge's `MAX_HANDLES`); `--payload-length` (default `0`) attaches an opaque
app-level blob of that many `0xff` bytes (forwarded verbatim to the destination
peer, useful for exercising the message-size impact on the fee). The
destination-side `lzCompose` gas budget is derived automatically from a simple
linear over-estimate (`base + perHandle·count + perByte·payloadLength`), so it
scales with both `--count` and `--payload-length`.

Once the send transaction is processed, the daemon's terminal logs the source-side
handle insertions (one per generated handle):

```
[mock-coprocessor:ethereum] processed blocks XXXXX-XXXXX (head=XXXXX): inserted=2 pending=0 skipped=0
```

You can already decrypt each printed source handle on Ethereum mainnet (they hold random
values; note them down to compare with the destination side later):

```bash
pnpm mock:query <source handle from task:sendHandlesList logs>
# → some random uint32, e.g. 2718281828
```

### 7.5 — Confirm delivery and decrypt on Polygon

LayerZero delivery on mainnet typically takes a few minutes. Track it on
[LayerZero Scan](https://layerzeroscan.com/), but as in step 5.4.2 a reliable
signal is to poll the destination app directly.

Once the destination side fires, the daemon terminal logs the derived-handle
insertions on Polygon:

```
[mock-coprocessor:polygon] processed blocks YYYYY-YYYYY (head=YYYYY): inserted=2 pending=0 skipped=0
```

Recover the destination handles from the most recent inbound delivery with
`task:readReceivedHandlesList`. To keep the destination-side `lzCompose` gas low,
the app does **not** persist the received arrays on-chain — it only commits a
single `keccak256(srcHandles, dstHandles, payload)` per delivery in
`resultBridgedHash[guid]`. The handles themselves are recovered off-chain from the
`HandlesListConfidentialOAppReceived` event, which this task scans and prints
(along with the on-chain commitment hash for cross-checking):

```bash
RPC_URL="$POLYGON_MAINNET_RPC_URL" npx hardhat task:readReceivedHandlesList \
  --app "$POLYGON_HANDLES_OAPP_ADDRESS" \
  --network polygon
```

If nothing is found yet, the bridge hasn't delivered — wait and retry, or widen
the overall scan window with `--from-block <number>`.

Finally decrypt each destination handle on Polygon:

```bash
pnpm mock:query <destination handle on Polygon>
# → matches the corresponding source value decrypted in step 7.4
```

Each destination value matching its source counterpart confirms the full path:
the app generated the encrypted list on Ethereum, the bridge delivered the handle
associations via LayerZero, the destination app committed the delivery hash and
re-granted the derived handles on Polygon, and the mock coprocessor propagated the
cleartext end-to-end.

### 7.6 — Stress test the bridge across a matrix of sizes

`scripts/stress/handlesListBridgeStress.ts` exercises the already-deployed-and-wired
`HandlesListConfidentialOApp` pair across a matrix of message sizes and reports the
cross-chain delivery status of each transfer. It fires one
`generateAndSendHandlesList` per couple of:

- `handleCounts = [1, 2, 4, 8, 16, 32]`
- `payloadLens  = [0, 1, 64, 256, 1024, 8192]`

i.e. **36 bridging transactions** from the chosen source chain, then waits (max
**10 minutes**, otherwise it throws a timeout error) while polling the destination
chain for the outcome of every transfer.

The script is environment-aware: `BRIDGE_ENV=mainnet` selects the Ethereum/Polygon
mainnet pair, reads RPC URLs from `ETHEREUM_MAINNET_RPC_URL` / `POLYGON_MAINNET_RPC_URL`,
defaults to the mainnet EndpointV2 address, and resolves the source/destination from
the `ethereum` / `polygon` argument.

The script keys everything off the LayerZero **guid**, derived on the source side
from the EndpointV2 `PacketSent(encodedPayload, …)` event (the guid is embedded in
`encodedPayload`). On the destination chain it listens only to
EndpointV2 events:

| Destination event (EndpointV2) | Meaning                             | Treated as              |
| ------------------------------ | ----------------------------------- | ----------------------- |
| `ComposeDelivered(guid)`       | both `lzReceive` and `lzCompose` ok | `SUCCESS` (terminal)    |
| `LzReceiveAlert(guid)`         | an `lzReceive` attempt failed       | transient retry (count) |
| `LzComposeAlert(guid)`         | an `lzCompose` attempt failed       | transient retry (count) |

`PacketDelivered` (the `lzReceive`-success event) is deliberately **not** tracked:
the bridge's `_lzReceive` always calls `endpoint.sendCompose(...)`, so every
successful `lzReceive` is necessarily followed by a compose outcome
(`ComposeDelivered` or `LzComposeAlert`).

> **Why alerts are transient, not failures.** The LayerZero **executor automatically
> retries** failed legs, so an alert is only a "last attempt failed" signal, not a
> verdict. Firing 36 messages back-to-back makes a few legs hit a transient out-of-gas
> (the per-message gas cost varies slightly with how the nonces are processed), so you'll
> often see one or two non-deterministic `LzReceiveAlert`s that the executor then retries
> and that always end up succeeding. The script
> therefore treats **only `ComposeDelivered` as terminal SUCCESS**, just counts the
> alerts (shown in the `alerts` column as `Nr/Mc` = lzReceive/lzCompose retries), and
> falls back to the last alerting leg (`RECEIVE_FAILED` / `COMPOSE_FAILED`) **only** for
> guids that never reach `SUCCESS` before the 10-minute deadline (those are genuine
> persistent failures, e.g. an `lzCompose` out-of-gas, which keeps re-alerting and never
> delivers).

Prerequisites: the app must be deployed and wired on **both** chains (steps
7.1–7.3), the mock daemon (step 4) running, and the deployer funded on the source
chain with enough native gas to cover 36 LayerZero fees (the larger
`count`/`payload-length` couples cost more). With `BRIDGE_ENV=mainnet` it reads RPC
URLs from your env (`ETHEREUM_MAINNET_RPC_URL`, `POLYGON_MAINNET_RPC_URL`) and the
app/bridge addresses from `addresses-{ethereum,polygon}/.env.host`.

Run it with the source chain as the only argument (the other chain is the
destination):

```bash
# Ethereum → Polygon
BRIDGE_ENV=mainnet pnpm stress:handlesList ethereum

# Polygon → Ethereum
BRIDGE_ENV=mainnet pnpm stress:handlesList polygon
```

When all 36 transfers reach `SUCCESS` (or the 10-minute deadline is hit), it prints a
table summarizing each transfer (row number `Tx#`, `count`, `payloadLen`, `composeGas`,
`fee_wei`, `alerts`, short `guid`, `status`) plus a per-status tally, e.g.:

```
=== HandlesListConfidentialOApp bridge stress results ===
+-----+-------+------------+------------+-----------+--------+-----------------+---------+
| Tx# | count | payloadLen | composeGas | fee_wei   | alerts | guid            | status  |
+-----+-------+------------+------------+-----------+--------+-----------------+---------+
| 1   | 1     | 0          | 300000     | 12345     | 0r/0c  | 0x1234ab…cdef01 | SUCCESS |
| 6   | 1     | 8192       | 709600     | 999999999 | 1r/0c  | 0x9f02cc…aa12be | SUCCESS |
| …   | …     | …          | …          | …         | …      | …               | …       |
+-----+-------+------------+------------+-----------+--------+-----------------+---------+
Summary: { SUCCESS: 36 }
```

A non-zero `alerts` value on a `SUCCESS` row (eg: an alerts of `1r/2c` means that the
`lzReceive` leg was retried once, and the `lzCompose` leg was retired twice) is just the
transient blip-then-retry described above — the executor healed it and
the row still ended `SUCCESS`. Only act on a row that ends `COMPOSE_FAILED` / `RECEIVE_FAILED`
(kept re-alerting until the deadline and never reached `SUCCESS`): inspect the failing leg's
revert reason and, if it's an out-of-gas error, raise the gas
budget for the failing leg, then re-run. A `TIMEOUT` row never produced any terminal event
within 10 minutes (probably because the DVN or Executor service is unavailable).
Finally, a row may also never leave the source chain: `SEND_FAILED`
means the send tx reverted or never confirmed (e.g. insufficient native balance for the
fee), and `NO_PACKET_SENT` means it was mined but no `PacketSent` log was found — this should never happen.
