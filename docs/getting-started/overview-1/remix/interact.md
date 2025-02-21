# 4. Interacting with the contract

After deploying your first **fhEVM** contract using **Remix**, this guide shows you how to interact with it directly in Remix using the **Zama Plugin**.

## Prerequisite

Before interacting with your deployed contract, ensure the following:

- **Deployment completed**: You have successfully deployed the `MyConfidentialERC20` contract (see [Step 3](deploying_cerc20.md)).
- **MetaMask wallet**: Your MetaMask wallet is connected to the Sepolia testnet(see [Step 2](connect_wallet.md)). You might want to prepare an additional wallet as the receiver to mock the transfer function.
- **Zama Plugin in Remix**: The Zama Plugin is installed and accessible in Remix (see [Step 1](remix.md)).

## Step 1. Connecting the deployed contract

To perform transactions directly in Remix, your contract needs to be connected to the **Zama Plugin**:

1. Open Deploy & run transaction from the side bar
2. In "**Deployed Contract**", copy the address of the `MYCONFIDENTIALERC20` contract that you just deployed.
3. Open the **Zama Plugin** from the side menu.
4. Paste the contract address into the **"At address"** field under the **Deploy** section.
5. Click **At address**.

If the address was entered correctly, the `MyConfidentialERC20.sol` contract will be displayed in the "**Deployed Contract**" inside the **Zama Plugin**.

{% embed url="https://scribehow.com/embed/Load_MyConfidentialERC20_Contract_in_Remix__tJ1PmbA4TuGQ2fj6kMdMtQ?removeLogo=true&skipIntro=true" %}

Click to expand the contract, you'll see the interface to interact with all the functions of your contract. `ConfidentialERC20Mintable` supports all standard ERC-20 functions, but adapted for encrypted values, including:

- `transfer`: Securely transfers encrypted tokens.
- `approve`: Approves encrypted amounts for spending.
- `transferFrom`: Transfers tokens on behalf of another address.
- `balanceOf`: Returns the encrypted balance of an account.
- `totalSupply`: Returns the total token supply.

## Step 2. Mint tokens to your account

From here, you can mint confidential ERC20 token to your account:

1. Copy your wallet address from **MetaMask.**
2. Inside **Zama Plugin,** click to expand the **mint** function of your contract.
3. Enter your wallet address and the amount of tokens to mint (e.g., `1000`).
4. Click `Submit`.
5. Confirm the transaction in **MetaMask**.

Once sccussful, you should see the message in the terminal.

{% embed url="https://scribehow.com/embed/Google_Chrome_Workflow__I0epwZMASXuUfaqe3iU3jg?removeLogo=true&skipIntro=true" %}

## Step 3. Verify total supply

After a successful mint transaction, click the **totalSupply** to reflect the minted tokens (e.g., `1000`).

{% embed url="https://scribehow.com/embed/Google_Chrome_Workflow__uV00AdSrSpO33EVGfI2LQg?removeLogo=true&skipIntro=true" %}

## Step 4. Check your balance

To verify your account balance:

1. Click to expand the **balanceOf** function.
2. Enter your wallet address.
3. Click `Submit` to verify your account balance.

Your balance is stored as encrypted data using FHE. No one else can view if except you.

To view the balance in plaintext:

- Click the **re-encrypt** option
- Confirm the transaction in Metamask

You can see that the ciphertext is decrypted and your balance is the amount that you just minted.

{% embed url="https://scribehow.com/embed/44__Q0zdu_sETU2e9tVRkvfxkQ?removeLogo=true&skipIntro=true" %}

## Step 5. Transfer tokens

To transfer confidential ERC20 tokens to another account:

1. Copy the address of the receiver's wallet.
2. Click **transfer** to expand the function, set the following parameters:
   - **To**: Enter the wallet address of **reveiver.**
   - **encryptedAmount**: Specify the amount that you want to transfer (e.g.`1000`). Choose `euint64`.
   - **inputProof**: Check the **input** box.
3. Click **Submit** to proceed.
4. Confirm the transaction in **MetaMask**.

If successful, you should see the message in the terminal.

{% embed url="https://scribehow.com/embed/Google_Chrome_Workflow__3owHVNoyTQmsgEOoLWcB5Q?removeLogo=true&skipIntro=true" %}

## Step 6. Verify updated balance

After making a transfer, you can verify your updated account balance:

1. Use the **balanceOf** function again to check the updated balance of your **original account** (see the [Step 5: Check your balance](interact.md#5-check-your-balance).)
2. Perform **re-encryption** to confirm the changes, you should see the remaining token in your account.(e.g., `900` tokens remaining).

{% embed url="https://scribehow.com/embed/Google_Chrome_Workflow__SB36tWugQLG2PdPwG-3iHw?removeLogo=true&skipIntro=true" %}

{% hint style="info" %}
Always re-encrypt to validate ciphertext transformations and confirm operations.
{% endhint %}

---

## Next steps

🎉 **Congratulations on completing this tutorial!** You’ve taken the first step in building confidential smart contracts using **fhEVM**. It's time now to take the next step and build your own confidential dApps!

### 1. Resources

To continue your journey and deepen your knowledge, explore the resources below.

- [**Read the Whitepaper**](https://github.com/zama-ai/fhevm/blob/main/fhevm-whitepaper-v2.pdf): Understand the core technology behind fhEVM, including its cryptographic foundations and use cases.
- [**See more demos and tutorials**](../../../tutorials/see-all-tutorials.md): Expand your skills with hands-on demos and tutorials crafted to guide you through various real-world scenarios.
- [**Try out AI coding assistant**](https://chatgpt.com/g/g-67518aee3c708191b9f08d077a7d6fa1-zama-solidity-developer): If you have a chatGPT plus account, try out our custom ChatGPT model tailored for Solidity and fhEVM developers.

### 2. Tools

Use out-of-box templates and frameworks designed for developers to build confidential dapps easily.

**Smart contract development**

- [**Hardhat Template**](https://github.com/zama-ai/fhevm-hardhat-template): A developer-friendly starting point for building and testing smart contracts on fhEVM.
- [**fhEVM Contracts Library**](https://github.com/zama-ai/fhevm-contracts): Access standardized contracts for encrypted operations.

**Frontend development**

- [**React.js Template**](https://github.com/zama-ai/fhevm-react-template): Quickly develop FHE-compatible dApps using a clean React.js setup.
- [**Next.js Template**](https://github.com/zama-ai/fhevm-next-template): Build scalable, server-rendered dApps with FHE integration.
- [**Vue.js Template**](https://github.com/zama-ai/fhevm-vue-template): Develop responsive and modular dApps with FHE support in Vue.js.

### 3. Community

Join the community to shape the future of blockchain together with us.

- [**Discord**](https://discord.gg/zama-ai): Join the community to get the latest update, have live discussion with fellow developers and Zama team.
- [**Community Forum**](https://community.zama.ai/): Get support on all technical questions related to fhEVM
- [**Zama Bounty Program**](https://github.com/zama-ai/bounty-program): Participate to tackle challenges and earn rewards in cash.
