This page covers deploying FHEVM contracts with Foundry — to a local Anvil node or to Sepolia.

## Deploy to Sepolia

The Sepolia FHEVM stack is already deployed at the canonical addresses listed in [Contract addresses](../contract_addresses.md). Your `forge` script just needs to broadcast against a Sepolia RPC — your contract picks up the FHEVM addresses through `ZamaEthereumConfig` (or whichever Zama config it inherits).

{% stepper %}

{% step %}

#### Configure environment variables

Copy the template's `.env.example` and fill in the values:

```bash
cp .env.example .env
```

```bash
# .env
DEPLOYER_PRIVATE_KEY=0x...
RPC_URL=https://sepolia.infura.io/v3/<key>
# ETHERSCAN_API_KEY=...   # optional, for verification
```

```bash
source .env
```

{% hint style="info" %}
If you only have a mnemonic / seed phrase, derive the private key with Foundry's `cast`:

```bash
cast wallet private-key "your twelve or twenty four words here"
```
{% endhint %}

{% endstep %}

{% step %}

#### Run the deploy script

```bash
forge script script/DeployFHECounter.s.sol \
    --rpc-url $RPC_URL \
    --private-key $DEPLOYER_PRIVATE_KEY \
    --broadcast --verify
```

{% endstep %}

{% endstepper %}

## Deploy to a local Anvil node

For local development, you must first deploy a **local FHEVM host stack** before deploying your own contract. This is provided by [forge-fhevm](https://github.com/zama-ai/forge-fhevm) via its `deploy-local.sh` script.

The local FHEVM host stack is a **cleartext** FHEVM where encrypted values are stored as plaintexts on-chain — nothing is actually encrypted, but every contract code path executes the same as on a real chain.

{% stepper %}

{% step %}

#### Clone forge-fhevm alongside your project

```bash
git clone https://github.com/zama-ai/forge-fhevm
```

{% endstep %}

{% step %}

#### Start an Anvil node

In a separate terminal:

```bash
anvil
```

{% endstep %}

{% step %}

#### Deploy the FHEVM host stack to the local node

From the `forge-fhevm` directory:

```bash
./deploy-local.sh
```

This materializes all FHEVM host contracts (`FHEVMExecutor`, `ACL`, `InputVerifier`, `KMSVerifier`) at their canonical addresses on the running Anvil node, using `setCode`/`setStorageAt`.

{% hint style="info" %}
`deploy-local.sh` is local-first and zero-config. It uses the addresses committed in `FHEVMHostAddresses.sol` and works out of the box with no `.env` file for the standard local setup that `ZamaConfig._getLocalConfig()` expects.
{% endhint %}

{% endstep %}

{% step %}

#### Deploy your contract

From your project directory:

```bash
forge script script/DeployFHECounter.s.sol \
    --rpc-url http://localhost:8545 \
    --broadcast
```

{% endstep %}

{% endstepper %}

### Multiple local nodes

`deploy-local.sh` can fan out to several Anvil instances from a single build:

```bash
# Deploy to two local nodes concurrently
./deploy-local.sh --anvil-port 8545 --anvil-port 8546

# Reuse already-built artifacts
./deploy-local.sh --skip-build --anvil-port 8545 --anvil-port 8546
```

## Where to go next

🟨 Go to [**Contract addresses**](../contract_addresses.md) for the canonical FHEVM addresses on each network.

🟨 Go to [**Write FHEVM tests in Foundry**](write_test.md) for the test side of the workflow.
