# React.js Tutorial

This tutorial is designed to provide you with a fully functional frontend for interacting with your previously deployed confidential ERC20 smart contract using Zama's `fhevmjs`. By simply connecting the environment variables, you'll have a complete project where you can perform token transfers and view your token balance. Behind the scenes, encryption and decryption are handled seamlessly, connecting to `fhevmjs`.

This `fhevm` React template may not be the most minimal setup, but it offers the best configuration for quickly getting a project up and running, such as in a hackathon setting with minimal customization. If you're working on a more long-term project, feel free to streamline this template or customize its features:

- **fhevmjs**: Fully Homomorphic Encryption for Ethereum Virtual Machine
- **React**: Modern UI framework for building interactive interfaces
- **Vite**: Next-generation frontend build tool
- **Wagmi**: React hooks for Ethereum
- **Tailwind**: Utility-first CSS framework for rapid UI development
- **@reown/appkit**: Comprehensive toolkit for Web3 authentication including social logins and multi-wallet support
- **@radix-ui**: Unstyled, accessible UI components for building high-quality design systems and web apps

**Prerequisites**
1. Deploy a **ConfidentialERC20** contract
2. Node.js (v20 or higher)
3. npm, yarn, or pnpm package manager
4. MetaMask or another Ethereum wallet

**In this tutorial series, you will learn to:**

1. Set up your React development environment
2. Connect the frontend with your deployed **ConfidentialERC20** contract
3. Understand and implement the building blocks of `fhevmjs`

## Tutorial Sections

1. [Environment Setup](./1-environment-setup.md)
   - Fork and clone the template
   - Install dependencies
   - Configure environment variables

2. [Project Structure](./2-project-structure.md)
   - Key components and features
   - Understanding the template architecture 
   - Available tools and libraries

3. [Understanding `fhevmjs`](./3-understanding-fhevmjs.md)
   - Handling encrypted transactions

4. [A few points on the wallet of choice](./4-connecting-to-web3.md)

Let's begin with the first section: [Environment Setup](./1-environment-setup.md)

