# 2. Connect your wallet to Remix

In this guide, you'll learn how to connect your wallet and the **Zama Plugin** in Remix IDE to interact with fhEVM smart contracts.

## Prerequisites

Before starting, ensure you have the following:

- **MetaMask** or another Ethereum-compatible wallet installed.
- **Zama Plugin** installed in Remix IDE ([See Setting up Remix](remix.md))

{% hint style="danger" %}
Note thate when using Remix to connect a wallet, issues may arise if **multiple** wallet extensions are installed (e.g., MetaMask, Phantom). This is a known issue of Remix that can affect wallet connection functionality.\
\
If you encounter errors, consider keeping **only** **MetaMask** as the active wallet extension, removing other wallet extensions, and refreshing Remix cookies and try to reconnect.
{% endhint %}

## Step 1. Setting up Sepolia testnet

If you're using Metamask, the Sepolia testnet should be pre-configured. Follow the steps to set it up:

1. Open **MetaMask**.
2. Click the **network dropdown** at the top left corner, and select **Sepolia Test Network**.
3. Ensure you have **Sepolia ETH** available. If you don’t have enough ETH, use a **Sepolia faucet** to request free SepoliaETH for testing:
   - [Alchemy Faucet](https://www.alchemy.com/faucets/ethereum-sepolia)
   - [QuickNode Faucet](https://faucet.quicknode.com/ethereum/sepolia)

{% embed url="https://scribehow.com/embed/Google_Chrome_Workflow__hfRvx_T1To-YVlnj28WYng?removeLogo=true&skipIntro=true" %}

{% hint style="info" %}
If Sepolia isn’t visible:

1. Go to **Settings > Advanced**.
2. Toggle **Show test networks** to **ON**.
   {% endhint %}

{% hint style="info" %}
If Sepolia isn’t pre-configured in your wallet, add it manually:

1. Open your wallet’s **network settings**.
2. Click **Add Network** or **Add Network Manually**.
3. Enter the following details:
   - **Network Name**: `Sepolia`
   - **RPC URL**: (provided by your node provider, e.g., Alchemy or Infura)
   - **Chain ID**: `11155111`
   - **Currency Symbol**: `SepoliaETH`
   - **Block Explorer URL**: `https://sepolia.etherscan.io`
     {% endhint %}

## Step 2. Connecting to Zama Plugin

**Zama Plugin** provides the **Zama Coprocessor - Sepolia configuration** that ensures Remix and the wallet are properly set up to interact with fhEVM smart contracts.

To complete the configuration:

1. Open the **Zama Plugin** in Remix from the side pannel.
2. Click **Connect your wallet.**
3. **Confirm** the connection in **MetaMask.**&#x20;
4. In the Zama Plugin, select **Zama Coprocessor - Sepolia**.
5. Click **Use this configuration** to finalize the setup.

Once successful, you should see the green text in the terminal indicating that the configuration is ready.

{% embed url="https://scribehow.com/embed/Google_Chrome_Workflow__pdS0_bEDRNGRMLcJ02A9Dw?removeLogo=true&skipIntro=true" %}

## Step 3. Connecting wallet to Remix

Follow the steps to connect your wallet to Remix:

1. Open Remix and navigate to **Deploy & run transactions**.
2. Under **Environment**, select **Injected Provider - MetaMask**.
3. MetaMask will prompt a connection request. Click **Connect** to proceed.
4. Choose your wallet address in **Account.**

{% embed url="https://scribehow.com/embed/Google_Chrome_Workflow__b4xdbTivQrCjPelyPxI0AQ?removeLogo=true&skipIntro=true" %}

---

Now that your wallet is connected and your SepoliaETH balance is ready, you can proceed to deploy the `ConfidentialERC20Mintable` contract!
