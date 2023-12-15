# Setup

We provide a docker image to spin up a fhEVM node for local development.

```bash
docker run -i -p 8545:8545 -p 8546:8546 --rm --name fhevm ghcr.io/zama-ai/evmos-dev-node:v0.1.10
```

<!-- markdown-link-check-disable -->

| Type      | URL                   |
| --------- | --------------------- |
| JSON-RPC  | http://127.0.0.1:8545 |
| Websocket | http://127.0.0.1:8546 |

<!-- markdown-link-check-enable -->
