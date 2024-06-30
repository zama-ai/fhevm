# Build with Node

First, you need to install the library.

```bash
# Using npm
npm install fhevmjs

# Using Yarn
yarn add fhevmjs

# Using pnpm
pnpm add fhevmjs
```

`fhevmjs` uses ESM format for web version and commonjs for node version. You need to set the [type to "commonjs" in your package.json](https://nodejs.org/api/packages.html#type) to load the correct version of fhevmjs. If your node project use `"type": "module"`, you can force the loading of the Node version by using `import { createInstance } from 'fhevmjs/node';`

An instance receives an object containing:

- `chainId` (optional): the chainId of the network
- `network` (optional): the Eip1193 object provided by `window.ethereum` (used to fetch the public key and/or chain id)
- `networkUrl` (optional): the URL of the network (used to fetch the public key and/or chain id)
- `publicKey` (optional): if the public key has been fetched separately (cache), you can provide it
- `gatewayUrl` (optional): the URL of the gateway to retrieve a reencryption
- `coprocessorUrl` (optional): the URL of the coprocessor

## Create an instance

```javascript
const { createInstance } = require("fhevmjs");

const createFhevmInstance = async () => {
  return createInstance({
    chainId: 8009,
    networkUrl: "https://devnet.zama.ai/",
    gatewayUrl: "https://gateway.zama.ai",
  });
};
createFhevmInstance().then((instance) => {
  console.log(instance);
});
```

You can now use your instance to [encrypt parameters](../fundamentals/inputs.md) or do a [reencryption](./reencryption.md).
