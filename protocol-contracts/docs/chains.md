# Chains

This page lists the chains involved in the Zama protocol, their block explorers, RPC endpoints, chain IDs, and LayerZero configurations.

## Mainnet

### Block explorers

* Ethereum: [https://eth.blockscout.com/](https://eth.blockscout.com/)
* Polygon: [https://polygon.blockscout.com/](https://polygon.blockscout.com/)
* Gateway: [https://explorer.mainnet.zama.org/](https://explorer.mainnet.zama.org/)
* BSC: [https://bscscan.com/](https://bscscan.com/)
* HyperEVM: [https://hyperevmscan.io/](https://hyperevmscan.io/)
* Solana: [https://solscan.io/](https://solscan.io/)

### RPC endpoints

* Gateway: [https://rpc.mainnet.zama.org](https://rpc.mainnet.zama.org)

### EVM chains - Chain IDs

Not to be confused with Endpoint IDs (see section below).

| Name                  |  Chain ID  |
| --------------------- | ---------- |
| `Ethereum`            |     1      |
| `Polygon`             |    137     |
| `Gateway`             |  261131    |
| `BSC`                 |    56      |
| `HyperEVM`            |    999     |

__Note:__ These are only for EVM chains, Solana does not have a chain ID (but has a LayerZero endpoint ID).

### LayerZero

#### Endpoint IDs

Those are LayerZero specific and should not be confused with Chain IDs (see section above).

| Name                  | Endpoint ID (eid) |
| --------------------- | ----------------- |
| `Ethereum`            |      30101        |
| `Polygon`             |      30109        |
| `Gateway`             |      30397        |
| `BSC`                 |      30102        |
| `SOL`                 |      30168        |
| `HyperEVM`            |      30367        |

## Testnet

### Block explorers

* Ethereum Sepolia: [https://eth-sepolia.blockscout.com](https://eth-sepolia.blockscout.com)
* Gateway Testnet: [https://explorer.testnet.zama.org/](https://explorer.testnet.zama.org/)
* Polygon Amoy: [https://amoy.polygonscan.com/](https://amoy.polygonscan.com/)

### RPC endpoints

* Gateway Testnet: [https://rpc.testnet.zama.org](https://rpc.testnet.zama.org)

### EVM chains - Chain IDs

| Name                  |  Chain ID  |
| --------------------- | ---------- |
| `Ethereum Sepolia`    | 11155111   |
| `Gateway Testnet`     |  10901     |
| `BSC Testnet`         |    97      |
| `Polygon Amoy`        |  80002     |

### LayerZero

#### Endpoint IDs

| Name                  | Endpoint ID (eid) |
| --------------------- | ----------------- |
| `Ethereum Sepolia`    |      40161        |
| `Gateway Testnet`     |      40424        |
| `BSC Testnet`         |      40102        |
| `Polygon Amoy`        |      40267        |
