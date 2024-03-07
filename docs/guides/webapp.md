# Build a web application

## Using a template

`fhevmjs` is working out of the box and we recommend you to use it. We also provide three GitHub templates to start your project with everything set.

### React + TypeScript

You can use [this template](https://github.com/zama-ai/fhevmjs-react-template) to start an application with fhevmjs, using Vite + React + TypeScript.

### VueJS + TypeScript

You can also use [this template](https://github.com/zama-ai/fhevmjs-vue-template) to start an application with fhevmjs, using Vite + Vue + TypeScript.

### NextJS + Typescript

You can also use [this template](https://github.com/zama-ai/fhevmjs-next-template) to start an application with fhevmjs, using Next + TypeScript.

## Using directly the library

First, you need to install the library.

```bash
# Using npm
npm install fhevmjs

# Using Yarn
yarn add fhevmjs

# Using pnpm
pnpm add fhevmjs
```

`fhevmjs` uses ESM format. You need to set the [type to "module" in your package.json](https://nodejs.org/api/packages.html#type). If your node project use `"type": "commonjs"` or no type, you can force the loading of the web version by using `import { createInstance } from 'fhevmjs/web';`

To use the library in your project, you need to load the WASM of [TFHE](https://www.npmjs.com/package/tfhe) first with `initFhevm`.

```javascript
import { initFhevm } from "fhevmjs";

const init = async () => {
  await initFhevm(); // Load TFHE
};

init().then((instance) => {
  console.log(instance);
});
```

Once the WASM is loaded, you can now create an instance. An instance needs two element:

- The blockchain public key. This key is needed to encrypt inputs
- The blockchain' chain ID. This value is needed for reencryption process.

```javascript
import { ethers, BrowserProvider } from "ethers";
import { initFhevm, createInstance, getPublicKeyCallParams } from "fhevmjs";

const createFhevmInstance = async () => {
  const provider = new BrowserProvider(window.ethereum);
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

const init = async () => {
  await initFhevm(); // Load TFHE
  return createFhevmInstance();
};

init().then((instance) => {
  console.log(instance);
});
```

You can now use your instance to [encrypt parameters](./inputs.md) or do a [reencryption](./reencryption.md).
