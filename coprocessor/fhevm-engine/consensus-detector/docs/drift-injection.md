# Manifest drift injection

For controlled E2E, devnet, and testnet recovery exercises, start one detector with:

```sh
consensus-detector ... --dangerous-drift-injection /config/drift-injection.json
```

The file is read once at startup, before connecting to external services:

- No flag, or an absent file: injection is disabled.
- A valid file with `"enabled": false`: injection is disabled.
- An enabled configuration: inject the selected manifest descriptor.
- An invalid or unreadable file: fail startup. Unknown fields and fault types
  are rejected.

Mount the directory `/config`, not an individual file, so the test runner can
create/remove the file and atomically replace it between service starts.
An example active file is:

```json
{
  "enabled": true,
  "chain_id": 12345,
  "handle": "0x5151515151515151515151515151515151515151515151515151515151515151",
  "fault": "ct64_digest_bit_flip"
}
```

The handle must be exactly 32 bytes. The chain ID must fit a non-negative SQL
BIGINT. The fault flips the least significant bit of the first byte of that
handle's ct64 digest in each freshly loaded computed manifest descriptor. Other
handles/chains and error/uncomputed descriptors are unchanged. Database digest
rows, ciphertext bytes, upload witnesses, and ciphertext S3 objects are unchanged.

Both sealing and manifest construction use the same transformation. Block,
range, and payload hashes and signatures are computed normally, yielding a valid
signed disagreement rather than an invalid signature. Activation and application
are warning-level events, including the handle and original/injected digests.
The ordinary publication event identifies the published manifest.

## E2E stop/start sequence

Use three homogeneous coprocessors with quorum two, and enable the file argument
only on the target detector. Use a publication cadence of one for the test chain.

1. Leave the file absent. Start the stack and wait for the detector to establish
   its publication frontier; a first-ever legacy startup starts at the current tip.
2. Stop the target detector before submitting the fixture. Other services continue.
3. Submit the fixture and wait for its ciphertexts to materialize. Obtain the
   selected producer handle from the fixture receipt/events.
4. Write the active JSON file and start the same detector container.
5. Wait for its actual S3 manifest to contain the injected descriptor, for healthy
   peers to establish the original digest's quorum, and for local drift/containment.
6. Once healing installation is available, assert `healed_at`, a change in the
   ciphertext row's `xmin::text`, unchanged correct ciphertext bytes, and resumption
   of eligible dependent work. A separate healing gate is needed to reliably
   observe containment before fast automatic repair finishes.
7. In cleanup, stop the detector, remove the file, and start it again.

The flag is fixed when the container is created; only the mounted file changes.
No container recreation or hot reload is needed. Leave the same active file in
place across publication retries/restarts, including recovery from an S3 PUT
that succeeded before the database transaction committed. Removing the file
mid-publication can change the intended payload and conflict with the immutable
object. Published manifests are not rewritten or retracted when injection is
removed. The mode targets the selected handle whenever it is loaded, not merely
one function call; use a fresh fixture handle for each exercise.

## Runnable local profile

The local CLI now supplies the mount, fixture, orchestration, and assertions:

```sh
cd test-suite/fhevm # from the repository root
./fhevm-cli up --target latest-main --scenario manifest-lifecycle --build
./fhevm-cli test manifest-lifecycle
```

This first profile covers signed publication, quorum verification, containment,
unchanged stored material, and independent progress. It does not yet test healing
installation. See `test-suite/fhevm/README.md` for report paths and cleanup details.
