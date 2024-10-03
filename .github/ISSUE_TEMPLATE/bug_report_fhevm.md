---
name: Smart Contracts Bug Report
about: Use this template to report issues encountered with confidential smart contracts on the FHEVM.
title: ""
labels: bug
assignees: ""
---

### **Description**

Provide a clear and detailed description of the bug.

### **Expected Behavior**

Describe what you expected to happen (e.g., successful decryption of the input, valid output from the encrypted function, no gas estimation errors).

### **Screenshots / Logs**

Include screenshots, relevant logs, or traces if available, particularly for encrypted computations or gateway interactions.

### **Smart Contract Code Snippet**

If applicable, provide a minimal reproducible example of the smart contract code highlighting the part where the issue occurs:

<details><summary>Example</summary>
<p>

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

</p>
</details>

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
