# pad-mock

Development only, never deployed (the image builds `-p relayer-http`). It exercises the relayer's
`POST /v4/pad-decrypt` (tech-spec RFC-030) before the KMS connector processes pad decryptions (its endpoint accepts
them, its worker rejects them). It has two
subcommands, driven by `dev/scripts/pad.sh` (`make -C dev pad-…`, see `dev/README.md`).

## In short

RFC-030 lets a user encrypt an input with a one-time pad instead of a full FHE ciphertext. To do that, the user
first needs the pad itself, and only the KMS can produce it: every KMS node returns its _share_ of the pad, and the
user rebuilds the pad from enough shares (2t + 1).

Today nothing on the KMS side computes pads yet. pad-mock fills the gap so the relayer route can be run end to end:

- **`pad-mock connectors`** pretends to be the KMS connectors. It starts one small HTTP server per KMS key. Each
  server answers the relayer the way a real connector will, but it gets the pad material from a Solidity contract
  (`CleartextPaddedInputter`) instead of from a real KMS.
- **`pad-mock flow`** pretends to be the user (the SDK). It builds one pad request, sends it to the relayer, and
  checks every answer that comes back.

The relayer in the middle is the real one, unchanged: it fans the request out to every connector, collects the
answers and returns them.

## The big picture

```text
                              ┌────────────────────────────┐
                              │  pad-mock flow             │   plays the user / the SDK
                              │  (one request, then checks)│
                              └──────┬──────────────▲──────┘
                                     │ (a)          │ (5)
          POST /v4/pad-decrypt       │              │  200 { responses: [ one per node ] }
                                     ▼              │
                              ┌─────────────────────┴──────┐
                              │  relayer-http  :8080       │   the real relayer
                              │  validates, then fans out  │
                              └──┬───────┬───────┬───────┬─┘
                     (2)         │       │       │       │     one POST /v1/pad-decrypt per node,
                                 ▼       ▼       ▼       ▼     sent at the same time
                              ┌──────┐┌──────┐┌──────┐┌──────┐
                              │node 1││node 2││node 3││node 4│ pad-mock connectors
                              │:8091 ││:8092 ││:8093 ││:8094 │ (one process, one KMS key per node)
                              └──┬───┘└──┬───┘└──┬───┘└──┬───┘
                     (3)         │       │       │       │     eth_call: "give me my material"
                                 ▼       ▼       ▼       ▼
                              ┌────────────────────────────┐
                              │  anvil  :8544              │   the SDK's cleartext stack
                              │  CleartextPaddedInputter   │   (does the KMS's work in clear)
                              │  KMSVerifier               │
                              │  ProtocolConfig            │
                              └────────────────────────────┘
                     (4) each node signs its own answer with its key and replies to the relayer
                     (6) pad-mock flow also reads anvil to check the answers (signers, decrypted shares)
```

Step by step:

| #   | who             | what                                                                                                                   |
| --- | --------------- | ---------------------------------------------------------------------------------------------------------------------- |
| 1   | flow → relayer  | the user's signed request: a permit plus the list of pads wanted (each a label, its _witness_ and a length)            |
| 2   | relayer → nodes | the relayer checks the request, then sends it to every node at once                                                    |
| 3   | node → anvil    | each node asks the contract for the material it has to sign                                                            |
| 4   | node → relayer  | each node signs with its own KMS key and answers                                                                       |
| 5   | relayer → flow  | once every node answered (or the deadline passed) with at least `threshold` good answers, the relayer returns them all |
| 6   | flow → anvil    | the flow verifies each answer and compares the pads with the expected ones                                             |

Reading order of the code: `main.rs` (flags), `chain.rs` (ABI, EIP-712 structs, JSON-RPC), `connector.rs`,
`flow.rs`.

## `connectors`: the fake KMS connectors

One mock KMS connector per key (`--keys`), on consecutive ports from `--listen` (in dev: 8091 to 8094). Each serves
the connector's `POST /v1/pad-decrypt` DTOs (`kms_connector_api::PadDecryptionRequest` / `PadDecryptionResponse`).

In the SDK's cleartext stack, the contract `CleartextPaddedInputter` already does all of the KMS's work: checks,
pad computation, encryption for the user. The only thing it cannot do is sign, because the keys stay off chain. So
a node is mostly a signing loop around two contract calls:

```text
   relayer                       node k (key k)                         anvil (CleartextPaddedInputter)
      │                               │                                            │
      │  POST /v1/pad-decrypt         │                                            │
      │──────────────────────────────▶│                                            │
      │                               │  1. getCleartextPadSharesMaterial(request) │
      │                               │───────────────────────────────────────────▶│ runs every RFC-030 check;
      │                               │◀───────────────────────────────────────────│ reverts if one fails
      │                               │     signers[], shareDigests[][]            │
      │                               │                                            │
      │                               │  2. find own address in signers[],         │
      │                               │     sign own row of shareDigests           │
      │                               │                                            │
      │                               │  3. getCleartextPadResponsesMaterial(      │
      │                               │       request, shareSignatures)            │
      │                               │───────────────────────────────────────────▶│ encrypts the shares
      │                               │◀───────────────────────────────────────────│ for the user's key
      │                               │     requestId, encryptedPayloads[],        │
      │                               │     transportDigests[]                     │
      │                               │                                            │
      │                               │  4. sign own transportDigest               │
      │  200 { decryptionId,          │                                            │
      │        encryptedPayload,      │                                            │
      │        signature }            │                                            │
      │◀──────────────────────────────│                                            │
```

The four steps in words:

1. **Ask for the material.** `getCleartextPadSharesMaterial(request)` runs the RFC-030 checks (permit signature and
   validity window, v2 `extraData` naming a valid context and epoch, witness rules, label = hash(witness),
   authorization, delegation). If all pass, it returns the list of KMS signers of the context and, for each signer,
   the digests that signer must sign (one per pad).
2. **Sign the shares.** The node finds its own address in `signers` and signs its row of `shareDigests`.
3. **Get the encrypted answer.** `getCleartextPadResponsesMaterial(request, shareSignatures)` builds the payload
   encrypted for the user. The contract expects one row of signatures per node but only reads this node's row for
   this node's payload, so the node simply repeats its own row for every node.
4. **Sign the answer.** The node signs its `transportDigests` entry and replies
   `{ decryptionId, encryptedPayload, signature }`, its `decryptionId` being the contract's `requestId`.

What the relayer gets back:

| outcome                                              | answer                                                    |
| ---------------------------------------------------- | --------------------------------------------------------- |
| success                                              | `200`, the node's `PadDecryptionResponse`                 |
| unreadable body                                      | `400 malformed`                                           |
| the call reverts (a check of step 1 failed)          | `422 unprocessable`, the revert data in the message       |
| chain unreachable                                    | `502 upstream_transient` (retryable: the relayer retries) |
| the key is not a signer of the context, bad material | `500 unknown`                                             |

### What a real connector does that this mock does not

| real connector                                                                      | pad-mock                                                                                               |
| ----------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| verifies the relayer's `FhevmSig` header                                            | ignores it                                                                                             |
| stores each request and replays the stored answer when the same request comes again | stores nothing, recomputes every time                                                                  |
| asks its own KMS node, which may be slow, down or wrong                             | all four nodes read the same anvil, so they always agree and answer within milliseconds                |
| `decryptionId` derived from the request (its EIP-712 id)                            | the contract's mock id, `keccak256(labels ‖ extraData ‖ signature)`; the relayer does not compare them |

So this setup tests the happy path end to end. Slow, failing or dishonest nodes are covered only by the relayer's
own unit tests (`kms_aggregator/mock.rs` and the YAML scenarios).

## `flow`: the fake user

Plays the SDK for one request, then checks the answers the way the SDK will.

**The request.** Two fresh witnesses (one 64-bit pad and one 300-bit pad, so one and two PRF blocks), signed by
`--user-key` under the KMSVerifier's EIP-712 domain on the host chain, posted to the relayer.

**The checks**, in order:

```text
  relayer answer: [ response node 1, response node 2, response node 3, response node 4 ]
         │
         ▼
  ① who signed it?      every transport signature (PadDecryptResponseVerification, `fhevm-pad-decryption`
                        domain on ProtocolConfig) recovers to a KMS signer of the context
                        (ProtocolConfig.getKmsSignersForContext), and no signer answers twice
         │
         ▼
  ② what is inside?     every payload decrypts (cleartextDecryptPadPayload) into an extraData naming the
                        request's context and epoch, and shares signed by that same signer together with
                        that extraData (PadShareVerification)
         │
         ▼
  ③ enough, and         per pad (label): at least --threshold shares (2t + 1),
     consistent?        all with the same sessionId and the same rawShares
         │
         ▼
  ④ right pad?          the pad equals the cleartext PRF stream:
                        keccak256("FHE_cpad" ‖ prfId ‖ label ‖ i_le32) for each 256-bit block i
```

One detail: the permit starts at the latest block's timestamp, not the wall clock. On an idle anvil the two drift
apart, and the contract refuses a permit that starts after `block.timestamp`.

## Running it

In four terminals:

```sh
make -C dev pad-stack        # 1. anvil + the SDK's cleartext contracts       (:8544)
make -C dev pad-connectors   # 2. the four fake connectors                    (:8091 … :8094)
make -C dev pad-run          # 3. relayer-http with dev/config/pad-cleartext.yaml  (:8080)
make -C dev pad-decrypt      # 4. one request through pad-mock flow, with all the checks
```

## Testing

Unit tests run with the workspace (`cargo nextest run --workspace`): the typehashes against the contracts' type
strings, the DTO conversion, the node against a stand-in JSON-RPC chain (its own entry, its key's signatures, the
error mapping), the flow's wire body against the relayer's validation.
