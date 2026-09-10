# Contract Addresses

{% hint style="info" %}
You do not need to configure these addresses manually. Inheriting from `ZamaEthereumConfig`, `ZamaPolygonConfig` or `ZamaMultiChainConfig` automatically resolves the correct addresses based on the current `block.chainid`. Ethereum and Polygon share the same gateway contracts; Sepolia and Polygon Amoy share the testnet gateway.
{% endhint %}
 
## Mainnet

### Ethereum

| Contract                   | Address                                    |
| -------------------------- | ------------------------------------------ |
| ACL                        | 0xcA2E8f1F656CD25C01F05d0b243Ab1ecd4a8ffb6 |
| FHEVM_EXECUTOR             | 0xD82385dADa1ae3E969447f20A3164F6213100e75 |
| INPUT_VERIFIER             | 0xCe0FC2e05CFff1B719EFF7169f7D80Af770c8EA2 |
| KMS_VERIFIER               | 0x77627828a55156b04Ac0DC0eb30467f1a552BB03 |
| HCU_LIMIT                  | 0x3b4da65e45Fda2CAa0285A735ab4361a44F171E2 |
| PROTOCOL_CONFIG            | 0xD8236B57394f90726b26aB25D38CeAC776E1a7C4 |

### Polygon

| Contract                   | Address                                    |
| -------------------------- | ------------------------------------------ |
| ACL                        | 0x6737F17e31cf26a1b62fb0362acC5a16CB156F49 |
| FHEVM_EXECUTOR             | 0xAB0075E77fe06083f52bdf10e2ccDB3712483057 |
| INPUT_VERIFIER             | 0xf40BD204B035522EaAc8E5afAdc55113Acac96ca |
| KMS_VERIFIER               | 0x14e609595474874Dd6b6128376E336EfADfdBE37 |
| HCU_LIMIT                  | 0x226cf23556E59e3284c3c7705868478746D338af |
| PROTOCOL_CONFIG            | 0x17f62Ab3A1Ea519703cD597410147A30Fa1a7f1e |

### Gateway (shared by Ethereum and Polygon)

| Contract                   | Address                                    |
| -------------------------- | ------------------------------------------ |
| GATEWAY_CONFIG             | 0xDE537Be194777A56f8B19d14079E6a78249390ab |
| DECRYPTION                 | 0x0f6024a97684f7d90ddb0fAAD79cB15F2C888D24 |
| INPUT_VERIFICATION         | 0xcB1bB072f38bdAF0F328CdEf1Fc6eDa1DF029287 |
| CIPHERTEXT_COMMITS         | 0xd82cF70FC102028cd01acB87D0E107780ae4F41F |
| KMS_GENERATION             | 0x290947F9fed2d91fdB22f35E162aDfA744b7aEe3 |
| MULTICHAIN_ACL             | 0x055d9FD50a612A9027716ec8db663E7D68562468 |
| PROTOCOL_PAYMENT           | 0x7E179E45E5fe0a21015Be25185363B4F2F2F7e89 |

## Testnet

### Sepolia

| Contract                   | Address                                    |
| -------------------------- | ------------------------------------------ |
| ACL_HOST                   | 0xf0Ffdc93b7E186bC2f8CB3dAA75D86d1930A433D |
| FHEVM_EXECUTOR             | 0x92C920834Ec8941d2C77D188936E1f7A6f49c127 |
| INPUT_VERIFIER             | 0xBBC1fFCdc7C316aAAd72E807D9b0272BE8F84DA0 |
| KMS_VERIFIER               | 0xbE0E383937d564D7FF0BC3b46c51f0bF8d5C311A |
| HCU_LIMIT                  | 0xa10998783c8CF88D886Bc30307e631D6686F0A22 |
| PROTOCOL_CONFIG            | 0x51f9AFBc89Ea792e1a21a12AB802ab58D4dbee83 |

### Polygon Amoy

| Contract                   | Address                                    |
| -------------------------- | ------------------------------------------ |
| ACL_HOST                   | 0xD99Cb9Fc3c42c87f2A4A12e8Fd60318d6bDdf985 |
| FHEVM_EXECUTOR             | 0x89420269f61e4db00545cd99da0aEcA7fF0912f9 |
| INPUT_VERIFIER             | 0x6e5A7D8b0c645467Cba7e62D6624917085118631 |
| KMS_VERIFIER               | 0xCD1D89E311bce4C8DEa9a0857a0c9A4E153D4041 |
| HCU_LIMIT                  | 0x462f1920A9a7b5Aa74A36c3f49E38C34392B0546 |
| PROTOCOL_CONFIG            | 0x4CcF009Aba90D04f52b31fc7aDdE240578aFe10F |

### Gateway (shared by Sepolia and Polygon Amoy)

| Contract                   | Address                                    |
| -------------------------- | ------------------------------------------ |
| GATEWAY_CONFIG             | 0x94153006067B89399e059284f5a7Fe016940E332 |
| DECRYPTION                 | 0x5D8BD78e2ea6bbE41f26dFe9fdaEAa349e077478 |
| INPUT_VERIFICATION         | 0x483b9dE06E4E4C7D35CCf5837A1668487406D955 |
| CIPHERTEXT_COMMITS         | 0xE327808C4aD514D6bd405e1f12cC86Fcd08e5228 |
| KMS_GENERATION             | 0x5779Ac320BbDB267Cc4d1b77195a203F926bBC60 |
| MULTICHAIN_ACL             | 0xe877cA18d8Ea5490e9256e4B28414320726a8c3c |
| PROTOCOL_PAYMENT           | 0xAA1d9D4927A62f842F0DE5AD6b8dFDB074Fa62f2 |