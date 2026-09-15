# Quick Reference

Fast lookup guide for developers working with the FHEVM codebase.

## Directory Map

```
fhevm/
├── gateway-contracts/     # Bridge to off-chain (Solidity)
├── host-contracts/        # On-chain FHE execution (Solidity)
├── library-solidity/      # Developer FHE library (Solidity)
├── protocol-contracts/    # Token, staking, governance (Solidity)
├── coprocessor/           # FHE compute engine (Rust)
├── kms-connector/         # KMS bridge (Rust)
├── charts/                # Kubernetes Helm charts
├── test-suite/            # E2E integration tests
├── docs/                  # Documentation source
└── sdk/                   # Rust SDK
```

## Entry Points for Developers

**Writing contracts:**
- Start: `/library-solidity/lib/FHE.sol`
- Examples: `/library-solidity/examples/`
- Types: `/library-solidity/lib/FheType.sol`

**Understanding execution:**
- Symbolic execution: `/host-contracts/contracts/FHEVMExecutor.sol`
- ACL: `/host-contracts/contracts/ACL.sol`
- Events: Look for `event` declarations in contracts

**Understanding coordination:**
- Gateway config: `/gateway-contracts/contracts/GatewayConfig.sol`
- Decryption: `/gateway-contracts/contracts/Decryption.sol`
- Ciphertext commits: `/gateway-contracts/contracts/CiphertextCommits.sol`

**Running tests:**
- Hardhat: `npm run test` in contract directories
- Foundry: `forge test` in contract directories
- E2E: `docker-compose up` in `/test-suite/`

**Deploying:**
- Helm: `/charts/` configurations
- Docker: `docker-compose.yml` files

## Common Commands

### Smart Contracts (Hardhat)

```bash
# In contract directories (host-contracts/, gateway-contracts/, etc.)
npm install              # Install dependencies
npm run compile          # Compile contracts
npm run test             # Run tests
npm run coverage         # Test coverage
npm run lint             # Lint Solidity
npm run format           # Format code
```

### Smart Contracts (Foundry)

```bash
forge build              # Compile
forge test               # Run tests
forge test -vvv          # Verbose test output
forge coverage           # Coverage report
forge fmt                # Format code
```

### Rust Services

```bash
# In coprocessor/ or kms-connector/
cargo build --release    # Build optimized binary
cargo test               # Run tests
cargo clippy             # Linting
cargo fmt                # Format code
cargo check              # Fast compile check
```

### Docker Compose (Full Stack)

```bash
# In test-suite/
docker-compose up        # Start all services
docker-compose down      # Stop services
docker-compose logs -f   # Follow logs
docker-compose ps        # Show status
```

## Key Contract Addresses (Example Deployment)

> **Note:** These are example addresses. Actual addresses vary by deployment.

**Host Chain Contracts:**
```
FHEVMExecutor:  0x1234...
ACL:            0x5678...
HCULimit:       0x9abc...
KMSVerifier:    0xdef0...
InputVerifier:  0x1357...
```

**Gateway Chain Contracts:**
```
GatewayConfig:      0x2468...
Decryption:         0x369c...
MultichainACL:      0x48ae...
CiphertextCommits:  0x59bf...
```

## FHE Operation Gas Costs

Symbolic execution (on-chain):

| Operation | Gas Cost (approx) |
|-----------|------------------|
| `fheAdd` | ~50k gas |
| `fheMul` | ~50k gas |
| `fheEq` | ~50k gas |
| `fheSelect` | ~60k gas |
| Input verification | ~150k-300k gas |

> **Note:** Actual FHE computation happens off-chain asynchronously (no gas cost).

## Encrypted Type Reference

```solidity
import {FHE, euint32, ebool, eaddress} from "fhevm/lib/FHE.sol";

// Type declarations
euint8 smallNumber;
euint32 balance;
euint256 largeNumber;
ebool flag;
eaddress addr;

// Operations
euint32 sum = FHE.add(a, b);
euint32 product = FHE.mul(a, b);
ebool isEqual = FHE.eq(a, b);
ebool isLess = FHE.lt(a, b);
euint32 result = FHE.select(condition, ifTrue, ifFalse);
```

## Common Patterns

### Confidential Balance Transfer

```solidity
function transfer(address to, euint32 amount) external {
    balances[msg.sender] = FHE.sub(balances[msg.sender], amount);
    balances[to] = FHE.add(balances[to], amount);
}
```

### Request Decryption

```solidity
function revealBalance() external {
    Gateway.requestDecryption(
        balances[msg.sender],
        this.onBalanceRevealed.selector
    );
}

function onBalanceRevealed(uint256 balance) external onlyGateway {
    emit BalanceRevealed(msg.sender, balance);
}
```

### Accept Encrypted Input

```solidity
function deposit(bytes calldata ciphertext, bytes calldata proof) external {
    euint32 amount = FHE.asEuint32(ciphertext, proof);
    balances[msg.sender] = FHE.add(balances[msg.sender], amount);
}
```

## Environment Variables

### Coprocessor

```bash
DATABASE_URL=postgresql://user:pass@localhost/fhevm
RPC_URL=https://eth-node.example.com
PRIVATE_KEY=0x...
GATEWAY_CONTRACT_ADDRESS=0x...
```

### KMS Connector

```bash
DATABASE_URL=postgresql://user:pass@localhost/kms
RPC_URL=https://eth-node.example.com
PRIVATE_KEY=0x...
KMS_CORE_URL=https://kms.example.com
```

## Debugging Tips

**Smart contracts:**
- Use `console.log()` from Hardhat (import `hardhat/console.sol`)
- Check events with `forge test -vvv` for detailed output
- Use Hardhat Network's `console.log` for TypeScript tests

**Rust services:**
- Set `RUST_LOG=debug` for verbose logging
- Use `tracing` macros: `tracing::debug!("message")`
- Check PostgreSQL logs for database issues

**E2E tests:**
- Check `docker-compose logs -f servicename`
- Inspect database: `docker-compose exec postgres psql -U user -d fhevm`
- Verify blockchain state: Use Hardhat console or cast commands

## Useful Links

**Official Documentation:**
- FHEVM Docs: (internal docs links)
- TFHE-rs: https://github.com/zama-ai/tfhe-rs
- OpenZeppelin: https://docs.openzeppelin.com/contracts/

**Development Tools:**
- Hardhat: https://hardhat.org/
- Foundry: https://book.getfoundry.sh/
- Tokio: https://tokio.rs/

---

**Related:**
- [Technology Stack](tech-stack.md) - Complete technology breakdown
- [Glossary](glossary.md) - Term definitions
- [Components](../components/README.md) - Detailed component documentation
