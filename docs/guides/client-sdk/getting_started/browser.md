# Browser

## Usage

`fhevmjs` uses ESM format. You need to set the [type to "module" in your package.json](https://nodejs.org/api/packages.html#type).

To use the library in your project, you need to load the WASM of [TFHE](https://www.npmjs.com/package/tfhe) first with `initFhevm`.

```javascript
import { BrowserProvider } from "ethers";
import { initFhevm, createInstance } from "fhevmjs";

const createFhevmInstance = async () => {
  const provider = new BrowserProvider(window.ethereum);
  const network = await provider.getNetwork();
  const chainId = +network.chainId.toString();
  const ret = await provider.call({
    // fhe lib address, may need to be changed depending on network
    to: "0x000000000000000000000000000000000000005d",
    // first four bytes of keccak256('fhePubKey(bytes1)') + 1 byte for library
    data: "0xd9d47bb001",
  });
  const decoded = ethers.AbiCoder.defaultAbiCoder().decode(["bytes"], ret);
  const publicKey = decoded[0];
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

## Webpack

### "Module not found: Error: Can't resolve 'tfhe_bg.wasm'"

In the codebase, there is a `new URL('tfhe_bg.wasm')` which triggers a resolve by Webpack. If you encounter an issue, you can add a fallback for this file by adding a resolve configuration in your `webpack.config.js`:

```javascript
resolve: {
  fallback: {
    'tfhe_bg.wasm': require.resolve('tfhe/tfhe_bg.wasm'),
  },
},
```

### ReferenceError: Buffer is not defined

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

### Issue with importing ESM version

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
