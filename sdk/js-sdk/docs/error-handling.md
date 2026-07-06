# Error handling

The SDK throws typed error classes. Each one sets a distinct `name` and carries
structured context — there are no numeric error codes. Identify an error by its
`name` (or `instanceof`), not by parsing its message.

```ts
try {
  await client.decryptValue({ transportKeyPair, encryptedValue, contractAddress, signedPermit });
} catch (err) {
  if (err instanceof Error && err.name === 'AclUserDecryptionError') {
    // the user isn't allowed to decrypt this value
  }
}
```

Every SDK error extends the built-in `Error`, so `err.message`, `err.name`, and
`err.cause` are always available. Domain errors add extra getters (a contract
address, a list of handles, an HTTP status) depending on the error.

## Categories

Errors fall into three families:

- **Validation errors** — you passed something malformed (a bad address, an
  invalid handle). Thrown synchronously, before any network call.
- **FHEVM domain errors** — the operation is well-formed but not permitted or not
  possible (Access Control List (ACL) denies decryption, an input proof is invalid).
- **Relayer errors** — the request to the Relayer failed, timed out, was aborted,
  or came back rejected.

## Validation errors

Thrown when an argument doesn't parse. Catch these to give users immediate
feedback without a round-trip.

| `name`                     | Meaning                                                |
| -------------------------- | ------------------------------------------------------ |
| `AddressError`             | A value isn't a valid Ethereum address.                |
| `ChecksummedAddressError`  | An address failed EIP-55 checksum validation.          |
| `FheTypeError`             | An unknown or invalid FHE type name/id.                |
| `InvalidTypeError`         | A value didn't match the expected type.                |
| `InvalidPropertyError`     | An object was missing or had an invalid property.      |
| `InvalidUrlError`          | A URL isn't valid.                                     |
| `Sha256VerificationError`  | A downloaded WASM/worker asset failed its SHA-256 check. |

## FHEVM domain errors

Thrown by encrypt, decrypt, and configuration operations.

| `name`                    | Thrown when…                                                        |
| ------------------------- | ------------------------------------------------------------------ |
| `AclUserDecryptionError`  | The user isn't allowed to privately decrypt this handle.           |
| `AclPublicDecryptionError`| One or more handles aren't allowed for public decryption. Exposes `handles`. |
| `EncryptionError`         | An encryption operation failed.                                    |
| `ZkProofError`            | The zero-knowledge proof is invalid.                               |
| `InputProofError`         | The input proof is invalid.                                        |
| `FhevmHandleError`        | A handle (encrypted value) couldn't be parsed.                     |
| `TooManyHandlesError`     | More than 256 variables packed into one input ciphertext.          |
| `FhevmConfigError`        | The FHEVM configuration is invalid.                                |
| `UnknownSignerError`      | A signer address isn't in the coprocessor/KMS signer set.          |
| `ThresholdSignerError`    | The signer quorum threshold wasn't reached.                        |
| `DuplicateSignerError`    | A duplicate signer address was found.                              |
| `TfheError`               | A failure inside the TFHE WASM layer.                              |

## Relayer errors

Thrown when talking to the Relayer. Response errors carry an HTTP `status`; fetch
errors carry the `url`, `operation`, and retry metadata.

| `name`                                | Thrown when…                                            |
| ------------------------------------- | ------------------------------------------------------- |
| `RelayerAbortError`                   | The request was aborted via an `AbortSignal`.           |
| `RelayerTimeoutError`                 | The request exceeded its timeout.                       |
| `RelayerMaxRetryError`                | Polling exceeded the maximum retry count.               |
| `RelayerFetchError`                   | A network error or JSON parse failure.                  |
| `RelayerStateError`                   | The request can't run in its current state.             |
| `RelayerResponseApiError`             | The Relayer returned an API error. Exposes `relayerApiError`. |
| `RelayerResponseStatusError`          | An unexpected HTTP status (e.g. a 403 from a proxy). Exposes `status`. |
| `RelayerResponseInvalidBodyError`     | The response body didn't match the expected schema.     |
| `RelayerResponseInputProofRejectedError` | The Relayer rejected the submitted input proof.      |

## Handling patterns

**Abort a slow request.** Pass an `AbortSignal` and catch `RelayerAbortError`:

```ts
const controller = new AbortController();
const timer = setTimeout(() => controller.abort(), 30_000);

try {
  await client.encryptValues({
    contractAddress,
    userAddress,
    values,
    options: { signal: controller.signal },
  });
} catch (err) {
  if (err instanceof Error && err.name === 'RelayerAbortError') {
    // handle cancellation
  }
} finally {
  clearTimeout(timer);
}
```

**Prefer `canDecrypt*` over catching ACL errors.** To decide whether to _offer_
decryption, check permission up front instead of catching
`AclUserDecryptionError`:

```ts
const { allowed } = await client.canDecryptValue({ encryptedValue, contractAddress, signedPermit });
if (allowed) {
  // safe to call decryptValue
}
```

**Inspect the cause.** Errors chain via the standard `cause` property, so you can
reach the underlying failure:

```ts
catch (err) {
  console.error(err.name, err.message, err.cause);
}
```

## Related

- [Decryption → Checking permission](decryption.md#checking-permission-before-decrypting) — avoid ACL errors entirely.
- [Encryption](encryption.md) — where `EncryptionError` / `ZkProofError` originate.
- [Runtime configuration](runtime-configuration.md) — `Sha256VerificationError` and asset loading.
```

