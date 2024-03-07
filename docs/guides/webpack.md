# Common webpack errors

## "Module not found: Error: Can't resolve 'tfhe_bg.wasm'"

In the codebase, there is a `new URL('tfhe_bg.wasm')` which triggers a resolve by Webpack. If you encounter an issue, you can add a fallback for this file by adding a resolve configuration in your `webpack.config.js`:

```javascript
resolve: {
  fallback: {
    'tfhe_bg.wasm': require.resolve('tfhe/tfhe_bg.wasm'),
  },
},
```

## ReferenceError: Buffer is not defined

If you encounter this issue with the Node Buffer object, you should offer an alternative solution. Similar issues might arise with different Node objects.
In such cases, install the corresponding browserified npm package and include the fallback as follows.

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

With a bundler such as Webpack or Rollup, imports will be replaced with the version mentioned in the `"browser"` field of the `package.json`. If you encounter issue with typing, you can use this [tsconfig.json](https://github.com/zama-ai/fhevmjs-react-template/blob/main/tsconfig.json) using TypeScript 5.

If you encounter any other issue, you can force import of the browser package.

```javascript
import { initFhevm, createInstance } from "fhevmjs/web";
```

## Use bundled version

If you have an issue with bundling the library (for example with some SSR framework), you can use the prebundled version available in `fhevmjs/bundle`. Just embed the library with a `<script>` tag and you're good to go.

```javascript
const start = async () => {
  await window.fhevm.initFhevm(); // load wasm needed
  const instance = window.fhevm.createInstance({ chainId, publicKey }).then((instance) => {
    console.log(instance);
  });
};
```
