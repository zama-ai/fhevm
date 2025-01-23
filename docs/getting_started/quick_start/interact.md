# Step 4: Interacting with Your Contract

After deploying your first **fhEVM** contract using **Remix**, follow these steps to interact with it using the **Zama plugin**.

## 1. Connect the Deployed Contract

1. Copy the deployed contract address from Remix.
2. Open the **Zama Plugin** in Remix.
3. Paste the contract address into the **"At Address"** field under the **Deploy** section and click **At Address**.

   ![Paste Contract Address](https://colony-recorder.s3.amazonaws.com/files/2025-01-16/3adc23b0-4914-40fd-97b7-2f251b905e8b/stack_animation.webp)

## 2. Verify Contract Visibility

- If the address was entered correctly, the `MyConfidentialERC20.sol` contract will be displayed in the Zama plugin interface.

  ![Contract Visibility](https://ajeuwbhvhr.cloudimg.io/colony-recorder.s3.amazonaws.com/files/2025-01-16/1adf1fef-d2f0-432c-85b2-8a0dcdd9f38c/ascreenshot.jpeg)

## 3. Mint Tokens to Your Account

1. In the **mint** function, specify the amount of tokens to mint (e.g., `10000`).
2. Confirm the transaction in **MetaMask**.

   ![Mint Tokens](https://colony-recorder.s3.amazonaws.com/files/2025-01-16/16476b39-2740-48ad-bcb8-7780035656e4/stack_animation.webp)

## 4. Verify Total Supply

- After a successful mint transaction, the **totalSupply** of the token should update to reflect the minted tokens (e.g., `10000`).

  ![Verify Total Supply](https://colony-recorder.s3.amazonaws.com/files/2025-01-16/7487004f-40bd-4455-9f00-f484da918a8f/stack_animation.webp)

## 5. Check your balance

Here your balance is encrypted you will have to preform re-encryption.

1. Use the **balanceOf** function to check your account balance.
2. Perform **re-encryption** to decrypt the ciphertext and view the balance in plaintext.

   ![Check Balance](https://colony-recorder.s3.amazonaws.com/files/2025-01-16/999cd003-f088-449c-978a-9ed1b158e00e/stack_animation.webp)

## 6. Transfer tokens

When transfering the tokens

1. Use the **transfer** function to send tokens to another account.
2. Specify the recipient’s address and the amount (e.g., `1000`).

   ![Transfer Tokens](https://colony-recorder.s3.amazonaws.com/files/2025-01-16/4f6320c3-0649-4402-ac47-68f9e8800bf1/stack_animation.webp)

**Provide encrypted input proof**

1. Under **inputProof**, check the **Input** box.
2. Set the following parameters:
   - **Amount**: `1000`
   - **Type**: `euint64`
3. Click **Submit** to proceed.

   ![Provide Input Proof](https://colony-recorder.s3.amazonaws.com/files/2025-01-16/de6141a7-4e85-4bb0-a5fd-9cc0e44807c1/stack_animation.webp)

## 7. Verify Updated Balance

1. Use the **balanceOf** function again to check your updated balance.
2. Perform **re-encryption** to confirm the changes (e.g., `9000` tokens remaining).

   ![Verify Updated Balance](https://colony-recorder.s3.amazonaws.com/files/2025-01-17/41be5952-5036-41ed-b0c6-be78b3490275/stack_animation.webp)

> **Note**: Always re-encrypt to validate ciphertext transformations and confirm operations.

---

## Next Steps

🎉 **Congratulations on completing this tutorial!** You’ve taken the first step in building confidential smart contracts using **fhEVM**.

To continue your journey and deepen your knowledge, explore the resources below:

---

### 🌟 **Resources**

#### 📄 **White Paper**

Understand the core technology behind fhEVM, including its cryptographic foundations and use cases.  
👉 [**Confidential EVM Smart Contracts using Fully Homomorphic Encryption**](https://github.com/zama-ai/fhevm/blob/main/fhevm-whitepaper-v2.pdf)

---

#### 🎓 **Demos and Tutorials**

Expand your skills with hands-on demos and tutorials crafted to guide you through various real-world scenarios.  
🔗 [**Visit the Tutorials Page**](https://docs.zama.ai/fhevm/tutorials/see-all-tutorials)

---

#### 📘 **Comprehensive Documentation**

Unlock the full potential of fhEVM with these in-depth resources:

- [**fhEVM Documentation**](https://docs.zama.ai/fhevm): Explore everything from architecture to advanced use cases.
- [**AI-Powered Solidity Developer**](https://chatgpt.com/g/g-67518aee3c708191b9f08d077a7d6fa1-zama-solidity-developer): Get instant coding assistance with our custom ChatGPT model tailored for Solidity and fhEVM.

---

### 🛠️ **Development Templates**

Start building with ready-to-use templates for smart contracts and front-end frameworks:

#### **Smart Contract Development**

- 🔧 [**Hardhat Template**](https://github.com/zama-ai/fhevm-hardhat-template):  
  A developer-friendly starting point for building and testing smart contracts on fhEVM.
- 💻 [**fhEVM Contracts Library**](https://github.com/zama-ai/fhevm-contracts):  
  Access standardized contracts for encrypted operations.

#### **Frontend Frameworks**

- 🌐 [**React.js Template**](https://github.com/zama-ai/fhevm-react-template):  
  Quickly develop FHE-compatible dApps using a clean React.js setup.
- ⚡ [**Next.js Template**](https://github.com/zama-ai/fhevm-next-template):  
  Build scalable, server-rendered dApps with FHE integration.
- 🖼️ [**Vue.js Template**](https://github.com/zama-ai/fhevm-vue-template):  
  Develop responsive and modular dApps with FHE support in Vue.js.

---

### 🚀 **What’s Next?**

- Experiment with encrypted operations, such as **confidential voting**, **private auctions**, or **secure data sharing**.
- Join the community on [Discord](https://discord.gg/zama-ai) or [GitHub Discussions](https://github.com/zama-ai/fhevm/discussions) to collaborate and share ideas.
- Share your feedback to help us improve by taking the [Developer Survey](https://zama.ai/survey).

🎯 Keep building, innovating, and leading the way in privacy-preserving smart contracts!
