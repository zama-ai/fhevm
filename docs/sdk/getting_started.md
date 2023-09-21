# Getting Started

Welcome to the documentation for fhevmjs, a JavaScript library that enables interaction with blockchain using Zama's technology! This comprehensive guide provides developers with detailed information on encryption of data using TFHE (Fully Homomorphic Encryption over the Torus) and generation of EIP-719 tokens for reencrypt data.

## Installation

To get started with fhevmjs, you need to install it as a dependency in your JavaScript project. You can do this using npm, Yarn or pnpm. Open your terminal and navigate to your project's directory, then run one of the following commands:

```bash
# Using npm
npm install fhevmjs

# Using Yarn
yarn add fhevmjs

# Using pnpm
pnpm add fhevmjs
```

This will download and install the fhevmjs library and its dependencies into your project.

## Quick start

{% tabs %}
{% tab title="Node" %}
```javascript
const { createInstance } = require('fhevmjs');
createInstance({ chainId, publicKey }).then((instance) => {
  console.log(instance);
});
```
{% endtab %}

{% tab title="Browser" %}
To use the library in your project, you need to load WASM of [TFHE](https://www.npmjs.com/package/tfhe) first with `initFhevm`.

```javascript
import { BrowserProvider } from 'ethers';
import { initFhevm, createInstance } from 'fhevmjs';

const createFhevmInstance = async () => {
  const provider = new BrowserProvider(window.ethereum);
  const network = await provider.getNetwork();
  const chainId = +network.chainId.toString();
  const publicKey = await provider.call({
    from: null,
    to: '0x0000000000000000000000000000000000000044',
  });
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

You can take a look at [this template](https://github.com/zama-ai/fhevmjs-react-template) for an example using Vite, React and TypeScript.

### Troubleshooting

#### Webpack: "Module not found: Error: Can't resolve 'tfhe\_bg.wasm'"

In the codebase, there is a `new URL('tfhe_bg.wasm')` which triggers a resolve by Webpack. If yo u encounter an issue, you can add a fallback for this file by adding a resolve configuration in y our `webpack.config.js`:

```javascript
    resolve: {
      fallback: {
        'tfhe_bg.wasm': require.resolve('tfhe/tfhe_bg.wasm'),
      },
    },
```

#### Issue with importing ESM version

With a bundler such as Webpack or Rollup, imports will be replaced with the version mentioned in the `"browser"` field of the `package.json`. If you encounter issue with typing, you can use this [tsconfig.json](https://github.com/zama-ai/fhevmjs-react-template/blob/main/tsconfig.json) using TypeScript 5.

If you encounter any other issue, you can force import of the browser package.

```javascript
import { initFhevm, createInstance } from 'fhevmjs/web';
```

#### Use bundled version

If you have an issue with bundling the library (for example with some SSR framework), you can use the prebundled version available in `fhevmjs/bundle`. Just embed the library with a `<script>` tag and you're good to go.

```javascript
const start = async () => {
  await window.fhevm.initFhevm(); // load wasm needed
  const instance = window.fhevm
    .createInstance({ chainId, publicKey })
    .then((instance) => {
      console.log(instance);
    });
};
```
{% endtab %}
{% endtabs %}



