# Local dev node
This document explains how to configure the local dev node for deploying smart contracts on fhEVM-native chains.

## Setup

We provide a docker image to spin up a fhEVM node for local development.

```bash
docker run -i -p 8545:8545 -p 8546:8546 --rm --name fhevm ghcr.io/zama-ai/ethermint-dev-node:v0.4.2
```

<!-- markdown-link-check-disable -->

| Type      | URL                   |
| --------- | --------------------- |
| JSON-RPC  | http://127.0.0.1:8545 |
| Websocket | http://127.0.0.1:8546 |

<!-- markdown-link-check-enable -->

However, we recommend using the `pnpm fhevm:start` or `npm run fhevm:start` commands available within the [Hardhat template]((../../fundamentals/write_contract/hardhat.md)). These commands launch a bash script that automatically deploys the gateway contract and starts the gateway relayer service, which are necessary for asynchronous decryption requests.

{% hint style="warning" %}
 `GatewayCaller.sol` must be imported at least once in one of your smart contracts if use the `fhevm:start` command. If not, the bash script will emit an error and decryptions will fail. 
 
 Hardhat needs to compile the gateway predeploy contract before your initial deployment. To do this, add the following import at the top of any of your project's smart contracts:
 `import "fhevm/gateway/GatewayCaller.sol";`
{% endhint %}

## Faucet

To get coins for a specific wallet, use the following faucet:

```bash
docker exec -i fhevm faucet 0xa5e1defb98EFe38EBb2D958CEe052410247F4c80
```
