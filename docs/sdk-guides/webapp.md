# Build a web application

This document guides you through building a web application using the @fhevm/sdk library. You can either start with a template or directly integrate the library into your project.

## Using a template

`@fhevm/sdk` is working out of the box and we recommend you to use it. We also provide three GitHub templates to start your project with everything set.

### React + TypeScript

You can use [this template](https://github.com/zama-ai/fhevmjs-react-template) to start an application with @fhevm/sdk, using Vite + React + TypeScript.

### NextJS + Typescript

You can also use [this template](https://github.com/zama-ai/fhevmjs-next-template) to start an application with @fhevm/sdk, using Next + TypeScript.

## Using the mocked coprocessor for frontend

As an alternative to use the real coprocessor deployed on Sepolia to help you develop your dApp faster and without needing testnet tokens, you can use a mocked fhevm. Currently, we recommend you to use the `ConfidentialERC20` dApp example available on the `mockedFrontend` branch of the [React template](https://github.com/zama-ai/fhevm-react-template/tree/mockedFrontend). Follow the README on this branch, and you will be able to deploy exactly the same dApp both on Sepolia as well as on the mocked coprocessor seamlessly.

## Using directly the library

### Step 1: Setup the library

`@fhevm/sdk` consists of multiple files, including WASM files and WebWorkers, which can make packaging these components correctly in your setup cumbersome. To simplify this process, especially if you're developing a dApp with server-side rendering (SSR), we recommend using our CDN.

#### Using UMD CDN

Include this line at the top of your project.

```html
<script src="https://cdn.zama.ai/fhevmjs/0.6.2/fhevmjs.umd.cjs" type="text/javascript"></script>
```

In your project, you can use the bundle import if you install `@fhevm/sdk` package:

```javascript
import { initFhevm, createInstance } from "@fhevm/sdk/bundle";
```

#### Using ESM CDN

If you prefer You can also use the `@fhevm/sdk` as a ES module:

```html
<script type="module">
  import { initFhevm, createInstance } from "https://cdn.zama.ai/fhevmjs/0.6.2/fhevmjs.js";

  await initFhevm();
  const instance = await createInstance({
    network: window.ethereum,
    kmsContractAddress: "0x9D6891A6240D6130c54ae243d8005063D05fE14b",
    aclContractAddress: "0xFee8407e2f5e3Ee68ad77cAE98c434e637f516e5",
    gatewayUrl: "https://gateway.sepolia.zama.ai",
  });
</script>
```

#### Using npm package

Install the `@fhevm/sdk` library to your project:

```bash
# Using npm
npm install @fhevm/sdk

# Using Yarn
yarn add @fhevm/sdk

# Using pnpm
pnpm add @fhevm/sdk
```

`@fhevm/sdk` uses ESM format. You need to set the [type to "module" in your package.json](https://nodejs.org/api/packages.html#type). If your node project use `"type": "commonjs"` or no type, you can force the loading of the web version by using `import { createInstance } from '@fhevm/sdk/web';`

```javascript
import { initFhevm, createInstance } from "@fhevm/sdk";
```

### Step 2: Initialize your project

To use the library in your project, you need to load the WASM of [TFHE](https://www.npmjs.com/package/tfhe) first with `initFhevm`.

```javascript
import { initFhevm } from "@fhevm/sdk/bundle";

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
import { initFhevm, createInstance } from "@fhevm/sdk/bundle";

const init = async () => {
  await initFhevm(); // Load FHE
  return createInstance({
    kmsContractAddress: "0x9D6891A6240D6130c54ae243d8005063D05fE14b",
    aclContractAddress: "0xFee8407e2f5e3Ee68ad77cAE98c434e637f516e5",
    network: window.ethereum,
    gatewayUrl: "https://gateway.sepolia.zama.ai/",
  });
};

init().then((instance) => {
  console.log(instance);
});
```

You can now use your instance to [encrypt parameters](../smart_contracts/inputs.md) or do a [reencryption](../smart_contracts/decryption/reencryption.md).
