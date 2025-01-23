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

   ![Create Template](https://colony-recorder.s3.amazonaws.com/files/2025-01-17/49d22a7e-bba5-4dee-b7f9-60d675970fa3/stack_animation.webp)

3. Clone the repository you created:

   ```bash
   git clone <your-new-repo-url>
   cd <your-new-repo>
   ```

   ![Clone Repository](https://colony-recorder.s3.amazonaws.com/files/2025-01-17/bbee17d3-aa84-45ce-92b7-4f9c44239711/stack_animation.webp)

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

## Development Workflow

- Write your **smart contracts** in the `contracts/` directory.
- Write your **tests** in the `tests/` directory.

  ![File Structure](https://ajeuwbhvhr.cloudimg.io/colony-recorder.s3.amazonaws.com/files/2025-01-20/03790808-a211-4b7d-a87f-1734305177b5/ascreenshot.jpeg)

---

### Testing

Run the test suite using:

```bash
pnpm test
```

---

## Deployment

### 1. Prepare for Deployment

1. Generate a mnemonic seed for accounts:

   ```bash
   cast wallet new-mnemonic
   ```

   Copy the mnemonic and paste it into the `.env` file under `MNEMONIC`.

   ![Generate Mnemonic](https://colony-recorder.s3.amazonaws.com/files/2025-01-20/93145686-0ef2-4636-a7ac-9bb25c8a43a6/stack_animation.webp)

2. Add a **Sepolia RPC URL** from a node provider (e.g., Alchemy or Infura).

   Update your `.env` file with the RPC URL:

   - **SEPOLIA_RPC_URL**: `<Your Node Provider URL>`

   ![Add RPC URL](https://ajeuwbhvhr.cloudimg.io/colony-recorder.s3.amazonaws.com/files/2025-01-20/05128377-e779-44bc-b177-a2159e766cd8/ascreenshot.jpeg)

---

### 2. Deploy the Contracts

1. Verify generated accounts:

   ```bash
   npx hardhat get-accounts --num-accounts 5
   ```

   ![Generated Accounts](https://colony-recorder.s3.amazonaws.com/files/2025-01-20/51576e45-72c9-492b-9c7e-0fb3c8393da4/stack_animation.webp)

2. Import the first two accounts (e.g., Alice and Bob) into your wallet and fund them with Sepolia ETH.

   ![Fund Accounts](https://colony-recorder.s3.amazonaws.com/files/2025-01-20/8009d260-6693-40f2-8117-0412fa7f6c39/stack_animation.webp)

3. Deploy the contracts to Sepolia:

   ```bash
   pnpm deploy-sepolia
   ```

   ![Deploy Contracts](https://colony-recorder.s3.amazonaws.com/files/2025-01-20/2dd9fa0a-190a-46c2-b3be-12394bb4de1c/stack_animation.webp)

---

### 3. Verify Deployment

1. After deploying, locate the deployed contract on Sepolia Etherscan:  
   <https://sepolia.etherscan.io/>

   ![View Deployed Contract](https://colony-recorder.s3.amazonaws.com/files/2025-01-20/7c45e315-a133-4592-b856-3ebb85fb23a6/stack_animation.webp)

---

With this setup, you’re ready to develop, test, and deploy privacy-preserving smart contracts using fhEVM and Hardhat. Happy coding! 🚀
