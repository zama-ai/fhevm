# Step 2: Connect your wallet to Remix

Learn how to connect your wallet and the **Zama plugin** in Remix IDE to interact with fhEVM smart contracts.

## Prerequisites

Before starting, ensure you have the following:

- ✅ **MetaMask** or another Ethereum-compatible wallet installed.
- ✅ **Zama Plugin** installed in Remix IDE ([See Setting Up Remix](remix.md))

## Connecting your wallet to Remix

1. Open Remix and navigate to **Deploy and Run Transactions**.
2. Under **Environment**, select **Injected Provider - MetaMask**.
3. MetaMask will prompt a connection request. Click **Connect** to proceed.

   ![Connect Wallet](https://colony-recorder.s3.amazonaws.com/files/2025-01-16/821f9695-9c60-479e-9ce5-63d7d2e97daf/stack_animation.webp)

## Setting up Sepolia testnet

### **Using MetaMask**

1. Open **MetaMask**.
2. Click the **network dropdown** at the top and select **Sepolia Test Network**.

   If Sepolia isn’t visible:

   - Go to **Settings > Advanced**.
   - Toggle **Show Test Networks** to **ON**.

### **Manual network configuration**

If Sepolia isn’t pre-configured in your wallet, add it manually:

1. Open your wallet’s **network settings**.
2. Click **Add Network** or **Add Network Manually**.
3. Enter the following details:

   - **Network Name**: `Sepolia`
   - **RPC URL**: (provided by your node provider, e.g., Alchemy or Infura)
   - **Chain ID**: `11155111`
   - **Currency Symbol**: `ETH`
   - **Block Explorer URL**: `https://sepolia.etherscan.io`

   ![Manual Configuration](https://colony-recorder.s3.amazonaws.com/files/2025-01-16/68cafdfb-2210-4e06-b24a-f39ff96727a3/stack_animation.webp)

## Connecting to the Zama plugin

1. Open the **Zama Plugin** in Remix.
2. Click **Connect Wallet**.
3. When prompted by MetaMask, confirm the connection by clicking **Connect**.

   ![Connect Zama Plugin](https://colony-recorder.s3.amazonaws.com/files/2025-01-16/213f4f6d-f0b7-4bae-be2d-d5e3b8f59ddd/stack_animation.webp)

## Verifying Sepolia ETH balance

1. Open **MetaMask** and ensure you have Sepolia ETH available.

   ![Check Sepolia ETH](https://colony-recorder.s3.amazonaws.com/files/2025-01-16/2cae3f4e-370a-4be0-a071-24b01745bcfc/stack_animation.webp)

2. If you don’t have enough ETH, use a Sepolia faucet to request free test ETH:
   - [Alchemy Faucet](https://www.alchemy.com/faucets/ethereum-sepolia)
   - [QuickNode Faucet](https://faucet.quicknode.com/ethereum/sepolia)

---

With your wallet connected and Sepolia configured, you're ready to deploy and interact with confidential smart contracts! 🎉
