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

Once the WASM is loaded, you can now create an instance. An instance receives an object containing:

- `chainId`: the chainId of the network
- `networkUrl` (optional): the URL of the network (used to fetch the public key)
- `publicKey` (optional): if the public key has been fetched separately (cache), you can provide it
- `gatewayUrl` (optional): the URL of the gateway to retrieve a reencryption

```javascript
import { initFhevm, createInstance } from "fhevmjs";

const createFhevmInstance = async () => {
  return createInstance({
    chainId: 8009,
    networkUrl: "https://devnet.zama.ai/",
    gatewayUrl: "https://gateway.zama.ai",
  });
};

const init = async () => {
  await initFhevm(); // Load TFHE
  return createFhevmInstance();
};

init().then((instance) => {
  console.log(instance);
});
```

You can now use your instance to [encrypt parameters](../fundamentals/inputs.md) or do a [reencryption](./reencryption.md).
