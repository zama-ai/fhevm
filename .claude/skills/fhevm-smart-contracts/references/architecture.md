# Architecture - Smart Contracts

Component structure and relationships for fhevm Solidity contracts.

---

## Component Overview

```text
+---------------------+     +---------------------+     +---------------------+
|  library-solidity   |     |   host-contracts    |     | gateway-contracts   |
|  (FHE.sol Library)  |     |  (Core Protocol)    |     | (Decryption Layer)  |
+----------+----------+     +----------+----------+     +----------+----------+
           |                           |                           |
           |  Uses                     |  Verifies                 |  Routes
           v                           v                           v
+----------+----------+     +----------+----------+     +----------+----------+
|  Developer          |     |      ACL.sol        |     |  GatewayContract    |
|  Contracts          |     |  FHEVMExecutor.sol  |     |  InputVerifier.sol  |
|  (Your Code)        |     |  KMSVerifier.sol    |     |                     |
+---------------------+     +---------------------+     +---------------------+
                                      |
                                      v
                            +---------------------+
                            | protocol-contracts  |
                            | (Tokens, Staking)   |
                            +---------------------+
```

---

## Core Components

### library-solidity/

The developer-facing FHE library.

| Contract           | Purpose                              |
| ------------------ | ------------------------------------ |
| `FHE.sol`          | Main library - all FHE operations    |
| `TFHE.sol`         | Type definitions (euint64, ebool)    |
| `ACLManager.sol`   | Permission management helpers        |

**Key Pattern**: Stateless library, all state in calling contracts.

### host-contracts/

On-chain protocol infrastructure deployed once per network.

| Contract            | Purpose                              |
| ------------------- | ------------------------------------ |
| `ACL.sol`           | Global permission registry           |
| `FHEVMExecutor.sol` | FHE operation coordinator            |
| `KMSVerifier.sol`   | Decryption signature verification    |

**Deployment**: Single instance per network (mainnet, Sepolia).

### gateway-contracts/

Decryption gateway infrastructure.

| Contract              | Purpose                              |
| --------------------- | ------------------------------------ |
| `GatewayContract.sol` | Decryption request routing           |
| `InputVerifier.sol`   | Encrypted input validation           |

**Pattern**: Receives decryption requests, coordinates with KMS.

### protocol-contracts/

Token standards and governance.

| Contract                 | Purpose                              |
| ------------------------ | ------------------------------------ |
| `ConfidentialERC20.sol`  | Encrypted ERC20 implementation       |
| `ERC7984Wrapper.sol`     | Standard wrapper for fhe tokens      |
| `Staking.sol`            | Protocol staking mechanics           |

**Pattern**: Reference implementations for developers.

---

## Contract Relationships

### Inheritance Hierarchy

```text
ZamaEthereumConfig
    └── YourContract
            ├── Uses FHE.sol (library)
            ├── Calls ACL.sol (permissions)
            └── May inherit OpenZeppelin contracts
```

### Typical Developer Contract

```solidity
import "@fhevm/solidity/lib/FHE.sol";
import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";

contract MyContract is Ownable2Step, ZamaEthereumConfig {
    mapping(address => euint64) private balances;

    // ZamaEthereumConfig sets up coprocessor automatically
}
```

---

## Upgrade Architecture

### UUPS Proxy Pattern

All upgradeable contracts use UUPS:

```text
+------------------+     +------------------+
|   Proxy          |---->| Implementation   |
| (ERC1967Proxy)   |     | (Logic Contract) |
+------------------+     +------------------+
        |
        v
+------------------+
|  Storage Layout  |
| (In Proxy)       |
+------------------+
```

### Upgrade Authorization

```solidity
contract MyUpgradeable is UUPSUpgradeable, Ownable2Step {
    function _authorizeUpgrade(address newImplementation)
        internal
        override
        onlyOwner
    {}
}
```

---

## Data Flows

### Encrypted Transfer Flow

```text
1. User encrypts amount client-side (fhevmjs)
2. User calls contract.transfer(recipient, encryptedAmount, proof)
3. Contract validates proof with FHE.fromExternal()
4. Contract computes: canTransfer = FHE.le(amount, balance)
5. Contract updates: newBalance = FHE.select(canTransfer, ...)
6. Contract sets ACL: FHE.allow(newBalance, recipient)
7. Event emitted (without decrypted values)
```

### Decryption Flow

```text
1. Contract marks value: FHE.makePubliclyDecryptable(value)
2. Off-chain: Gateway detects decryption request
3. KMS nodes compute threshold signatures
4. Result submitted to GatewayContract
5. Anyone can verify with KMSVerifier
6. Callback triggered with cleartext
```

---

## Network Addresses

### Ethereum Mainnet (chainId=1)

| Contract      | Address                                      |
| ------------- | -------------------------------------------- |
| ACL           | `0xcA2E8f1F656CD25C01F05d0b243Ab1ecd4a8ffb6` |
| Coprocessor   | `0xD82385dADa1ae3E969447f20A3164F6213100e75` |
| KMSVerifier   | `0x77627828a55156b04Ac0DC0eb30467f1a552BB03` |

### Sepolia Testnet (chainId=11155111)

| Contract      | Address                                      |
| ------------- | -------------------------------------------- |
| ACL           | `0xf0Ffdc93b7E186bC2f8CB3dAA75D86d1930A433D` |
| Coprocessor   | `0x92C920834Ec8941d2C77D188936E1f7A6f49c127` |
| KMSVerifier   | `0xbE0E383937d564D7FF0BC3b46c51f0bF8d5C311A` |

---

## Storage Patterns

### Encrypted Mapping

```solidity
// Standard pattern for encrypted balances
mapping(address => euint64) private _balances;

// With allowances
mapping(address => mapping(address => euint64)) private _allowances;
```

### Storage Gaps for Upgrades

```solidity
contract MyContractV1 {
    euint64 private _totalSupply;
    mapping(address => euint64) private _balances;

    // Reserve 48 slots for future variables
    uint256[48] private __gap;
}
```
