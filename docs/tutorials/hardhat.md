# Step 1: Setting Up Hardhat

This guide walks you through setting up a development environment for creating **privacy-preserving smart contracts** using the Fully Homomorphic Encryption Virtual Machine (**FHEVM**) with Hardhat.

---

## Prerequisites

Before you begin, ensure you have the following installed on your system:

- **Node.js** (v20 or later)
- A package manager: `npm`, `yarn`, or `pnpm`
- **Git**

---

## Getting Started

Learn how to configure your Hardhat project for developing and deploying fhEVM-specific smart contracts.

### 1. Clone the Hardhat Template

1. Go to the [FHEVM Hardhat Template Repository](https://github.com/zama-ai/fhevm-hardhat-template).
2. Create a new repository by clicking "Use this template."
3. Clone the repository you created:

   ```bash
   git clone <your-new-repo-url>
   cd <your-new-repo>
   ```

{% embed url="https://scribehow.com/embed/Step_1__M1Gjr6SAQuOsPyT7luekmw?skipIntro=true&removeLogo=true" %}

---

### 2. Configure Your Environment

1. Copy the environment configuration template:

   ```bash
   cp .env.example .env
   ```

2. Install project dependencies:

   ```bash
   # Using npm
   npm install

   # Using yarn
   yarn install

   # Using pnpm
   pnpm install
   ```

---

### Development Workflow

- Write your **smart contracts** in the `contracts/` directory.
- Write your **tests** in the `tests/` directory.

---

### 3. Testing your contracts

Run the test suite using:

```bash
pnpm test
```

{% embed url="https://scribehow.com/embed/Step_2__X1NmSAwhRiSiToABTDD31Q?skipIntro=true&removeLogo=true" %}

---

## Deployment

### 1. Prepare for Deployment

1. Generate a mnemonic seed for accounts:

   ```bash
   cast wallet new-mnemonic
   ```

   Copy the mnemonic and paste it into the `.env` file under `MNEMONIC`.

2. Add a **Sepolia RPC URL** from a node provider (e.g., Alchemy or Infura).

   Update your `.env` file with the RPC URL:

   - **SEPOLIA_RPC_URL**: `<Your Node Provider URL>`

3. Verify generated accounts:

   ```bash
   npx hardhat get-accounts --num-accounts 5
   ```

4. Import the first two accounts (e.g., Alice and Bob) into your wallet and fund them with Sepolia ETH.

   {% embed url="https://scribehow.com/embed/Step_3__Hbg4nSgdR3KMcCkB4aw8Jw?skipIntro=true&removeLogo=true" %}

---

### 2. Deploy the Contracts

1. Write your **deployments** under the `deploy/` directory.

2. Deploy the contracts to Sepolia:

   ```bash
   pnpm deploy-sepolia
   ```

---

### 3. Verify Deployment

1. After deploying, locate the deployed contract on Sepolia Etherscan:  
   <https://sepolia.etherscan.io/>

   {% embed url="https://scribehow.com/embed/Step_4__b3lGkybMS3ihZa8FklQo5A?skipIntro=true&removeLogo=true" %}

---

With this setup, you’re ready to develop, test, and deploy privacy-preserving smart contracts using fhEVM and Hardhat. Happy coding! 🚀
