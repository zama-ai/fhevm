---
name: Smart Contracts Bug Report  
about: Use this template to report issues encountered with confidential smart contracts on the FHEVM.  
title: '[BUG] - [Brief Description of Issue]'  
labels: bug  
assignees: ''
---

### **Description**  
Provide a clear and detailed description of the bug. Specify which part of the contract (e.g., encryption, decryption, state updates, branching logic) is malfunctioning and any observed anomalies in the contract’s behavior.

### **Steps to Reproduce**  
Please include a step-by-step guide to replicate the issue:

1. **Smart Contract Version**: Specify the version or branch being tested (e.g., v0.0.0, latest Devnet branch).
2. **Setup**: Outline the environment configuration (e.g., Docker node, Devnet connection, Mocked mode).
3. **Contract Deployment**: Provide details of the deployed contract, including key initial state values and encryption parameters.
4. **Action Sequence**: Describe the specific transactions or function calls leading to the error.
5. **Observed Behavior**: Highlight any transaction failures, unexpected decryption results, or anomalous state changes.

### **Expected Behavior**  
Describe what you expected to happen (e.g., successful decryption of the input, valid output from the encrypted function, no gas estimation errors).

### **Screenshots / Logs**  
Include screenshots, relevant logs, or traces if available, particularly for encrypted computations or gateway interactions.

### **Smart Contract Code Snippet (Optional)**  
If applicable, provide a minimal reproducible example of the smart contract code highlighting the part where the issue occurs:

```solidity
// Example
contract Counter {
    uint32 value;
    function increment() public {
        value += 1;
    }

    function currentValue() public view returns (uint32) {
        return value;
    }
}
```

### **Environment Information**  
Fill in the relevant environment details where the issue was observed:

- **Operating System**: [e.g., Ubuntu 20.04, Windows 10]  
- **Browser**: [e.g., Chrome v90, Safari v14]  
- **FHEVM Version**: [e.g., v1.0.0]  
- **Tooling**: [Hardhat, Remix, FHEVM CLI]  
- **Devnet / Local Node Configuration**:  
  - Chain ID: [e.g., 9000]  
  - RPC URL: [e.g., `https://devnet.zama.ai`]  
  - Faucet Usage: [Yes/No]
