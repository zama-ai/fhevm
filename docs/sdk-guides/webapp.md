# Build a web application

This document guides you through building a web application using the `@zama-fhe/relayer-sdk` library.

<!-- NOTE: uncomment once templates are updated to latest testnet -->

<!-- You can either start with a template or directly integrate the library into your project. -->
<!-- ## Using a template -->
<!---->
<!-- `@zama-fhe/relayer-sdk` is working out of the box and we recommend you to use it. We also provide three GitHub templates to start your project with everything set. -->
<!---->
<!-- ### React + TypeScript -->
<!---->
<!-- You can use [this template](https://github.com/zama-ai/fhevmjs-react-template) to start an application with @zama-fhe/relayer-sdk, using Vite + React + TypeScript. -->
<!---->
<!-- ### NextJS + Typescript -->
<!---->
<!-- You can also use [this template](https://github.com/zama-ai/fhevmjs-next-template) to start an application with @zama-fhe/relayer-sdk, using Next + TypeScript. -->
<!---->
<!-- ## Using the mocked coprocessor for frontend -->
<!---->
<!-- As an alternative to use the real coprocessor deployed on Sepolia to help you develop your dApp faster and without needing testnet tokens, you can use a mocked FHEVM. Currently, we recommend you to use the `ConfidentialERC20` dApp example available on the `mockedFrontend` branch of the [React template](https://github.com/zama-ai/fhevm-react-template/tree/mockedFrontend). Follow the README on this branch, and you will be able to deploy exactly the same dApp both on Sepolia as well as on the mocked coprocessor seamlessly. -->
<!---->

## Using directly the library

### Step 1: Setup the library

`@zama-fhe/relayer-sdk` consists of multiple files, including WASM files and WebWorkers, which can make packaging these components correctly in your setup cumbersome. To simplify this process, especially if you're developing a dApp with server-side rendering (SSR), we recommend using our CDN.

#### Using UMD CDN

Include this line at the top of your project.

```html
<script src="https://cdn.zama.ai/relayer-sdk-js/0.1.0-9/relayer-sdk-js.umd.cjs" type="text/javascript"></script>
```

In your project, you can use the bundle import if you install `@zama-fhe/relayer-sdk` package:

```javascript
import { initSDK, createInstance, SepoliaConfig } from "@zama-fhe/relayer-sdk/bundle";
```

#### Using ESM CDN

If you prefer You can also use the `@zama-fhe/relayer-sdk` as a ES module:

```html
<script type="module">
  import { initSDK, createInstance, SepoliaConfig } from "https://cdn.zama.ai/relayer-sdk-js/0.1.0-9/relayer-sdk-js.js";

  await initSDK();
  const config = { ...SepoliaConfig, network: window.ethereum };
  config.network = window.ethereum;
  const instance = await createInstance(config);
</script>
```

#### Using npm package

Install the `@zama-fhe/relayer-sdk` library to your project:

```bash
# Using npm
npm install @zama-fhe/relayer-sdk

# Using Yarn
yarn add @zama-fhe/relayer-sdk

# Using pnpm
pnpm add @zama-fhe/relayer-sdk
```

`@zama-fhe/relayer-sdk` uses ESM format. You need to set the [type to "module" in your package.json](https://nodejs.org/api/packages.html#type). If your node project use `"type": "commonjs"` or no type, you can force the loading of the web version by using `import { createInstance } from '@zama-fhe/relayer-sdk/web';`

```javascript
import { initSDK, createInstance, SepoliaConfig } from "@zama-fhe/relayer-sdk";
```

### Step 2: Initialize your project

To use the library in your project, you need to load the WASM of [TFHE](https://www.npmjs.com/package/tfhe) first with `initSDK`.

```javascript
import { initSDK } from "@zama-fhe/relayer-sdk/bundle";

const init = async () => {
  await initSDK(); // Load needed WASM
};
```

### Step 3: Create an instance

Once the WASM is loaded, you can now create an instance.

```javascript
import { initSDK, createInstance, SepoliaConfig } from "@zama-fhe/relayer-sdk/bundle";

const init = async () => {
  await initSDK(); // Load FHE
  const config = { ...SepoliaConfig, network: window.ethereum };
  return createInstance(config);
};

init().then((instance) => {
  console.log(instance);
});
```

You can now use your instance to [encrypt parameters](./input.md), perform [user decryptions](./user-decryption.md) or [public decryptions](./public-decryption.md).
