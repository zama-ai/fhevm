# Local dev node

## Setup

We provide a docker image to spin up a fhEVM node for local development.

```bash
docker run -i -p 8545:8545 -p 8546:8546 --rm --name fhevm ghcr.io/zama-ai/ethermint-dev-node:v0.2.4
```

<!-- markdown-link-check-disable -->

| Type      | URL                   |
| --------- | --------------------- |
| JSON-RPC  | http://127.0.0.1:8545 |
| Websocket | http://127.0.0.1:8546 |

<!-- markdown-link-check-enable -->

## Faucet

If you need to get coins for a specific wallet, you can use the faucet as follow:

```bash
docker exec -i fhevm faucet 0xa5e1defb98EFe38EBb2D958CEe052410247F4c80
```
