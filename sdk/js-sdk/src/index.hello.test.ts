import { describe, it } from "node:test";
// import { readFile } from "node:fs/promises";
// import { resolve } from "node:path";
import { threads } from "wasm-feature-detect";
import { terminateWorkers } from "./wasm/tfhe/tfhe.v1.5.3.js";
// NOT OK
import {
  createFhevmDecryptClient,
  createFhevmEncryptClient,
  setFhevmRuntimeConfig,
} from "./ethers/index.js";
import { createFhevm } from "./ethers/clients/createFhevm.js";
import { sepolia } from "./core/chains/index.js";
import { ethers } from "ethers";
import { decryptModule } from "./core/modules/decrypt/module/index.js";
import {
  createFhevmDecryptionKey,
  type FhevmDecryptionKey,
} from "./core/user/FhevmDecryptionKey-p.js";

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

function setConfig(numThreads: number) {
  setFhevmRuntimeConfig({
    numberOfThreads: numThreads,
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

describe("hello", () => {
  it("should say hello", async () => {
    try {
      const supportsThreads = await threads();
      console.log("threads supported:", supportsThreads);

      setConfig(20);
      const fhevm = createFhevm();
      const fhevmDecryptClient = createFhevmDecryptClient({
        chain: sepolia,
        provider: new ethers.JsonRpcProvider(
          "https://ethereum-sepolia-rpc.publicnode.com",
        ),
      });

      const r = fhevm.runtime.extend(decryptModule);
      const pk = await r.decrypt.generateTkmsPrivateKey();
      const pkBytes = await r.decrypt.serializeTkmsPrivateKey({
        tkmsPrivateKey: pk,
      });

      const decryptionKey: FhevmDecryptionKey = await createFhevmDecryptionKey(
        r,
        { tkmsPrivateKey: pkBytes },
      );

      const eip712 = fhevmDecryptClient.createUserDecryptEIP712({
        contractAddresses: ["0x1E7eA8fE4877E6ea5dc8856f0dA92da8d5066241"],
        durationDays: 356,
        startTimestamp: timestampNow(),
        extraData: "0x00",
        publicKey: await decryptionKey.getTkmsPublicKeyHex(),
      });

      const fhevmEncryptClient = createFhevmEncryptClient({
        chain: sepolia,
        provider: new ethers.JsonRpcProvider(
          "https://ethereum-sepolia-rpc.publicnode.com",
        ),
      });

      const globalFhePkeParams =
        await fhevmEncryptClient.fetchGlobalFhePkeParams();

      console.log(fhevm.uid);
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
