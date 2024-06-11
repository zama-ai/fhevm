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

## Create an instance

```javascript
const { createInstance, getPublicKeyCallParams } = require("fhevmjs");
const { ethers, JsonRpcProvider } = require("ethers");

const provider = new JsonRpcProvider(`https://devnet.zama.ai/`);

const createFhevmInstance = async () => {
  // 1. Get the chain id
  const network = await provider.getNetwork();
  const chainId = +network.chainId.toString();
  // 2. Fetch the FHE public key from the blockchain
  const ret = await provider.call(getPublicKeyCallParams());
  const decoded = ethers.AbiCoder.defaultAbiCoder().decode(["bytes"], ret);
  const publicKey = decoded[0];

  // 3. Create the instance
  return createInstance({ chainId, publicKey });
};
createFhevmInstance().then((instance) => {
  console.log(instance);
});
```

You can now use your instance to [encrypt parameters](./inputs.md) or do a [reencryption](./reencryption.md).
