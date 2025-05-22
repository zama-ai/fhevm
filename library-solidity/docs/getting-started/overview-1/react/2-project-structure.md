# Project Structure

In this section, we'll explore the structure of the fhevm React template. Understanding the layout and purpose of each component will help you navigate and extend the project effectively.

## Directory Overview

Here's a high-level overview of the project's directory structure:

```
├── README.md
├── components.json
├── package-lock.json
├── package.json
├── pnpm-lock.yaml
├── postcss.config.js
├── public
├── src
│   ├── abi
│   ├── components
│   ├── config
│   ├── hooks
│   ├── lib
│   ├── pages
│   ├── providers
│   ├── types
│   ├── App.css
│   ├── App.tsx
│   ├── index.css
│   ├── main.tsx
│   └── vite-env.d.ts
├── tsconfig.app.json
├── tsconfig.json
├── tsconfig.node.json
└── vite.config.ts
```

## Key Directories and Files

- **`src/abi/`**: Stores the ABI (Application Binary Interface) files for smart contracts, such as `confidentialErc20Abi.ts`.

- **`src/components/`**: Contains reusable React components organized by functionality. Key subdirectories include:
  - **`src/transfers/`**: Components related to token transfers, such as `ConfidentialTransferForm.tsx`.
  - **`src/ui/`**: UI components using Radix UI, like `button.tsx` and other components you might need for quick prototyping.
  - **`src/wallet/`**: Components for wallet interactions, such as `ConnectWallet.tsx`.

- **`src/hooks/`**: Custom React hooks for various functionalities, including fhevm operations and wallet management.

- **`src/lib/`**: Library files for fhevm integration and utility functions.

- **`src/providers/`**: Context providers for managing global state, like `FhevmProvider.tsx`.


With this understanding of the project structure, you're ready to dive into connecting your application to Web3. Proceed to the next section: [Connecting to Web3](./3-connecting-to-web3.md)

