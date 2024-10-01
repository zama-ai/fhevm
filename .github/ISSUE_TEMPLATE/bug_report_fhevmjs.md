---
name: `fhevmjs` Bug report
about: Use this template to report issues encountered while using the `fhevmjs` library for interacting with fhEVM smart contracts.
title: '[BUG]'
labels: bug
assignees: ''
---

### **Description**  
Provide a clear and detailed description of the issue encountered. Include relevant context such as the operation that failed (e.g., encryption, decryption, reencryption, proof verification), error messages, and unexpected outputs.

### **Steps to Reproduce**  
Please list all the necessary steps to reproduce the issue:

1. **Environment Setup**: Specify how `fhEVMjs` is configured (e.g., Node.js, browser-based application, frontend framework like React/Vue).
2. **Library Version**: State the exact `fhEVMjs` version (e.g., `v0.2.1`, `main` branch).
3. **Smart Contract Interaction**: If applicable, provide the exact method calls and arguments used in the interaction:
   ```javascript
   const encryptedBalance = await instance.balanceOf(userAddress);
   ```
4. **Observed Behavior**: Describe any anomalies, errors, or exceptions that were thrown. Provide stack traces if available.

### **Expected Behavior**  
A clear and concise description of the expected behavior (e.g., successful encryption of parameters, correct reencryption result, accurate balance retrieval).

### **Screenshots / Logs**  
If applicable, provide screenshots or logs that capture the issue, particularly during encryption, reencryption, or smart contract interactions. Include any console errors, warnings, or unexpected outputs.

### **Environment Information**  
Fill in the relevant environment details where the issue was observed:

- **Browser**: [e.g., Chrome v90, Firefox v88]  
- **Node.js Version**: [e.g., `v14.17.0`]  
- **`fhEVMjs` Version**: [e.g., `v0.2.1`]  
- **Package Manager**: [npm, yarn, pnpm]  
- **Network Configuration**:  
  - **Chain ID**: [e.g., `9000`]  
  - **RPC URL**: [e.g., `https://devnet.zama.ai`]  
  - **Gateway URL**: [e.g., `https://gateway.devnet.zama.ai`]  

### **Steps Taken to Debug**  
Outline any steps you’ve taken to troubleshoot the issue (e.g., using mocked mode, enabling verbose logging, testing with a different environment or network).

### **Resources / References**  
If applicable, include links to any related documentation or reference materials that may help us understand the context:

By including these details, we can ensure a faster and more accurate resolution for your issue. Thank you for helping improve `fhEVMjs`!
