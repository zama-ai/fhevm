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
- remove E2e in E2eTransportKeypair
- OK EIP712 -> Eip712 etc.
- ClearValueTypeMap is unused!
- handle : euint8
- verifyKmsPublicDecryptEIP712 should use createKmsPublicDecryptEIP712
- handle = Bytes32Hex (0x....)
- object { } FhevmHandleImpl implement

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
