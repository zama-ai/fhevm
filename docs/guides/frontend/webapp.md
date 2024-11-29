# Build a web application

This document guides you through building a web application using the fhevmjs library. You can either start with a template or directly integrate the library into your project.

## Using a template

`fhevmjs` is working out of the box and we recommend you to use it. We also provide three GitHub templates to start your project with everything set.

### React + TypeScript

You can use [this template](https://github.com/zama-ai/fhevmjs-react-template) to start an application with fhevmjs, using Vite + React + TypeScript.

### VueJS + TypeScript

You can also use [this template](https://github.com/zama-ai/fhevmjs-vue-template) to start an application with fhevmjs, using Vite + Vue + TypeScript.

### NextJS + Typescript

You can also use [this template](https://github.com/zama-ai/fhevmjs-next-template) to start an application with fhevmjs, using Next + TypeScript.

## Using directly the library

### Step 1: Install the library

Install the `fhevmjs` library to your project:

```bash
# Using npm
npm install fhevmjs

# Using Yarn
yarn add fhevmjs

# Using pnpm
pnpm add fhevmjs
```

### Step 2: Initialize your project

`fhevmjs` uses ESM format. You need to set the [type to "module" in your package.json](https://nodejs.org/api/packages.html#type). If your node project use `"type": "commonjs"` or no type, you can force the loading of the web version by using `import { createInstance } from 'fhevmjs/web';`

To use the library in your project, you need to load the WASM of [TFHE](https://www.npmjs.com/package/tfhe) first with `initFhevm`.

```javascript
import { initFhevm } from "fhevmjs";

const init = async () => {
  await initFhevm(); // Load needed WASM
};
```

### Step 3: Create an instance

Once the WASM is loaded, you can now create an instance. An instance receives an object containing:

- `chainId` (optional): the chainId of the network
- `network` (optional): the Eip1193 object provided by `window.ethereum` (used to fetch the public key and/or chain id)
- `networkUrl` (optional): the URL of the network (used to fetch the public key and/or chain id)
- `publicKey` (optional): if the public key has been fetched separately (cache), you can provide it
- `gatewayUrl` (optional): the URL of the gateway to retrieve a reencryption
- `coprocessorUrl` (optional): the URL of the coprocessor

```javascript
import { initFhevm, createInstance } from "fhevmjs";

const init = async () => {
  await initFhevm(); // Load TFHE
  return createInstance({
    kmsContractAddress: "0x208De73316E44722e16f6dDFF40881A3e4F86104",
    aclContractAddress: "0xc9990FEfE0c27D31D0C2aa36196b085c0c4d456c",
    network: window.ethereum,
    gatewayUrl: "https://gateway.zama.ai/",
  });
};

init().then((instance) => {
  console.log(instance);
});
```

You can now use your instance to [encrypt parameters](../../fundamentals/first_step/inputs.md) or do a [reencryption](../../fundamentals/first_step/reencryption.md).
