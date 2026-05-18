- JSDoc on all public fn in API: decrypt/encrypt/base modules
- Cleanup public API modules
- in test : call a non-public API fn -> something more advanced
- OK implement readCoprocessorSignersContext cache
- missing multiple overloads for decrypt following the encrypt pattern
- cause error if call with an empty array : decrypt(encrvalyues:[])
- OK: remove FheEncryptionKey from action parameters
- OK: decrypt return type should have ClearValues
- OK: encrypt return type should have ExternalHandles
- OK: FheEncryptionKeyBytes as arg of createFhevmClient
- OK: throw new Error("assertIsInputHandleLike Not yet implemented");
- Export TypedValue (needed in encrypt)
- Export PublicDecryptionProof (needed in publicDecrypt)
- decrypt should accept string addresses not Checksummed addresses
- Add a skill that controls the input values of each api functions. They must be loose.
- security: verify that the relayer is never trusted. Meaning the sdk should never use the relayer responses but always verify the relayer responses first before using them (check signatures for example)
- Must call close threads for tests: example node-ethers example
- missing toEncryptedValues toExternalEncryptedValue ?
- Critical: verify domain hash on-chain to make sure EIP712 are correct
- Missing a README / ARCHITECTURE.md
- uid to rewrite
- 32-bits -> 32-bit
- confusing UnsignedIntNumber UnsignedInt UintNumber ...
- 120 columns
- logger.error not used
- logger global ?
- fix fheType array ascii art
- remove E2e in E2eTransportKeyPair
- OK EIP712 -> Eip712 etc.
- ClearValueTypeMap is unused!
- handle : euint8
- verifyKmsPublicDecryptEIP712 should use createKmsPublicDecryptEIP712
- handle = Bytes32Hex (0x....)
- object { } FhevmHandleImpl implement
- isAllowedForDecryption => canReadPublicValue(h), readPublicValue(h) -> TypedValue | TypedValue[]
- new handle format: https://github.com/zama-ai/fhevm/pull/2014
- fix: const originToken = Symbol('readPublicValues');
- Missing export RelayerPublicDecryptOptionsType etc...
- Missing export Fhevm<> types
- authorize multiple calls (no-op) to setFhevmRuntime (React)
- initFhevmEncryptRuntime / initFhevmDecryptRuntime / initFhevmRuntime
- remove userDecryptModule
- rename RelayerPublicDecryptOptions etc. does not match the API names
- MAX_INPUT_INDEX = 254 (see https://github.com/zama-ai/fhevm/blob/main/coprocessor/fhevm-engine/zkproof-worker/src/verifier.rs#L40)
- do not export private types like: SignDecryptionPermitContext
- missing type FhevmClient in exports
- missing TransportKeyPair export (generateTransportKeyPair)
- Fix peerDependency issue: test-suite/e2e: conflict with ethers, see:

```ts
type FhevmClient = ReturnType<typeof createFhevmClient>;
type CreateFhevmClientParameters = Parameters<typeof createFhevmClient>[0];
type FhevmClientProvider = CreateFhevmClientParameters['provider'];
```

- Verify async race conditions issues, following init-p.ts encrypt module init bug:

```ts
let resolvedTfheModuleConfig: ResolvedTfheModuleConfig | undefined = undefined;
let resolvingTfheModuleConfigPromise: Promise<ResolvedTfheModuleConfig> | undefined;

/**
 * @internal
 * Returns the existing resolved config, or resolves it from the runtime config.
 */
async function _getOrResolveTfheModuleConfig(runtime: FhevmRuntime): Promise<ResolvedTfheModuleConfig> {
  if (resolvedTfheModuleConfig !== undefined) {
    return resolvedTfheModuleConfig;
  }

  resolvingTfheModuleConfigPromise ??= _resolveTfheModuleConfig(runtime.config)
    .then((cfg) => {
      resolvedTfheModuleConfig = cfg;
      return cfg;
    })
    .catch((error: unknown) => {
      resolvingTfheModuleConfigPromise = undefined;
      throw error;
    });

  return resolvingTfheModuleConfigPromise;
}
```

```ts
    function checkSignatures(
        bytes32[] memory handlesList,
        bytes memory abiEncodedCleartexts,
        bytes memory decryptionProof
    ) internal;


    function run() {
      ...
      handle1= FHE.add();
      map[contractAddress] = handle1
      emit (please decrypt handle1)
    }

    function callbackDec(h, bytes memory abiEncodedCleartexts, bytes memory decryptionProof) {
      uint32 cv = abi.decode(abiEncodedCleartexts)[0];

      // verif
      FHE.checkSignatures([h], abiEncodedCleartexts, decryptionProof);

      transferDollars(cv, target);
    }

    {}
    npm install @zama-fhe/relayer-sdk

    npm run build
    cd js-sdk/src
    npm pack

    {
      dependencies: {
        "@fhevm/sdk": file.tgz
      }
    }
    index.ts import { create} from "@fhevm/sdk"


```

```ts

-- 4 functions --
const res: boolean = await client.canReadPublicValue({ handles });
const res: boolean[] = await client.canReadPublicValues({ handles });

const res: TypedValue = await client.readPublicValue({ handle });
const res: TypedValue[] = await client.readPublicValues({ handles });

-- or 2 functions --
const res: boolean = await client.canReadPublicValue({ handles });
const res: boolean[] = await client.canReadPublicValue({ handles });

const res: TypedValue = await client.readPublicValue({ handle });
const res: TypedValue[] = await client.readPublicValue({ handles });



const res = await client.readPublicValuesWithSignatures({ handles });
res.clearValues
res.checkSignaturesArgs {
  handleList,
  abiEncodedCleartexts,
  decryptionProof
}

// verif
FHE.checkSignatures([h], abiEncodedCleartexts, decryptionProof);
const res = await client.fetchCheckSignaturesArgs({ handles });
res {
  handleList,
  abiEncodedCleartexts,
  decryptionProof
}

foo(handles, res.abiEncodedCleartexts, res.decryptionProof)
```

```ts
// TODO TypedValue[]
export type DecryptReturnType = readonly ClearValue[];
```

FhevmHandle { bytes32Hex, fheType }
const h: FhevmHandleImpl;
h.handle

handle = encrypt(123)
123 === decrypt(handle)

- Check Error names

```ts
export type TfheErrorType = TfheError & {
  name: 'TFHEError';
};

export type TfheErrorParams = Prettify<
  Omit<FhevmErrorBaseParams, 'name' | 'message'> & {
    readonly message: string;
  }
>;

export class TfheError extends FhevmErrorBase {
  constructor(params: TfheErrorParams) {
    super({
      ...params,
      name: 'TfheError',
    });
  }
}
```

```ts
// Option 1
export type DecryptValueParameters = {
  readonly handle: HandleLike;
  readonly contractAddress: string;
  readonly signedPermit: SignedSelfDecryptionPermit;
  readonly transportKeyPair: TransportKeyPair;
  readonly options?: RelayerUserDecryptOptions | undefined;
};
export async function decryptValue(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptValueParameters,
): Promise<TypedValue>;

// Option 2
export type DecryptHandleParameters = {
  readonly handle: HandleLike;
  readonly contractAddress: string;
  readonly signedPermit: SignedSelfDecryptionPermit;
  readonly transportKeyPair: TransportKeyPair;
  readonly options?: RelayerUserDecryptOptions | undefined;
};
export async function decryptHandle(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptValueParameters,
): Promise<TypedValue>;

// Option 1 (multi)
export type DecryptValuesParameters = {
  readonly handles: { readonly handle: HandleLike; readonly contractAddress: string }[];
  readonly signedPermit: SignedSelfDecryptionPermit;
  readonly transportKeyPair: TransportKeyPair;
  readonly options?: RelayerUserDecryptOptions | undefined;
};
export async function decryptValues(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptValuesParameters,
): Promise<TypedValue[]>;

// Option 1-bis (multi-single contract)
export type DecryptValuesParameters = {
  readonly handles: HandleLike[];
  readonly contractAddress: string;
  readonly signedPermit: SignedSelfDecryptionPermit;
  readonly transportKeyPair: TransportKeyPair;
  readonly options?: RelayerUserDecryptOptions | undefined;
};
export async function decryptValues(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptValuesParameters,
): Promise<TypedValue[]>;

// Option 2 (multi)
export type DecryptHandlesParameters = {
  readonly handles: { readonly handle: HandleLike; readonly contractAddress: string }[];
  readonly signedPermit: SignedSelfDecryptionPermit;
  readonly transportKeyPair: TransportKeyPair;
  readonly options?: RelayerUserDecryptOptions | undefined;
};
export async function decryptHandles(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptHandlesParameters,
): Promise<TypedValue[]>;

// Option 2 (multi)
export type DecryptHandleContractPairsParameters = {
  readonly handles: { readonly handle: HandleLike; readonly contractAddress: string }[];
  readonly signedPermit: SignedSelfDecryptionPermit;
  readonly transportKeyPair: TransportKeyPair;
  readonly options?: RelayerUserDecryptOptions | undefined;
};
export async function decryptHandleContractPairs(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: DecryptHandleContractPairsParameters,
): Promise<TypedValue[]>;
```
