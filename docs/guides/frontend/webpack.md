# Common webpack errors

This document provides solutions for common Webpack errors encountered during the development process. Follow the steps below to resolve each issue.

## Can't resolve 'tfhe_bg.wasm'

**Error message:** `Module not found: Error: Can't resolve 'tfhe_bg.wasm'`

**Cause:** In the codebase, there is a `new URL('tfhe_bg.wasm')` which triggers a resolve by Webpack.

**Possible sultions:** You can add a fallback for this file by adding a resolve configuration in your `webpack.config.js`:

```javascript
resolve: {
  fallback: {
    'tfhe_bg.wasm': require.resolve('tfhe/tfhe_bg.wasm'),
  },
},
```

## Buffer not defined

**Error message:** `ReferenceError: Buffer is not defined`

**Cause:** This error occurs when the Node.js `Buffer` object is used in a browser environment where it is not natively available.

**Possible sultions:** To resolve this issue, you need to provide browser-compatible fallbacks for Node.js core modules. Install the necessary browserified npm packages and configure Webpack to use these fallbacks.

```javascript
resolve: {
  fallback: {
    buffer: require.resolve('buffer/'),
    crypto: require.resolve('crypto-browserify'),
    stream: require.resolve('stream-browserify'),
    path: require.resolve('path-browserify'),
  },
},
```

## Issue with importing ESM version

**Error message:** Issues with importing ESM version

**Cause:** With a bundler such as Webpack or Rollup, imports will be replaced with the version mentioned in the `"browser"` field of the `package.json`. This can cause issues with typing.

**Possible solutions:**

1. If you encounter issues with typing, you can use this [tsconfig.json](https://github.com/zama-ai/fhevmjs-react-template/blob/main/tsconfig.json) using TypeScript 5.
2. If you encounter any other issue, you can force import of the browser package.

## Use bundled version

**Error message:** Issues with bundling the library, especially with SSR frameworks.

**Cause:** The library may not bundle correctly with certain frameworks, leading to errors during the build or runtime process.

**Possible solutions:** Use the prebundled version available in `fhevmjs/bundle`. Embed the library with a `<script>` tag and initialize it as shown below:

```javascript
const start = async () => {
  await window.fhevm.initFhevm(); // load wasm needed
  const instance = window.fhevm
    .createInstance({
      kmsContractAddress: "0x208De73316E44722e16f6dDFF40881A3e4F86104",
      aclContractAddress: "0xc9990FEfE0c27D31D0C2aa36196b085c0c4d456c",
      network: window.ethereum,
      gatewayUrl: "https://gateway.zama.ai/",
    })
    .then((instance) => {
      console.log(instance);
    });
};
```
