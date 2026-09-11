# Documentation Roadmap

This page tracks areas that need deeper documentation, prioritized by component activity level and development needs.

## Priority System

Documentation priorities are based on:
- 🔥 **High Priority**: Active development components, frequently changing
- ✅ **Medium Priority**: Stable components, foundational understanding needed
- 📦 **Lower Priority**: Infrastructure, less critical for understanding core protocol

## High Priority (🔥 Active Components)

### 1. Coprocessor Deep Dives

**Worker Architecture**
- Detail tfhe-worker, zkproof-worker, sns-worker implementations
- Document processing pipelines and optimization strategies
- Explain TFHE-rs integration and GPU utilization

**Scheduler and Job Orchestration**
- Job lifecycle from event reception to result submission
- Priority queues, retry logic, and failure handling
- Database schema and query optimization

**GPU Optimization**
- GPU scheduling and memory management
- Performance tuning for FHE operations
- Metrics and monitoring

**Consensus Mechanism**
- How multiple coprocessors coordinate
- Threshold agreement on computation results
- Byzantine fault tolerance properties

### 2. Gateway Contracts

**Gateway Consensus Mechanism**
- Threshold-based consensus for ciphertext commits
- How multiple coprocessors agree on computation results
- Verification and slashing mechanisms

**Multichain ACL Flow**
- Access control delegations across different host chains
- Cross-chain permission synchronization
- LayerZero integration details

**Payment Protocol Design**
- Fee collection, distribution, and operator compensation
- Economic incentives and game theory
- ProtocolPayment contract deep dive

### 3. KMS Connector

**KMS Integration Flow**
- Complete flow from decryption request to signed response
- Sequence diagrams and error handling patterns
- Performance characteristics and optimization

**Threshold Signature Scheme**
- MPC-based threshold signature mechanism
- Trust model and security guarantees
- Key generation, rotation, and reshare protocols

**External KMS Core Integration**
- Interface contract with external KMS Core service
- API expectations and deployment patterns
- Operational considerations

### 4. Protocol Contracts

**Staking/Delegation Contracts**
- OperatorStaking and Rewarder implementations
- Staking mechanics, slashing conditions, reward calculation
- Delegation patterns and operator economics

**Confidential Wrapper Pattern**
- How public ERC20 tokens are wrapped for confidential use
- Bridge security model and liquidity management
- Unwrap process and guarantees

**Governance Mechanisms**
- DAO voting contracts and proposal lifecycle
- Safe integration for admin operations
- Upgrade patterns and security

## Medium Priority (✅ Stable Components)

### 5. Host Contracts

**FHEVMExecutor Operators**
- All 20+ FHE operators and symbolic execution semantics
- Type compatibility and conversion rules
- Gas costs and performance characteristics

**ACL Permission Model**
- allowList, denyList, and delegation mechanisms
- Permission grant/revoke flows
- Cross-contract access patterns

**HCU Limit Enforcement**
- 20M HCU/tx and 5M depth limits
- How limits prevent DoS attacks
- Optimization strategies for complex operations

### 6. Solidity Library

**Encrypted Type System**
- All supported types, bit sizes, and conversion rules
- Memory and gas implications
- Type safety guarantees

**Codegen System**
- Operator overload generation process
- How to extend the library with new operations
- Codegen toolchain and customization

**Best Practices**
- Recommended patterns for confidential smart contracts
- Common pitfalls and how to avoid them
- Optimization strategies

**Examples Deep-Dive**
- Walkthrough of reference implementations
- EncryptedERC20, ConfidentialVoting, etc.
- Detailed explanations with commentary

## Lower Priority (📦 Infrastructure)

### 7. Testing Infrastructure

**Mock FHE System**
- SQLite-backed mocking implementation
- How to write tests using mock FHE
- Performance characteristics vs real FHE

**E2E Testing Patterns**
- docker-compose setup and component interaction
- Writing effective E2E tests
- Debugging techniques for integration tests

**Test Fixtures**
- Reusable test data and scenarios
- Contract deployment helpers
- Common test utilities

### 8. Deployment

**Helm Charts Deep Dive**
- Kubernetes deployment process
- Configuration options and tuning
- Multi-environment strategies (dev/staging/prod)

**Operational Best Practices**
- Monitoring and alerting
- Backup and disaster recovery
- Performance tuning

**Docker Compose Stack**
- test-suite setup and component configuration
- How components interact in local development
- Debugging issues in the stack

### 9. CI/CD Pipeline

**GitHub Actions Workflows**
- Testing strategy and coverage requirements
- Build and publish pipelines
- Security scanning and auditing

**Release Process**
- Versioning strategy
- Changelog generation
- Deployment automation

## Contribution Guidelines

When expanding documentation:

1. **High-priority areas** (🔥): Document current state, note potential for change
2. **Stable areas** (✅): Create comprehensive, detailed documentation
3. **Infrastructure** (📦): Focus on operational patterns and deployment

### Documentation Standards

Each expanded section should include:
- **Overview**: High-level summary
- **Detailed explanation**: Step-by-step with examples
- **Diagrams**: Architecture diagrams, sequence diagrams
- **Code examples**: Working code snippets
- **Edge cases**: Error handling and unusual scenarios
- **Performance**: Gas costs, timing, optimization tips
- **Related sections**: Cross-links to related docs

## Tracking Progress

Each TODO in component documentation maps to a section above. As documentation is created:
1. Remove [TODO] marker from source document
2. Add link to new detailed documentation
3. Update this roadmap to reflect completion

---

**Related:**
- [Component Health](../component-health.md) - Which areas are actively evolving
- [Components](../components/README.md) - Component overview with TODO markers
- [Workflows](../workflows/README.md) - Workflow documentation with TODOs
