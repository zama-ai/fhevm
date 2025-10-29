<p align="center">
  <a href="https://layerzero.network">
    <img alt="LayerZero" style="width: 400px" src="https://docs.layerzero.network/img/LayerZero_Logo_Black.svg"/>
  </a>
</p>

<p align="center">
  <a href="https://docs.layerzero.network/" style="color: #a77dff">LayerZero Docs</a>
</p>

<h1 align="center">Omnichain Application (OApp) Example</h1>

<p align="center">Template project for creating custom omnichain applications (<a href="https://docs.layerzero.network/v2/concepts/applications/oapp-standard">OApp</a>) powered by the LayerZero protocol. This example demonstrates how to build applications that can send and receive arbitrary messages across different blockchains.</p>

## Table of Contents

- [Prerequisite Knowledge](#prerequisite-knowledge)
- [Requirements](#requirements)
- [Scaffold this example](#scaffold-this-example)
- [Helper Tasks](#helper-tasks)
- [Setup](#setup)
- [Build](#build)
  - [Compiling your contracts](#compiling-your-contracts)
- [Deploy](#deploy)
- [Enable Messaging](#enable-messaging)
- [Sending Messages](#sending-messages)
- [Next Steps](#next-steps)
- [Production Deployment Checklist](#production-deployment-checklist)
- [Appendix](#appendix)
  - [Running Tests](#running-tests)
  - [Adding other chains](#adding-other-chains)
  - [Using Multisigs](#using-multisigs)
  - [LayerZero Hardhat Helper Tasks](#layerzero-hardhat-helper-tasks)
  - [Contract Verification](#contract-verification)
  - [Troubleshooting](#troubleshooting)

## Prerequisite Knowledge

- [What is an OApp (Omnichain Application)?](https://docs.layerzero.network/v2/concepts/applications/oapp-standard)
- [How does LayerZero work?](https://docs.layerzero.network/v2/concepts/protocol/core-concepts)

## Requirements

- `Node.js` - `>=18.16.0`
- `pnpm` (recommended) - or another package manager of your choice (npm, yarn)
- `forge` (optional) - `>=0.2.0` for testing, and if not using Hardhat for compilation

## Scaffold this example

Create your local copy of this example:

```bash
pnpm dlx create-lz-oapp@latest --example oapp
```

Specify the directory, select `OApp` and proceed with the installation.

Note that `create-lz-oapp` will also automatically run the dependencies install step for you.

## Helper Tasks

Throughout this walkthrough, helper tasks will be used. For the full list of available helper tasks, refer to the [LayerZero Hardhat Helper Tasks section](#layerzero-hardhat-helper-tasks). All commands can be run at the project root.

## Setup

- Copy `.env.example` into a new `.env`
- Set up your deployer address/account via the `.env`

  - You can specify either `MNEMONIC` or `PRIVATE_KEY`:

    ```
    MNEMONIC="test test test test test test test test test test test junk"
    or...
    PRIVATE_KEY="0xabc...def"
    ```

- Fund this deployer address/account with the native tokens of the chains you want to deploy to. This example by default will deploy to the following chains' testnets: **Base Sepolia** and **Arbitrum Sepolia**.

## Build

### Compiling your contracts

This project supports both `hardhat` and `forge` compilation. By default, the `compile` command will execute both:

```bash
pnpm compile
```

If you prefer one over the other, you can use the tooling-specific commands:

```bash
pnpm compile:forge
pnpm compile:hardhat
```

## Deploy

To deploy the OApp contracts to your desired blockchains, run the following command:

```bash
pnpm hardhat lz:deploy --tags MyOApp
```

Select all the chains you want to deploy the OApp to.

## Enable Messaging

After deploying the OApp on the respective chains, you enable messaging by running the [wiring](https://docs.layerzero.network/v2/concepts/glossary#wire--wiring) task.

> :information_source: This example uses the [Simple Config Generator](https://docs.layerzero.network/v2/tools/simple-config), which is recommended over manual configuration.

Run the wiring task:

```bash
pnpm hardhat lz:oapp:wire --oapp-config layerzero.config.ts
```

Submit all the transactions to complete wiring. After all transactions confirm, your OApps are wired and can send messages to each other.

## Sending Messages

With your OApps wired, you can now send messages cross-chain.

Send a message from **Base Sepolia** to **Arbitrum Sepolia**:

```bash
pnpm hardhat lz:oapp:send --dst-eid 40231 --string 'Hello from Base!' --network base-sepolia
```

Send a message from **Arbitrum Sepolia** to **Base Sepolia**:

```bash
pnpm hardhat lz:oapp:send --dst-eid 40245 --string 'Hello from Arbitrum!' --network arbitrum-sepolia
```

> :information_source: `40245` and `40231` are the Endpoint IDs of Base Sepolia and Arbitrum Sepolia respectively. The source network is determined by the `--network` flag, not a separate `--src-eid` parameter. View the list of chains and their Endpoint IDs on the [Deployed Endpoints](https://docs.layerzero.network/v2/deployments/deployed-contracts) page.

Upon a successful send, the script will provide you with the link to the message on LayerZero Scan.

Once the message is delivered, you will be able to click on the destination transaction hash to verify that the message was received.

Congratulations, you have now sent a message cross-chain!

> If you run into any issues, refer to [Troubleshooting](#troubleshooting).

## Next Steps

Now that you've gone through a simplified walkthrough, here are what you can do next.

- If you are planning to deploy to production, go through the [Production Deployment Checklist](#production-deployment-checklist).
- Read on [DVNs / Security Stack](https://docs.layerzero.network/v2/concepts/modular-security/security-stack-dvns)
- Read on [Message Execution Options](https://docs.layerzero.network/v2/concepts/technical-reference/options-reference)

## Production Deployment Checklist

Before deploying, ensure the following:

- (recommended) you have profiled the gas usage of `lzReceive` on your destination chains
- (recommended) you have configured appropriate DVNs for your security requirements
- (recommended) you have tested your application thoroughly on testnets

<p align="center">
  Join our <a href="https://layerzero.network/community" style="color: #a77dff">community</a>! | Follow us on <a href="https://x.com/LayerZero_Labs" style="color: #a77dff">X (formerly Twitter)</a>
</p>

# Appendix

## Running Tests

Similar to the contract compilation, we support both `hardhat` and `forge` tests. By default, the `test` command will execute both:

```bash
pnpm test
```

If you prefer one over the other, you can use the tooling-specific commands:

```bash
pnpm test:forge
pnpm test:hardhat
```

## Adding other chains

If you're adding another EVM chain, first, add it to the `hardhat.config.ts`. Adding non-EVM chains do not require modifying the `hardhat.config.ts`.

Then, modify `layerzero.config.ts` with the following changes:

- declare a new contract object (specifying the `eid` and `contractName`)
- decide whether to use an existing EVM enforced options variable or declare a new one
- create a new entry in the `connections` array
- add the new contract into the `contracts` array of the `export default` function

After applying the desired changes, make sure you re-run the wiring task:

```bash
pnpm hardhat lz:oapp:wire --oapp-config layerzero.config.ts
```

## Using Multisigs

The wiring task supports the usage of Safe Multisigs.

To use a Safe multisig as the signer for these transactions, add the following to each network in your `hardhat.config.ts` and add the `--safe` flag to `lz:oapp:wire --safe`:

```typescript
// hardhat.config.ts

networks: {
  // Include configurations for other networks as needed
  fuji: {
    /* ... */
    // Network-specific settings
    safeConfig: {
      safeUrl: 'http://something', // URL of the Safe API, not the Safe itself
      safeAddress: 'address'
    }
  }
}
```

## LayerZero Hardhat Helper Tasks

LayerZero Devtools provides several helper hardhat tasks to easily deploy, verify, configure, connect, and interact with OApps cross-chain.

<details>
<summary> <a href="https://docs.layerzero.network/v2/developers/evm/create-lz-oapp/deploying"><code>pnpm hardhat lz:deploy</code></a> </summary>

 <br>

Deploys your contract to any of the available networks in your [`hardhat.config.ts`](./hardhat.config.ts) when given a deploy tag (by default contract name) and returns a list of available networks to select for the deployment. For specifics around all deployment options, please refer to the [Deploying Contracts](https://docs.layerzero.network/v2/developers/evm/create-lz-oapp/deploying) section of the documentation. LayerZero's `lz:deploy` utilizes `hardhat-deploy`.

More information about available CLI arguments can be found using the `--help` flag:

```bash
pnpm hardhat lz:deploy --help
```

</details>

<details>
<summary> <a href="https://docs.layerzero.network/v2/developers/evm/create-lz-oapp/start"><code>pnpm hardhat lz:oapp:config:init --oapp-config YOUR_OAPP_CONFIG --contract-name CONTRACT_NAME</code></a> </summary>

 <br>

Initializes a `layerzero.config.ts` file for all available pathways between your hardhat networks with the current LayerZero default placeholder settings. This task can be incredibly useful for correctly formatting your config file.

You can run this task by providing the `contract-name` you want to set for the config and `file-name` you want to generate:

```bash
pnpm hardhat lz:oapp:config:init --contract-name CONTRACT_NAME --oapp-config FILE_NAME
```

</details>

<details>
<summary> <a href="https://docs.layerzero.network/v2/developers/evm/create-lz-oapp/wiring"><code>pnpm hardhat lz:oapp:config:wire --oapp-config YOUR_OAPP_CONFIG</code></a> </summary>

 <br>

Calls the configuration functions between your deployed OApp contracts on every chain based on the provided `layerzero.config.ts`.

Running `lz:oapp:wire` will make the following function calls per pathway connection for a fully defined config file using your specified settings and your environment variables (Private Keys and RPCs):

- <a href="https://github.com/LayerZero-Labs/LayerZero-v2/blob/main/packages/layerzero-v2/evm/oapp/contracts/oapp/OAppCore.sol#L33-L46"><code>function setPeer(uint32 \_eid, bytes32 \_peer) public virtual onlyOwner {}</code></a>

- <a href="https://github.com/LayerZero-Labs/LayerZero-v2/blob/main/packages/layerzero-v2/evm/protocol/contracts/MessageLibManager.sol#L304-L311"><code>function setConfig(address \_oapp, address \_lib, SetConfigParam[] calldata \_params) external onlyRegistered(\_lib) {}</code></a>

- <a href="https://github.com/LayerZero-Labs/LayerZero-v2/blob/main/packages/layerzero-v2/evm/oapp/contracts/oapp/libs/OAppOptionsType3.sol#L18-L36"><code>function setEnforcedOptions(EnforcedOptionParam[] calldata \_enforcedOptions) public virtual onlyOwner {}</code></a>

- <a href="https://github.com/LayerZero-Labs/LayerZero-v2/blob/main/packages/layerzero-v2/evm/protocol/contracts/MessageLibManager.sol#L223-L238"><code>function setSendLibrary(address \_oapp, uint32 \_eid, address \_newLib) external onlyRegisteredOrDefault(\_newLib) onlySupportedEid(\_newLib, \_eid) {}</code></a>

- <a href="https://github.com/LayerZero-Labs/LayerZero-v2/blob/main/packages/layerzero-v2/evm/protocol/contracts/MessageLibManager.sol#L223-L273"><code>function setReceiveLibrary(address \_oapp, uint32 \_eid, address \_newLib, uint256 \_gracePeriod) external onlyRegisteredOrDefault(\_newLib) isReceiveLib(\_newLib) onlySupportedEid(\_newLib, \_eid) {}</code></a>

To use this task, run:

```bash
pnpm hardhat lz:oapp:wire --oapp-config YOUR_LAYERZERO_CONFIG_FILE
```

Whenever you make changes to the configuration, run `lz:oapp:wire` again. The task will check your current configuration, and only apply NEW changes.

</details>

<details>
<summary> <a href="https://docs.layerzero.network/v2/developers/evm/create-lz-oapp/wiring#checking-pathway-config"><code>pnpm hardhat lz:oapp:config:get --oapp-config YOUR_OAPP_CONFIG</code></a> </summary>

 <br>

Returns your current OApp's configuration for each chain and pathway in 3 columns:

- **Custom Configuration**: the changes that your `layerzero.config.ts` currently has set

- **Default Configuration**: the default placeholder configuration that LayerZero provides

- **Active Configuration**: the active configuration that applies to the message pathway (Defaults + Custom Values)

If you do NOT explicitly set each configuration parameter, your OApp will fallback to the placeholder parameters in the default config.

</details>

## Contract Verification

You can verify EVM chain contracts using the LayerZero helper package:

```bash
pnpm dlx @layerzerolabs/verify-contract -n <NETWORK_NAME> -u <API_URL> -k <API_KEY> --contracts <CONTRACT_NAME>
```

## Troubleshooting

Refer to [Debugging Messages](https://docs.layerzero.network/v2/developers/evm/troubleshooting/debugging-messages) or [Error Codes & Handling](https://docs.layerzero.network/v2/developers/evm/troubleshooting/error-messages).
