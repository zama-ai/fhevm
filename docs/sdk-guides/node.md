# Build with Node

This document provides instructions on how to build with `Node.js` using the `@fhevm/sdk` library.

## Install the library

First, you need to install the library:

```bash
# Using npm
npm install @fhevm/sdk

# Using Yarn
yarn add @fhevm/sdk

# Using pnpm
pnpm add @fhevm/sdk
```

`@fhevm/sdk` uses ESM format for web version and commonjs for node version. You need to set the
[type to "commonjs" in your package.json](https://nodejs.org/api/packages.html#type) to load the correct version of
@fhevm/sdk. If your node project use `"type": "module"`, you can force the loading of the Node version by using
`import { createInstance } from '@fhevm/sdk/node';`

## Create an instance

An instance receives an object containing:

- `chainId` (optional): the chainId of the network
- `network` (optional): the Eip1193 object provided by `window.ethereum` (used to fetch the public key and/or chain id)
- `networkUrl` (optional): the URL of the network (used to fetch the public key and/or chain id)
- `publicKey` (optional): if the public key has been fetched separately (cache), you can provide it
- `gatewayUrl` (optional): the URL of the gateway to retrieve a reencryption
- `coprocessorUrl` (optional): the URL of the coprocessor

```javascript
const { createInstance } = require("@fhevm/sdk");

const createFhevmInstance = async () => {
  return createInstance({
    chainId: 11155111, // Sepolia chain ID
    networkUrl: "https://eth-sepolia.public.blastapi.io", // Sepolia RPC URL
    gatewayUrl: "https://gateway.sepolia.zama.ai",
  });
};
createFhevmInstance().then((instance) => {
  console.log(instance);
});
```

You can now use your instance to [encrypt parameters](../smart_contracts/inputs.md) or do a
[reencryption](../smart_contracts/decryption/reencryption.md).
