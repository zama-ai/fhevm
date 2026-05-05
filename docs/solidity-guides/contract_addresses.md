## Ethereum mainnet

| Contract/Service           | Address                                    |
| -------------------------- | ------------------------------------------ |
| ACL_CONTRACT               | 0xcA2E8f1F656CD25C01F05d0b243Ab1ecd4a8ffb6 |
| FHEVM_EXECUTOR_CONTRACT    | 0xD82385dADa1ae3E969447f20A3164F6213100e75 |
| KMS_VERIFIER_CONTRACT      | 0x77627828a55156b04Ac0DC0eb30467f1a552BB03 |

## Sepolia testnet

| Contract/Service           | Address/Value                              |
| -------------------------- | ------------------------------------------ |
| ACL_CONTRACT               | 0xf0Ffdc93b7E186bC2f8CB3dAA75D86d1930A433D |
| FHEVM_EXECUTOR_CONTRACT    | 0x92C920834Ec8941d2C77D188936E1f7A6f49c127 |
| KMS_VERIFIER_CONTRACT      | 0xbE0E383937d564D7FF0BC3b46c51f0bF8d5C311A |
| HCU_LIMIT_CONTRACT         | 0xa10998783c8CF88D886Bc30307e631D6686F0A22 |
| INPUT_VERIFIER_CONTRACT    | 0xBBC1fFCdc7C316aAAd72E807D9b0272BE8F84DA0 |
| DECRYPTION_ADDRESS         | 0x5D8BD78e2ea6bbE41f26dFe9fdaEAa349e077478 |
| INPUT_VERIFICATION_ADDRESS | 0x483b9dE06E4E4C7D35CCf5837A1668487406D955 |
| RELAYER_URL                | `https://relayer.testnet.zama.org`         |
| GATEWAY_CHAIN_ID           | 10901                                      |

## Local / Hardhat (chain ID 31337)

| Contract/Service           | Address                                    |
| -------------------------- | ------------------------------------------ |
| ACL_CONTRACT               | 0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D |
| FHEVM_EXECUTOR_CONTRACT    | 0xe3a9105a3a932253A70F126eb1E3b589C643dD24 |
| KMS_VERIFIER_CONTRACT      | 0x901F8942346f7AB3a01F6D7613119Bca447Bb030 |

{% hint style="info" %}
You do not need to configure these addresses manually. Inheriting from `ZamaEthereumConfig` automatically resolves the correct addresses based on the current `block.chainid`.
{% endhint %}
