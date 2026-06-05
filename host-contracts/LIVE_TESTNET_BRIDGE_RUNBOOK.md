# Live testnet runbook: Sepolia + Polygon Amoy

End-to-end procedure for spinning up the full FHEVM host stack on two EVM
testnets, wiring the `ConfidentialBridge` between them with production-grade
LayerZero V2 settings, running the mock coprocessor daemon, and exercising a
confidential OFT bridging using `ConfidentialOFT` contract instances.

Target chains used as the running example:

| Chain            | EVM chainId | LZ V2 EID |
| ---------------- | ----------- | --------- |
| Ethereum Sepolia | `11155111`  | `40161`   |
| Polygon Amoy     | `80002`     | `40267`   |

The procedure generalizes to any chain pair that has a canonical LayerZero V2
endpoint. The LZ V2 endpoint address is the same on every chain:
`0x6EDCE65403992e310A62460808c4b910D972f10f`.

> **Scope.** This runbook covers the host-contract side of the stack plus the
> mock coprocessor needed to make FHE operations observable in plaintext. The
> production coprocessor (signers, KMS endpoints, …) is out of scope — see
> the protocol RFCs for that.

## Prerequisites

### Wallets and funds

A single deployer key is reused across both chains. Make sure to have pre-funded your deployer account with enough Sepolia ETH and Amoy POL.

First step, after installing dependencies, is to copy the `.env.example` to `.env`:

```bash
cp .env.example .env
```

Keep all the default values inside this newly created `.env` file, with the exception of only both variables `DEPLOYER_PRIVATE_KEY` and `ETHERSCAN_API_KEY` which you must replace with your own values.

---

## Step 1 — Deploy host stack on Sepolia

Then deploy all host contracts on Sepolia with (make sure to replace `<YOUR_SEPOLIA_RPC_URL>` by your own Sepolia RPC url):

```bash
export SEPOLIA_RPC_URL=<YOUR_SEPOLIA_RPC_URL>
RPC_URL="$SEPOLIA_RPC_URL" npx hardhat --network sepolia task:deployAllHostContracts --with-kms-generation true
```

Then verify all Sepolia contracts on Etherscan via:

```bash
RPC_URL="$SEPOLIA_RPC_URL" npx hardhat task:verifyAllHostContracts --use-internal-proxy-address true --network sepolia
```

After contract verification, save the Sepolia addresses in a dedicated folder:

```bash
mkdir -p addresses-sepolia
cp -r addresses/ addresses-sepolia/
```

The `cp` is **critical** — the next chain's deploy overwrites `addresses/`.

## Step 2 — Deploy host stack on Polygon Amoy

Deploy all host contracts on Polygon Amoy with (make sure to replace `<YOUR_POLYGON_AMOY_RPC_URL>` by your own Polygon Amoy RPC url):

```bash
export POLYGON_AMOY_RPC_URL=<YOUR_POLYGON_AMOY_RPC_URL>
RPC_URL="$POLYGON_AMOY_RPC_URL" npx hardhat --network polygonAmoy task:deployAllHostContracts --with-kms-generation false
```

Then verify all Polygon Amoy contracts on Etherscan via:

```bash
RPC_URL="$POLYGON_AMOY_RPC_URL" npx hardhat task:verifyAllHostContracts --use-internal-proxy-address true --network polygonAmoy
```

After contract verification, save the Sepolia addresses in a dedicated folder:

```bash
mkdir -p addresses-amoy
cp -r addresses/ addresses-amoy/
```

Note each chain's `CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS` from `addresses-{sepolia,amoy}/.env.host` — you'll use both in step 3.

---

## Step 3 — Wire the ConfidentialBridge instances

`lz:oapp:wire` (in `lz-wiring/`) handles peers, send/receive libraries, DVNs,
Executor config, confirmations and enforced options. It does **not** set the
bridge-specific `dstChainId` mapping (an FHEVM extension to the OApp
interface), so there's an extra call per chain at the end.

### 3.1 — Export the bridge addresses

First run this command to extract the ConfidentialBridge addresses instances on both chains:

```bash
export SEPOLIA_BRIDGE_ADDRESS=$(grep '^CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS=' addresses-sepolia/.env.host | cut -d= -f2)
export POLYGON_AMOY_BRIDGE_ADDRESS=$(grep '^CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS=' addresses-amoy/.env.host | cut -d= -f2)
echo "Sepolia bridge: $SEPOLIA_BRIDGE_ADDRESS"
echo "Amoy bridge:    $POLYGON_AMOY_BRIDGE_ADDRESS"
```

Then install with `pnpm` (recommended package manager for LayerZero) the needed dependencies for the LayerZero wiring task:

```bash
cd lz-wiring
pnpm i
```

Now replace the address field values in both `lz-wiring/deployments/{polygonAmoy,sepolia}/ConfidentialBridge.json` files by their actual values logged earlier by their corresponding values, i.e replace the `<ConfidentialBridgeAddress>` by their actual values for both chains:

```
{
  "address": "<ConfidentialBridgeAddress>" <-- replace by actual value on Sepolia/PolygonAmoy
}
```

And finally copy the ABI of the ConfidentialBridge to those json files by running (inside `lz-wiring` directory):

```bash
npx ts-node scripts/copyAbiToDeployments.ts
```

### 3.2 — Wiring the bridges

While still in the `lz-wiring` directory, and after checking the config inside `layerzero.config.testnet.ts`[./layerzero.config.testnet.ts] is correct - it is crucial to double check the values for required DVNs and number of confirmations for security, run:

```bash
npx hardhat lz:oapp:wire --oapp-config layerzero.config.testnet.ts
```

The CLI submits one tx per change, on each chain, signed by `DEPLOYER_PRIVATE_KEY`.

### 3.3 — Set the bridge-specific `dstChainId`

```bash
cd ..   # back to host-contracts root

# Sepolia → Amoy
RPC_URL="$SEPOLIA_RPC_URL" \
  npx hardhat --network sepolia task:setDstChainId \
  --bridge-address "$SEPOLIA_BRIDGE_ADDRESS" \
  --remote-eid 40267 \
  --remote-chain-id 80002

# Amoy → Sepolia
RPC_URL="$POLYGON_AMOY_RPC_URL" \
  npx hardhat --network polygonAmoy task:setDstChainId \
  --bridge-address "$POLYGON_AMOY_BRIDGE_ADDRESS" \
  --remote-eid 40161 \
  --remote-chain-id 11155111
```

---

## Step 4 — Start the mock coprocessor daemon

**Open a dedicated terminal** and keep it running. The daemon is
**live-events-only**: it seeds its cursor at each chain's current head on
startup and never catches up missed events. Any test transaction submitted
before the daemon's `initialised at chain head N` line will be missed.

```bash
export SEPOLIA_RPC_URL=<YOUR_SEPOLIA_RPC_URL>
export POLYGON_AMOY_RPC_URL=<YOUR_POLYGON_AMOY_RPC_URL>
npm run mock:daemon
```

Wait until you see two `initialised at chain head …` lines (one per chain):

```
[mock-coprocessor:sepolia]     initialised at chain head 9301245 — only events from block 9301246 onwards will be processed
[mock-coprocessor:polygonAmoy] initialised at chain head 15403200 — only events from block 15403201 onwards will be processed
```

Only proceed once both have printed. Leave the daemon alive for the rest of
the runbook.

---

## Step 5 — Deploy and test `ConfidentialOFT`

`ConfidentialOFT` is a minimalistic cross-chain confidential token that rides
on top of the bridge: each chain holds encrypted per-user balances, and
`send(...)` burns from the sender on the source chain while the destination
chain's `onReceive(...)` mints to the recipient.

### 5.1 — Deploy on Sepolia

```bash
cp addresses-sepolia/{FHEVMHostAddresses.sol,.env.host} addresses/

npx hardhat clean

npx hardhat compile

npx hardhat compile:specific --contract examples

RPC_URL="$SEPOLIA_RPC_URL" npx hardhat --network sepolia task:deployConfidentialOFT

cp addresses/.env.host addresses-sepolia/.env.host
```

The new address is appended to `addresses/.env.host` as
`CONFIDENTIAL_OFT_CONTRACT_ADDRESS`. The constructor wires the OFT to the
local bridge it just read from the same file, and sets the deployer wallet as
the OFT's `Ownable2Step` owner.

Then, if you want to verify the contract on Sepolia Etherscan (optional):

```bash
RPC_URL="$SEPOLIA_RPC_URL" npx hardhat --network sepolia task:verifyConfidentialOFT --use-internal-proxy-address true
```

### 5.2 — Deploy on Polygon Amoy

```bash
cp addresses-amoy/{FHEVMHostAddresses.sol,.env.host} addresses/

npx hardhat clean

npx hardhat compile

npx hardhat compile:specific --contract examples

RPC_URL="$POLYGON_AMOY_RPC_URL" npx hardhat --network polygonAmoy task:deployConfidentialOFT

cp addresses/.env.host addresses-amoy/.env.host
```

Then, if you want to verify the contract on Amoy Etherscan (optional):

```bash
RPC_URL="$POLYGON_AMOY_RPC_URL" npx hardhat --network polygonAmoy task:verifyConfidentialOFT --use-internal-proxy-address true
```

Export the two OFT addresses for the next step:

```bash
export SEPOLIA_OFT_ADDRESS=$(grep '^CONFIDENTIAL_OFT_CONTRACT_ADDRESS=' addresses-sepolia/.env.host | cut -d= -f2)
export POLYGON_AMOY_OFT_ADDRESS=$(grep '^CONFIDENTIAL_OFT_CONTRACT_ADDRESS=' addresses-amoy/.env.host | cut -d= -f2)
```

### 5.3 — Trust each remote OFT as a peer

`ConfidentialOFT.onReceive` rejects any inbound `(srcEid, srcApp)` pair the
owner hasn't whitelisted (`UntrustedPeer`). Call `task:wireConfidentialOFT`
once per direction.

```bash
# Sepolia → trust the Amoy OFT
cp addresses-sepolia/{FHEVMHostAddresses.sol,.env.host} addresses/

RPC_URL="$SEPOLIA_RPC_URL" \
  npx hardhat --network sepolia task:wireConfidentialOFT \
  --remote-eid 40267 \
  --remote-oft "$POLYGON_AMOY_OFT_ADDRESS"

# Amoy → trust the Sepolia OFT
cp addresses-amoy/{FHEVMHostAddresses.sol,.env.host} addresses/

RPC_URL="$POLYGON_AMOY_RPC_URL" \
  npx hardhat --network polygonAmoy task:wireConfidentialOFT \
  --remote-eid 40161 \
  --remote-oft "$SEPOLIA_OFT_ADDRESS"
```

### 5.4 — End-to-end send: seed balance via `mint`, bridge to the other chain

`ConfidentialOFT` exposes an `onlyOwner` `mint(address to, externalEuint64
encryptedAmount, bytes inputProof)` for bootstrapping the initial supply on
each chain. The encrypted `(handle, inputProof)` bundle is
produced off-chain by the mock coprocessor's `mock:encrypt` CLI, which
emulates the relayer / input-verifier path.

#### 5.4.1 — Mint an initial balance on Sepolia

```bash
# Resolve a deployer address for convenience.
export DEPLOYER_PRIVATE_KEY=$(grep '^DEPLOYER_PRIVATE_KEY=' .env | sed -E 's/^[^"]*"([^"]*)".*/\1/')
export DEPLOYER_ADDRESS=$(cast wallet address "$DEPLOYER_PRIVATE_KEY")

# Build a single euint64=1000 encrypted input bound to (OFT, deployer) on Sepolia
# and insert its cleartext into the mock DB. Prints handle=… inputProof=… on stdout.
pnpm mock:encrypt \
  --contract "$SEPOLIA_OFT_ADDRESS" \
  --user    "$DEPLOYER_ADDRESS" \
  --type    euint64 \
  --value   1000 \
  --host-chain-id 11155111
```

Carefully note the values logged for `handle` and its corresponding `inputProof`.
Now call the `mint` function on the cOFT instance deployed on Sepolia while reusing those value:

```bash
RPC_URL="$SEPOLIA_RPC_URL" npx hardhat task:mintCOFT --coft "$SEPOLIA_OFT_ADDRESS" --handle <INPUT_HANDLE> --input-proof <INPUT_PROOF> --network sepolia
```

Once the mint transaction is processed, you should see on the daemon's terminal some log such as:

```
[mock-coprocessor:sepolia] processed blocks XXXXX-XXXXX (head=XXXXX): inserted=2 pending=0 skipped=0
```

You should then be able to decrypt your current balance on Sepolia:

```bash
pnpm mock:query <balance handle from task:mintCOFT logs>
# → 1000
```

If the result isn't 1000, something went wrong; consult the daemon log.

#### 5.4.2 — Bridge it to Amoy

Once you have a non-zero balance handle on Sepolia, bridge your whole balance
to Polygon Amoy with `task:bridgeCOFT`:

```bash
RPC_URL="$SEPOLIA_RPC_URL" npx hardhat task:bridgeCOFT \
  --coft "$SEPOLIA_OFT_ADDRESS" \
  --dst-eid 40267 \
  --dst-oft "$POLYGON_AMOY_OFT_ADDRESS" \
  --network sepolia
```

LayerZero delivery on testnet typically takes up to **10 minutes**. Track it on
[LayerZero Scan Testnet](https://testnet.layerzeroscan.com/).

**WARNING:** Beware that on testnet the layerzeroscan website is very poorly responsive, so a better way to track the cross-chain transfer is to keep watching whenever your balance on Amoy changes — initially
it should be an uninitialized (i.e. zero) handle. Run this command several times if needed, the printed `bytes32` will flip from `0x0000…0000` to a real handle the moment the bridge delivers and the OFT mints on Amoy:

```bash
cast call "$POLYGON_AMOY_OFT_ADDRESS" "balanceOf(address)(bytes32)" "$DEPLOYER_ADDRESS" --rpc-url "$POLYGON_AMOY_RPC_URL"
```

Note also that when the destination side fires, the daemon termina should log something like:

```
[mock-coprocessor:polygonAmoy] processed blocks YYYYY-YYYYY (head=YYYYY): inserted=1 pending=0 skipped=0
```

Finally you can decrypt your bridged balance (i.e the non-null bytes32 logged via previous `cast` command) on Amoy via:

```bash
pnpm mock:query <balance handle on Amoy>
# → 1000
```

A printed `1000` on Amoy confirms: the OFT burned on Sepolia, the bridge
delivered the handle association via LayerZero, the OFT minted on Amoy, and
the mock coprocessor propagated the cleartext end-to-end.
