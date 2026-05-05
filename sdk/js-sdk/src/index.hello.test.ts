import { describe, it } from 'node:test';
// import { readFile } from "node:fs/promises";
// import { resolve } from "node:path";
import { threads } from 'wasm-feature-detect';
import { terminateWorkers } from './wasm/tfhe/tfhe.v1.5.3.js';
// NOT OK
import {
  createFhevmClient,
  createFhevmDecryptClient,
  createFhevmEncryptClient,
  setFhevmRuntimeConfig,
} from './ethers/index.js';
import { sepolia } from './core/chains/index.js';
import { ethers } from 'ethers';
//import { safeJSONstringify } from "./core/base/string.js";
import { decryptActions } from './ethers/decorators/decrypt.js';
// import type {
//   SignedDelegatedDecryptionPermit,
//   SignedSelfDecryptionPermit,
// } from "./core/types/signedDecryptionPermit.js";
import { createFhevmBaseClient } from './ethers/clients/createFhevmBaseClient.js';

// node --test --import tsx ./src/index.hello.test.ts

export function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export function timestampNow(): number {
  return Math.floor(Date.now() / 1000);
}

// async function getDirname(): Promise<string> {
//   const { fileURLToPath } = await import("node:url");
//   const { dirname } = await import("node:path");
//   return dirname(fileURLToPath(import.meta.url));
// }

// async function preloadTfheUsingWorker2(numThreads: number) {
//   await setTfheModuleConfig({
//     numberOfThreads: numThreads,
//     // locateFile: (_file: string): URL => {
//     //   return undefined as unknown as URL;
//     // },
//     logger: {
//       debug: (message: string) => {
//         console.log(message);
//       },
//       error: (message: string, cause: unknown) => {
//         console.log(message);
//         if (cause !== undefined) {
//           console.log(cause);
//         }
//       },
//     },
//   });
//   await initTfheModule();
// }

/**
 * Configures the FHEVM runtime with the specified number of threads and a console logger.
 * @param numThreads - The number of threads to use. Pass `0` for single-threaded mode.
 */
function setConfig(numThreads: number) {
  const singleThread = numThreads === 0;
  setFhevmRuntimeConfig({
    numberOfThreads: numThreads,
    singleThread,
    // Uncomment to use base64
    // locateFile: (_file: string): URL => {
    //   return undefined as unknown as URL;
    // },
    logger: {
      debug: (message: string) => {
        console.log(message);
      },
      error: (message: string, cause: unknown) => {
        console.log(message);
        if (cause !== undefined) {
          console.log(cause);
        }
      },
    },
  });
}

describe('hello', () => {
  it('should say hello', async () => {
    try {
      const supportsThreads = await threads();
      console.log('threads supported:', supportsThreads);

      // A minimum config is required prior to any SDK interactions
      // - setup the number of threads (0 === single threaded)
      // - setup the urls to the tkms + tfhe wasms
      //   If urls are left blank, the SDK will try to load local files or base64 embedded js
      setConfig(20);

      // Create a full client (with encryption and decryption features)
      const fhevmFullClient = createFhevmClient({
        chain: sepolia,
        provider: new ethers.JsonRpcProvider('https://ethereum-sepolia-rpc.publicnode.com'),
      });

      // Initialize the full client
      // 1. init tkms.wasm (lightweight)
      // 2. init tfhe.wasm (heavyweight)
      // 3. download+init tfhe global pub key (50MB)
      await fhevmFullClient.ready;

      const fhevmBaseClient = createFhevmBaseClient({
        chain: sepolia,
        provider: new ethers.JsonRpcProvider('https://ethereum-sepolia-rpc.publicnode.com'),
      });
      await fhevmBaseClient.ready;
      // Naming: GlobalEncryptionKey, FheEncryptionKey, PublicEncryptionKey, EncryptionKey
      // it is a public key
      // everybody can access it
      // with this key anyone can encrypt a value in its FHE encrypted form
      // it is 50MB
      // you need the relayer to get url of this key
      // internally: it is a 2 components structure: crs (big) + publicKey (small)

      // Get the FheEncru
      // fhevmBaseClient.fetchFheEncryptionKey;

      const fheEncryptionKeyBytes = await fhevmFullClient.fetchFheEncryptionKeyBytes();

      const fhevmBaseClient2 = createFhevmBaseClient({
        chain: fhevmBaseClient.chain,
        provider: fhevmBaseClient.client,
        options: {
          fheEncryptionKey: fheEncryptionKeyBytes,
        },
      });

      await fhevmBaseClient2.fetchFheEncryptionKeyBytes();

      // Let's create a partial decrypt client
      // Only using the lightweight tkms.wasm
      const fhevmDecryptClient = createFhevmDecryptClient({
        chain: sepolia,
        provider: new ethers.JsonRpcProvider('https://ethereum-sepolia-rpc.publicnode.com'),
      });

      // since the full client has already been initialized, the new partial client will be instantly initialized
      // They are sharing the same runtime modules
      await fhevmDecryptClient.ready;

      // Let's generate a simple kms private decryption key
      // const transportKeypair =
      //   await fhevmDecryptClient.generateE2eTransportKeypair();

      // const dummySigner: ethers.Signer =
      //   transportKeypair as unknown as ethers.Signer;

      // SignedSelfDecryptionPermit
      // const signedPermit: SignedSelfDecryptionPermit =
      //   await fhevmDecryptClient.signDecryptionPermit({
      //     transportKeypair,
      //     contractAddresses: ["0x1E7eA8fE4877E6ea5dc8856f0dA92da8d5066241"],
      //     durationDays: 356,
      //     startTimestamp: timestampNow(),
      //     signerAddress: "0x37ac010c1c566696326813b840319b58bb5840e4",
      //     signer: dummySigner,
      //   });

      // // SignedDelegatedDecryptionPermit
      // const signedDelegatePermit: SignedDelegatedDecryptionPermit =
      //   await fhevmDecryptClient.signDecryptionPermit({
      //     transportKeypair,
      //     contractAddresses: ["0x1E7eA8fE4877E6ea5dc8856f0dA92da8d5066241"],
      //     durationDays: 356,
      //     startTimestamp: timestampNow(),
      //     delegatorAddress: "0x37ac010c1c566696326813b840319b58bb5840e4",
      //     signerAddress: "0x37ac010c1c566696326813b840319b58bb5840e4",
      //     signer: dummySigner,
      //   });

      // const clearText = await fhevmDecryptClient.decrypt({
      //   encryptedValues: [],
      //   transportKeypair,
      //   signedPermit,
      // });

      // const clearText2 = await fhevmDecryptClient.decrypt({
      //   encryptedValues: [],
      //   transportKeypair,
      //   signedPermit: signedDelegatePermit,
      // });

      // // Let's test the creation of a simple EIP712
      // const eip712 = fhevmDecryptClient.createUserDecryptEIP712({
      //   contractAddresses: ["0x1E7eA8fE4877E6ea5dc8856f0dA92da8d5066241"],
      //   durationDays: 356,
      //   startTimestamp: timestampNow(),
      //   extraData: "0x00",
      //   publicKey: await privateDecryptionKey.getTkmsPublicKeyHex(),
      // });
      // console.log(safeJSONstringify(eip712, 2));

      // Let's create a new encrypt client (cost is zero, modules and keys are globally shared)
      const fhevmEncryptClient = createFhevmEncryptClient({
        chain: sepolia,
        provider: new ethers.JsonRpcProvider('https://ethereum-sepolia-rpc.publicnode.com'),
      });

      // Let's xform the encryptClient by extending it with decryption features
      const encryptClientWithDecryptFeatures = fhevmEncryptClient.extend(decryptActions);

      // call init() - instant (modules are already initialized)
      await encryptClientWithDecryptFeatures.init();
      // or await on ready property - instant (modules are already initialized)
      // it's a matter of taste
      await encryptClientWithDecryptFeatures.ready;

      // const verifiedInputProof = await fhevmEncryptClient.encrypt({
      //   contractAddress: "0x1E7eA8fE4877E6ea5dc8856f0dA92da8d5066241",
      //   extraData: "0x00",
      //   userAddress: "0x37ac010c1c566696326813b840319b58bb5840e4",
      //   values: {
      //     type: "uint16",
      //     value: 123,
      //   },
      //   options: {
      //     debug: true, // enable relayer internal traces
      //     onProgress: (args) => {
      //       console.log(
      //         `[${args.operation}] jobId=${args.jobId} retry=${args.retryCount}`,
      //       );
      //     },
      //   },
      // });

      // console.log(verifiedInputProof.inputProof);
      // console.log(verifiedInputProof.externalEncryptedValue);

      console.log(fhevmDecryptClient.uid);

      //   console.log(fhevmEncryptClient.uid);
      //   await fhevmEncryptClient.runtime.encrypt.initTfheModule();

      //   await fhevmDecryptClient.userDecrypt({
      //     decryptionKey,
      //     handleContractPairs: [
      //       {
      //         handle: toFhevmHandle(
      //           "0x35e5fd5f40571c8b53b6136711893b61509f3e9490ff0000000000aa36a70400",
      //         ),
      //         contractAddress: asChecksummedAddress(
      //           "0x1E7eA8fE4877E6ea5dc8856f0dA92da8d5066241",
      //         ),
      //       },
      //     ],
      //     userDecryptEIP712Message: eip712.message,
      //     userDecryptEIP712Signer:
      //       "0x37ac010c1c566696326813b840319b58bb5840e4" as ChecksummedAddress,
      //     userDecryptEIP712Signature: "0x" as Bytes65Hex,
      //   });

      /*
      const handle = toFhevmHandle(
        "0x35e5fd5f40571c8b53b6136711893b61509f3e9490ff0000000000aa36a70400",
      );

      const result = await fhevmDecryptClient.publicDecrypt({
        handles: [handle],
        extraData: asBytesHex("0x00"),
      });

      console.log(result.orderedDecryptedHandles[0]?.value);
      */

      // Testnet public handles:
      // =======================
      // 0xf1673094de7c833604f1b62183cbcdf2cdc968db90ff0000000000aa36a70400 euint32 1083783185
      // 0x9797f8eb707b0a32c47a80ea86c0648df36bfe7cd0ff0000000000aa36a70300 euint16 15764
      // 0x6f17228bda73a5e57b94511c5bab2665e6a2870399ff0000000000aa36a70200 euint8 171
      // 0x821c6ef4218b335278214b00b1ad41757c7bc644ffff0000000000aa36a70500 euint64 12168711736151452489
      // 0x9d430a3e950560ba22013ce885d6d90f0da36efdf1ff0000000000aa36a70600 euint128 308429577281045301472547520724787086512
      // 0xf6751d547a5c06123575aad93f22f76b7d841c4cacff0000000000aa36a70000 ebool false
      //
      // npx . test public-decrypt --types euint32 --network testnet --version 1
      // npx . test public-decrypt --types euint32 --network testnet --version 2
      // npx . test public-decrypt --types euint32 --network mainnet --version 2

      //await fhevmDecryptClient.runtime.decrypt.initTkmsModule();

      //const p = preloadTfheBase64FetchBlobThen(20);
      //const p = preloadTfheBase64FetchBlobAwait(20);
      //const p = preloadTfheUsingWorker2(20);

      for (let i = 0; i < 2; ++i) {
        // why is p completed even before the first iteration ?
        console.log(`sleep ${i + 1}`);
        await sleep(1000);
      }

      const t0 = Date.now();
      // const wasmBytes = await readWasmBytes();
      // await p;
      const t1 = Date.now();

      console.log(`preloadTfheBase64Atob Time=${t1 - t0}ms`);

      // console.log("wasmBytes=" + wasmBytes.length);
      // console.log("wasmBytes=" + wasmBytes.length);

      // await initTFHE({ module_or_path: wasmBytes });

      // console.log("init_panic_hook...");
      // init_panic_hook();

      // const numThreads = 20;
      // await initThreadPool(numThreads);
    } catch (e) {
      console.log(e);
    }
    await terminateWorkers();
  });
});
