# Pausing

Circuit breakers are deployed on all chains involved with the Zama protocol. Any operator can trigger any of these on their own to pause parts of the protocol, but a governance vote is needed to unpause again.

## Contract information

| Resource | Link |
| --- | --- |
| Deployed addresses | [Addresses directory](addresses/README.md) |
| Source code | [PauserSetWrapper.sol](https://github.com/zama-ai/fhevm/blob/main/protocol-contracts/contracts/pauserSetWrapper/contracts/PauserSetWrapper.sol) |

## Structure

```mermaid
flowchart
    subgraph Ethereum
        Protocol-DAO
        PauserSet-Host
        ACL-Host
        PauserSet-Wrapper
        ZAMA-ERC20
    end

    subgraph Gateway
        Gateway-Multisig
        PauserSet-Gateway
        GatewayConfig
    end

    Protocol-DAO -- owner (via ACL contract) --> PauserSet-Host
    Pauser-1..n -. member .-> PauserSet-Host

    PauserSet-Host -. defines pausers .-> ACL-Host
    PauserSet-Host -. defines pausers .-> PauserSet-Wrapper
    PauserSet-Wrapper -- pauser role --> ZAMA-ERC20

    Gateway-Multisig -- owner (via GatewayConfig contract) --> PauserSet-Gateway
    Pauser-1..n -. member .-> PauserSet-Gateway
    PauserSet-Gateway -. defines pausers .-> GatewayConfig
```

## Wallets

Each operator has their own wallet that can be used to trigger the circuit breakers. This address presents a trade-off between being readily available, for instance to anyone who’s on-call, while also being able to potentially cause significant damage if misused.

Operators are free to choose their implementation, but we suggest to use a hot wallet kept as a secret in their deployment system.

## Targets

The following components can be paused.

| Component   | Functionality |
| ----------- | ------------- |
| $ZAMA token | Minting can be paused |
| Ethereum    | ACL updates can be paused |
| Gateway     | Decryption requests can be paused |
| Gateway     | Input verification requests can be paused |

