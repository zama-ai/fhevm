# Protocol Contracts 🔥

**Location**: `/protocol-contracts/`
**Status**: Active Development
**Purpose**: Protocol-level infrastructure including token, staking, and governance

## Overview

Protocol contracts implement the economic and governance layer of the FHEVM ecosystem. They are organized into domain-specific subdirectories.

## Submodules

| Directory | Purpose |
|-----------|---------|
| `token/` | ZAMA ERC20 token and OFT (Omnichain Fungible Token) for cross-chain |
| `staking/` | Node operator staking mechanisms |
| `governance/` | DAO voting and protocol governance |
| `confidential-wrapper/` | Wraps public tokens for confidential transfers |
| `feesBurner/` | Fee collection and token burning |
| `safe/` | Safe module for protocol administration |

## Key Contracts

**Token Layer:**
- `token/ZamaERC20.sol` - Protocol token implementation
- `token/ZamaOFT.sol` - Omnichain token (LayerZero integration)

**Economic Layer:**
- `staking/OperatorStaking.sol` - Operator staking and slashing
- `staking/OperatorRewarder.sol` - Reward distribution mechanisms
- `confidential-wrapper/Wrapper.sol` - Public-to-confidential token bridge
- `feesBurner/ProtocolFeesBurner.sol` - Fee collection and burning

**Governance Layer:**
- `governance/GovernanceOAppSender.sol` - Cross-chain proposal transmission
- `governance/GovernanceOAppReceiver.sol` - Proposal execution on Gateway
- `safe/AdminModule.sol` - Safe module for admin operations

## Recent Development Focus (Dec 2025)

- **Staking/delegation**: `OperatorStaking` and `OperatorRewarder` implementation
- **Fee management**: Burner implementation and distribution logic
- **Governance**: Safe ownership and admin modules
- **ERC1363 integration**: Callback-based token transfers
- **UUPS upgradeability**: Proxy patterns for contract upgrades

## Deprecation Notice

> ⚠️ **Note**: `ProtocolOperatorRegistry` has been removed. Use `OperatorStaking` for staking functionality.

---

## Staking/Delegation Contracts

The staking system implements a three-tier architecture that enables users to delegate stake to operators while earning rewards. This design separates protocol-level staking from operator-specific delegation and reward distribution.

### Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    User Deposit Flow                            │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  OperatorStaking                                                │
│  - Receives user deposits                                       │
│  - Mints shares to delegators                                   │
│  - Manages redemption requests                                  │
│  - Tracks operator authorization                                │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  ProtocolStaking                                                │
│  - Core staking with cooldown                                   │
│  - sqrt-weighted rewards                                        │
│  - Eligibility management                                       │
│  - Slashing execution                                           │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  OperatorRewarder                                               │
│  - Claims rewards from ProtocolStaking                          │
│  - Distributes to delegators                                    │
│  - Applies operator fee                                         │
└─────────────────────────────────────────────────────────────────┘
```

### Key Contracts

| Contract | File | Purpose |
|----------|------|---------|
| `ProtocolStaking` | `staking/contracts/ProtocolStaking.sol` | Core staking with sqrt-weighted rewards and cooldown |
| `OperatorStaking` | `staking/contracts/OperatorStaking.sol` | Liquid staking derivative for operator delegation |
| `OperatorRewarder` | `staking/contracts/OperatorRewarder.sol` | Reward distribution with operator fee |

### Deposit Flow

1. **User deposits ZAMA tokens** to `OperatorStaking.deposit()`
2. **OperatorStaking stakes** into `ProtocolStaking` on behalf of delegators
3. **Shares are minted** to the user (1:1 initially, then based on exchange rate)
4. **Rewarder hook** is triggered to track reward allocations

```solidity
// User deposits 100 ZAMA to operator
operatorStaking.deposit(100e18, userAddress);

// Or with ERC1363 callback
zamaToken.transferAndCall(operatorStaking, 100e18, userData);
```

### Redemption Flow

Redemption uses a two-phase process with cooldown:

1. **Request redemption**: `requestRedeem()` burns shares and initiates unstaking
2. **Wait for cooldown**: Subject to `ProtocolStaking` unstaking period
3. **Claim assets**: `redeem()` transfers underlying tokens after cooldown

```solidity
// Phase 1: Request (burns shares, starts cooldown)
operatorStaking.requestRedeem(100e18, controller, owner);

// Phase 2: Redeem (after cooldown expires)
operatorStaking.redeem(100e18, receiver, controller);
```

### Reward Mechanics

**ProtocolStaking** uses square-root weighting for rewards:

```
weight = sqrt(stakedAmount)
allocation = historicalReward × (weight / totalWeight)
```

This design:
- Provides diminishing returns on large stakes
- Encourages network decentralization
- Reduces incentive for stake consolidation

**OperatorRewarder** distributes rewards to delegators minus an operator fee:

```solidity
// Operator can claim fee percentage of rewards
uint256 fee = rewards * feeBasisPoints / 10000;
uint256 delegatorRewards = rewards - fee;
```

### Delegation Patterns

**Operator Authorization:**
```solidity
// User authorizes operator to manage redemptions
operatorStaking.setOperator(operatorAddress, true);
```

**Reward Claimer Delegation:**
```solidity
// User authorizes another address to claim rewards
operatorRewarder.setClaimer(claimerAddress);
```

### Slashing

Slashing occurs at the `ProtocolStaking` level and symmetrically affects all delegators:

1. Protocol executes slash on `ProtocolStaking`
2. `totalAssets()` in `OperatorStaking` decreases
3. Share-to-asset exchange rate drops for all holders
4. Losses are proportionally distributed

### ERC1363 Integration

Both `OperatorStaking` and `ZamaERC20` support ERC1363 callbacks, enabling atomic deposit operations:

```solidity
// Single transaction: approve + deposit
zamaToken.transferAndCall(operatorStaking, amount, "");
```

---

## Confidential Wrapper Pattern

The confidential wrapper bridges public ERC20 tokens to confidential tokens using FHE encryption. This enables privacy-preserving token transfers while maintaining 1-to-1 collateral backing.

### Architecture

```
┌──────────────────┐     wrap()      ┌──────────────────┐
│  Public ERC20    │ ───────────────▶│  Wrapper         │
│  (e.g., USDC)    │                 │  Contract        │
└──────────────────┘                 └────────┬─────────┘
                                              │
                                              │ mint()
                                              ▼
                                     ┌──────────────────┐
                                     │  Confidential    │
                                     │  ERC7984 Token   │
                                     │  (encrypted)     │
                                     └──────────────────┘
```

### Key Contracts

| Contract | File | Purpose |
|----------|------|---------|
| `Wrapper` | `confidential-wrapper/contracts/wrapper/Wrapper.sol` | Core wrap/unwrap logic |
| `WrapperUpgradeable` | `confidential-wrapper/contracts/wrapper/WrapperUpgradeable.sol` | UUPS upgradeable version |
| `RegulatedERC7984Upgradeable` | `confidential-wrapper/contracts/token/RegulatedERC7984Upgradeable.sol` | Confidential token with sanctions |
| `WrapperFactory` | `confidential-wrapper/contracts/factory/WrapperFactory.sol` | Deploys wrapper pairs |
| `FeeManager` | `confidential-wrapper/contracts/admin/FeeManager.sol` | Centralized fee configuration |

### Wrap Flow (Public → Confidential)

```
User sends 100 USDC
        │
        ▼
┌───────────────────────────────────────┐
│ 1. Calculate fees                     │
│    baseFee = amount × feeBasisPoints  │
│    wrapDust = amount % rate           │
│    totalFee = baseFee + wrapDust      │
└───────────────────────────────────────┘
        │
        ▼
┌───────────────────────────────────────┐
│ 2. Transfer tokens                    │
│    - Fee to feeRecipient              │
│    - Remainder held by Wrapper        │
└───────────────────────────────────────┘
        │
        ▼
┌───────────────────────────────────────┐
│ 3. Mint confidential tokens           │
│    mintAmount = transferAmount / rate │
│    cToken.mint(user, mintAmount)      │
└───────────────────────────────────────┘
```

**Rate Conversion:**
- Confidential tokens use `euint64` (64-bit encrypted integers)
- `rate = 10^(decimals - 6)` for tokens with >6 decimals
- Example: 18-decimal token has `rate = 10^12`

### Unwrap Flow (Confidential → Public)

Unwrapping is a **two-stage asynchronous process**:

**Stage 1: Initiation**
```solidity
// User transfers cTokens to wrapper
cToken.confidentialTransferAndCall(
    wrapper,
    encryptedAmount,
    inputProof,
    abi.encode(receiver, refund, callbackData)
);
```

The wrapper:
1. Burns the encrypted tokens
2. Marks amounts as publicly decryptable via `FHE.makePubliclyDecryptable()`
3. Stores receiver info with committed fee rate
4. Emits `UnwrappedStarted` event

**Stage 2: Finalization**
```solidity
// Called with decrypted values and proof
wrapper.finalizeUnwrap(
    requestId,
    abiEncodedClearBurnAmounts,
    decryptionProof
);
```

The wrapper:
1. Verifies decryption proof via `FHE.checkSignatures()`
2. Calculates fee and transfer amounts
3. Transfers underlying tokens to receiver
4. On failure: mints cTokens back to refund address

### Security Model

**Collateral Parity Invariant:**
- Wrapper always holds exactly enough underlying tokens to back all minted cTokens
- Even if transfers fail, tokens are re-minted to maintain parity

**Decryption Trust:**
- Off-chain decryption proof is cryptographically verified
- No on-chain decryption occurs

**Protection Mechanisms:**
- Reentrancy guard on all state-changing functions
- Replay protection via unique requestIds
- Fee-on-transfer token handling via balance tracking

### Operator System

Users can authorize operators to finalize unwraps on their behalf:

```solidity
// Authorize operator for 1 hour
wrapper.setFinalizeUnwrapOperator(operatorAddress, block.timestamp + 3600);
```

This enables relayers and intent solvers to complete unwraps.

### Regulatory Features

- **SanctionsList**: Blocks transfers to/from sanctioned addresses
- **Regulator role**: Can decrypt specific handles for compliance
- Enforced at ERC7984 transfer level

---

## Governance Mechanisms

Governance operates cross-chain: the DAO on Ethereum votes on proposals that execute on the Gateway chain via LayerZero messaging and Safe multi-sig.

### Architecture

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Aragon DAO     │     │  LayerZero      │     │  Gateway Safe   │
│  (Ethereum)     │────▶│  Protocol       │────▶│  (Gateway)      │
└─────────────────┘     └─────────────────┘     └─────────────────┘
        │                                               │
        ▼                                               ▼
┌─────────────────┐                            ┌─────────────────┐
│  GovernanceOApp │                            │  GovernanceOApp │
│  Sender         │                            │  Receiver       │
└─────────────────┘                            └────────┬────────┘
                                                        │
                                                        ▼
                                               ┌─────────────────┐
                                               │  AdminModule    │
                                               │  (Safe Module)  │
                                               └────────┬────────┘
                                                        │
                                                        ▼
                                               ┌─────────────────┐
                                               │  Protocol       │
                                               │  Contracts      │
                                               └─────────────────┘
```

### Key Contracts

| Contract | File | Purpose |
|----------|------|---------|
| `GovernanceOAppSender` | `governance/contracts/GovernanceOAppSender.sol` | Sends proposals via LayerZero |
| `GovernanceOAppReceiver` | `governance/contracts/GovernanceOAppReceiver.sol` | Receives and routes to Safe |
| `AdminModule` | `safe/contracts/AdminModule.sol` | Executes transactions through Safe |

### Proposal Lifecycle

1. **DAO Vote (Ethereum)**: Aragon DAO votes on proposal
2. **Send Cross-Chain**: DAO calls `sendRemoteProposal()` on GovernanceOAppSender
3. **LayerZero Relay**: Message transmitted to Gateway chain
4. **Receive & Route**: GovernanceOAppReceiver calls AdminModule
5. **Safe Execution**: AdminModule executes via Safe's `execTransactionFromModuleReturnData()`

```solidity
// On Ethereum: Send proposal to Gateway
governanceOAppSender.sendRemoteProposal(
    targets,      // Contract addresses to call
    values,       // ETH values (usually 0)
    signatures,   // Function signatures
    calldatas,    // Encoded function arguments
    operations    // CALL or DELEGATECALL
);
```

### Safe Integration

The Safe wallet on Gateway owns protocol infrastructure:

```
Safe Wallet
    │
    ├── AdminModule (enabled module)
    │       │
    │       └── Only GovernanceOAppReceiver can call
    │
    └── Owns: GatewayConfig, ProtocolStaking, etc.
```

**AdminModule** validates:
- Caller is the authorized admin (GovernanceOAppReceiver)
- All input arrays have matching lengths
- Supports both CALL and DELEGATECALL operations

### UUPS Upgrade Pattern

Protocol contracts use UUPS (EIP-1822) for upgradeability:

```solidity
contract ProtocolStaking is
    UUPSUpgradeable,
    AccessControlDefaultAdminRulesUpgradeable,
    ERC20VotesUpgradeable
{
    function _authorizeUpgrade(address) internal override onlyRole(UPGRADER_ROLE) {}
}
```

**Storage Pattern (ERC-7201):**
```solidity
bytes32 private constant STORAGE_LOCATION =
    keccak256(abi.encode(uint256(keccak256("fhevm_protocol.storage.ProtocolStaking")) - 1))
    & ~bytes32(uint256(0xff));
```

### Permission Hierarchy

```
Aragon DAO (Ethereum)
    │
    └── GovernanceOAppSender
            │
            └── [LayerZero]
                    │
                    └── GovernanceOAppReceiver
                            │
                            └── AdminModule
                                    │
                                    └── Safe Wallet
                                            │
                                            └── Protocol Contracts
```

---

## Fee Economics

The protocol collects fees from wrapper operations and burns ZAMA tokens to create deflationary pressure.

### Fee Flow

```
┌─────────────────┐
│  Wrapper Fees   │  (wrap/unwrap basis points)
│  (Gateway)      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  FeesSenderTo   │  Accumulates ZAMA fees
│  Burner         │
│  (Gateway)      │
└────────┬────────┘
         │
         │ sendFeesToBurner()
         │ [LayerZero]
         │
         ▼
┌─────────────────┐
│  Protocol       │  Receives ZAMA tokens
│  FeesBurner     │
│  (Ethereum)     │
└────────┬────────┘
         │
         │ burnFees()
         ▼
┌─────────────────┐
│  Permanently    │
│  Burned         │
└─────────────────┘
```

### Key Contracts

| Contract | File | Purpose |
|----------|------|---------|
| `FeeManager` | `confidential-wrapper/contracts/admin/FeeManager.sol` | Configures fee basis points |
| `FeesSenderToBurner` | `feesBurner/contracts/FeesSenderToBurner.sol` | Sends fees cross-chain |
| `ProtocolFeesBurner` | `feesBurner/contracts/ProtocolFeesBurner.sol` | Burns ZAMA on Ethereum |

### Fee Types

**Wrapper Fees:**
- `wrapFeeBasisPoints`: Fee on wrap operations (ceiling division)
- `unwrapFeeBasisPoints`: Fee on unwrap operations
- `swapperFee`: Reduced fee for whitelisted swap contracts

**Collection:**
```solidity
// Wrap fee calculation (ceiling division prevents leakage)
uint256 fee = (amount * basisPoints + 9999) / 10000;
```

### Burning Mechanism

Anyone can trigger burning of accumulated fees:

```solidity
// On Ethereum: Burn all held ZAMA tokens
protocolFeesBurner.burnFees();
// Emits: FeesBurned(amount)
```

Uses `ERC20Burnable.burn()` to permanently remove tokens from circulation.

### Economic Incentives

1. **Deflationary Pressure**: Fee burning reduces total ZAMA supply
2. **Swapper Incentives**: Reduced fees for swap contracts encourage composability
3. **Operator Alignment**: OperatorRewarder fees align operator incentives with staking growth

---

## Cross-Chain Token Flows

ZAMA tokens use LayerZero's OFT (Omnichain Fungible Token) standard for cross-chain transfers.

### Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Ethereum                                 │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐        ┌─────────────────┐                    │
│  │ ZamaERC20   │◀──────▶│ ZamaOFTAdapter  │                    │
│  │ (ERC20)     │  lock/ │ (OFT wrapper)   │                    │
│  │             │ unlock │                 │                    │
│  └─────────────┘        └────────┬────────┘                    │
└──────────────────────────────────┼──────────────────────────────┘
                                   │
                          [LayerZero Protocol]
                                   │
┌──────────────────────────────────┼──────────────────────────────┐
│                        Gateway Chain                            │
├──────────────────────────────────┼──────────────────────────────┤
│                          ┌───────┴───────┐                      │
│                          │   ZamaOFT     │                      │
│                          │ (native OFT)  │                      │
│                          │  mint/burn    │                      │
│                          └───────────────┘                      │
└─────────────────────────────────────────────────────────────────┘
```

### Key Contracts

| Contract | File | Purpose |
|----------|------|---------|
| `ZamaERC20` | `token/contracts/ZamaERC20.sol` | Main token on Ethereum |
| `ZamaOFTAdapter` | `token/contracts/ZamaOFTAdapter.sol` | Adapts ERC20 for OFT |
| `ZamaOFT` | `token/contracts/ZamaOFT.sol` | Native OFT on Gateway |

### Cross-Chain Transfer: Ethereum → Gateway

```solidity
// 1. Approve adapter
zamaERC20.approve(zamaOFTAdapter, amount);

// 2. Send cross-chain
zamaOFTAdapter.send(
    SendParam({
        dstEid: GATEWAY_EID,      // Destination chain ID
        to: bytes32(receiver),     // Receiver address
        amountLD: amount,          // Amount in local decimals
        minAmountLD: minAmount,    // Minimum received
        extraOptions: options,     // Gas options
        composeMsg: "",            // Optional compose message
        oftCmd: ""                 // OFT command
    }),
    fee,
    refundAddress
);
```

**Flow:**
1. ZamaOFTAdapter locks ERC20 tokens
2. LayerZero relays message to Gateway
3. ZamaOFT mints equivalent tokens on Gateway

### Cross-Chain Transfer: Gateway → Ethereum

```solidity
// On Gateway
zamaOFT.send(sendParam, fee, refundAddress);
```

**Flow:**
1. ZamaOFT burns tokens on Gateway
2. LayerZero relays message to Ethereum
3. ZamaOFTAdapter unlocks ERC20 tokens

### Decimal Handling

OFT uses "shared decimals" for cross-chain consistency:
- Local decimals: Token's native decimal places
- Shared decimals: Normalized for cross-chain (usually 6)
- Conversion: `amountSD = amountLD / conversionRate`

### ZamaERC20 Features

Beyond standard ERC20:
- **ERC20Burnable**: Token holders can burn their tokens
- **ERC20Permit**: Gasless approvals via signatures (EIP-2612)
- **ERC1363**: Callback-based transfers (`transferAndCall`)
- **AccessControl**: Role-based minting (MINTER_ROLE)
- **Pausable**: Minting can be paused (MINTING_PAUSER_ROLE)

```solidity
// ERC1363: Atomic transfer + callback
zamaToken.transferAndCall(receiver, amount, data);

// ERC20Permit: Gasless approval
zamaToken.permit(owner, spender, value, deadline, v, r, s);
```

---

**Related:**
- [Gateway Contracts](gateway-contracts.md) - ProtocolPayment integrates with Gateway
- [Reference: Technology Stack](../reference/tech-stack.md) - ERC1363, UUPS patterns
- [Workflows: Decryption Pipeline](../workflows/decryption-pipeline.md) - FHE decryption process
