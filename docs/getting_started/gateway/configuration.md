# Configuration

The gateway acts as a bridge between the execution layer and the Threshold Key Management System (TKMS). Due to its central role, it needs to be properly configured. This document details all 
the environment variables and gives an example of docker compose to run the gateway.

## Dependencies

- **Zama Gateway**: Depends on **fhEVM** and **Gateway KV Store**, which is initialized with the **Zama KMS** Docker Compose command. Therefore, this is the _last_ Docker Compose command that should be run.

## Prerequisites

- **Docker 26+** installed on your system.
- **fhEVM** validator running and configured.
- **TKMS** running and configured.

## Configuring Docker Compose Environment Variables

### Example Docker Compose for Zama Gateway

```yaml
name: zama-gateway

services:

  gateway:
    image: ghcr.io/zama-ai/kms-blockchain-gateway-dev:latest
    command:
      - "gateway"
    environment:
      - GATEWAY__ETHEREUM__CHAIN_ID=9000
      - GATEWAY__ETHEREUM__LISTENER_TYPE=FHEVM_V1_1
      - GATEWAY__ETHEREUM__WSS_URL=ws://fhevm-validator:8546
      - GATEWAY__ETHEREUM__HTTP_URL=http://fhevm-validator:8545
      - GATEWAY__ETHEREUM__FHE_LIB_ADDRESS=000000000000000000000000000000000000005d
      - GATEWAY__ETHEREUM__ORACLE_PREDEPLOY_ADDRESS=c8c9303Cd7F337fab769686B593B87DC3403E0ce
      - GATEWAY__KMS__ADDRESS=http://kms-validator:9090
      - GATEWAY__KMS__KEY_ID=408d8cbaa51dece7f782fe04ba0b1c1d017b1088
      - GATEWAY__STORAGE__URL=http://gateway-store:8088
      - ASC_CONN__BLOCKCHAIN__ADDRESSES=http://kms-validator:9090
      - GATEWAY__ETHEREUM__RELAYER_KEY=7ec931411ad75a7c201469a385d6f18a325d4923f9f213bd882bbea87e160b67
```

**Zama Gateway** requires several specific configurations as shown in the provided `docker-compose-gateway.yml` file.
<!-- markdown-link-check-disable -->
| Variable | Description | Default Value |
| --- | --- | --- |
| GATEWAY__ETHEREUM__CHAIN_ID | Chain ID for fhEVM | 9000 |
| GATEWAY__ETHEREUM__LISTENER_TYPE | Listener type for Ethereum gateway | FHEVM_V1_1 |
| GATEWAY__ETHEREUM__WSS_URL | WebSocket URL for fhEVM Ethereum. You need to run fhEVM first and set this data | ws://localhost:9090 |
| GATEWAY__ETHEREUM__FHE_LIB_ADDRESS | FHE library address for Ethereum gateway. This should be obtained from fhEVM once it is running and configured | 000000000000000000000000000000000000005d |
| GATEWAY__ETHEREUM__ORACLE_PREDEPLOY_ADDRESS | Oracle predeploy contract address for fhEVM gateway | c8c9303Cd7F337fab769686B593B87DC3403E0cd |
| GATEWAY__KMS__ADDRESS | Address for KMS gateway | http://localhost:9090 |
| GATEWAY__KMS__KEY_ID | Key ID for KMS gateway. Refer to the [How to Obtain KMS Key ID](#kms-key-id) section | 04a1aa8ba5e95fb4dc42e06add00b0c2ce3ea424 |
| GATEWAY__STORAGE__URL | URL for storage gateway | http://localhost:8088 |
| ASC_CONN__BLOCKCHAIN__ADDRESSES | Blockchain addresses for ASC connection. Same as `GATEWAY__KMS__ADDRESS` | http://localhost:9090 |
| GATEWAY__ETHEREUM__RELAYER_KEY | Private key of the relayer | 7ec931411ad75a7c201469a385d6f18a325d4923f9f213bd882bbea87e160b67 |
<!-- markdown-link-check-enable-->
  
## Steps for Running

1. Run the **Zama Gateway** Docker Compose:

```bash
docker compose -f docker-compose-gateway.yml up -d
```

> :warning: **Requirement**: At start, the Gateway will try to connect to the websocker URL `GATEWAY__ETHEREUM__WSS_URL`. Ensure it is running and the port is opened.

## KMS Key ID

To obtain the `Key ID` for the `GATEWAY__KMS__KEY_ID` environment variable, run the following command:

```bash
> docker run -ti ghcr.io/zama-ai/kms-service-dev:latest ls keys/PUB/PublicKey
04a1aa8ba5e95fb4dc42e06add00b0c2ce3ea424  8e917efb2fe00ebbe8f73b2ba2ed80e7e28970de
```


