# Environment Setup

In this section, you'll set up your development environment to start building a decentralized application (dApp) using the FHEVM React template.

## Prerequisites

Before you begin, ensure you have the following installed:

- **Node.js** (v20 or higher)
- A package manager: **npm**, **yarn**, or **pnpm**
- **MetaMask** or another Ethereum wallet

## Step 1: Fork the Repository

1. Visit the [FHEVM React Template repository](https://github.com/zama-ai/fhevm-react-template).
2. Click on the "Fork" button to create a copy of the repository under your GitHub account.

## Step 2: Clone Your Repository

Once you've forked the repository, clone it to your local machine:

```bash
git clone https://github.com/your-username/fhevm-react-template
cd fhevm-react-template
```

## Step 3: Install Dependencies

Navigate to the project directory and install the necessary dependencies:

```bash
npm install
# or
yarn install
# or
pnpm install
```

## Step 4: Configure Environment Variables

1. Copy the example environment file to create your own configuration:

```bash
cp .env.example .env
```

2. Open the `.env` file and update it with your specific configuration:

- `VITE_ACL_ADDRESS`: FHEVM specific
- `VITE_KMS_ADDRESS`: FHEVM specific
- `VITE_GATEWAY_URL`: FHEVM specific
- `VITE_PROJECT_ID`: Obtain your project ID by signing up at [reown.com](https://reown.com/). This enables social login and multi-wallet support.
- `VITE_CONF_TOKEN_ADDRESS`: The address of your deployed confidential ERC20 token contract on Sepolia testnet. You'll get this after deploying the smart contract.

## Step 5: Start the Development Server

Run the development server to see your application in action:

```bash
npm run dev
# or
yarn dev
# or
pnpm dev
```

Visit [http://localhost:5173/](http://localhost:5173/) to view your application.

---

Once your environment is set up, you're ready to move on to the next section: [Project Structure](./2-project-structure.md)
