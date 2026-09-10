# EVM → Solana parity

This maps the implemented PoC to EVM FHEVM outcomes. It does not claim identical ACL
semantics. The current account and composition model is RFC35 / PR3883; DD-049 in
[DESIGN_DECISIONS.md](DESIGN_DECISIONS.md) replaces older account interfaces.

## Encrypted computation and permissions

| EVM capability | Current Solana path | Difference |
|---|---|---|
| Contract storage holds encrypted handles | Host-owned `EncryptedState` PDA `(program, authority, scope)` holds bounded keyed slots and a shared MMR | Apps read accounts; slot keys do not create separate PDAs. |
| `FHE.fromExternal` | `VerifiedInput` operand verifies coprocessor threshold attestations in `fhe_execute` | Host binds chain and app program; app checks the attested user. Verification alone creates no persistent decrypt permission. |
| FHE expression evaluation | `fhe_execute` supports mixed binary, ternary, unary, trivial, random, sum, isIn and mulDiv operations | At most 32 steps; packet, heap, CU and HCU limits may bind earlier. |
| Previously allowed encrypted operand | State-slot input requires exact current handle and State authority signature; a transient input requires its producing State or an exact scratch grant, plus that State authority signature | Compute permission is separate from an MMR decrypt permission. |
| `FHE.allow` / `allowThis` | A fresh State output appends exact-handle/key leaves, optionally without storing a slot | Existing viewers cannot directly re-share a historical handle. New history-only grants are deferred to [#2007](https://github.com/zama-ai/fhevm-internal/issues/2007). |
| `FHE.allowTransient` | All results are implicitly usable by their producing State across calls; explicit grants share them with other States | Scratch must be opened and closed in that transaction. It authorizes exact handle/consumer-State use, and its expiry does not restrict derived outputs. |
| Encrypted return value | `returned_results` selects up to 32 handles by `(step_index, output_index)` | Current operations have one output. Order and duplicates are preserved; empty selection returns none. Return bytes confer no permission. |
| Make publicly decryptable | Fresh outputs may append public leaves; `make_state_handle_public` publishes an exact current slot under its authority signature | Publication of history-only handles is deferred to #2007. An existing public leaf stays valid after slot replacement. |
| `isAllowed` / decrypt authorization | Connector verifies exact-handle/key MMR inclusion against its own State snapshot | It requests proofs from coprocessors; clients do not supply authorization proofs. Compute access does not follow from this leaf alone. |
| Application identity (`msg.sender`) | Program identity proven by authority PDA creation; execution names `(program, scope)` | Program, scope and authority have separate roles. Program upgrade authority remains trusted to preserve app policy. |
| Handle type/chain metadata | Shared 32-byte handle layout and high-bit Solana chain classification | Handles are not portable across chains. Bank-hash/timestamp entropy is Solana-specific; missing entropy fails closed. |
| HCU limits | Per-execution total/depth plus per-application per-slot cap | No global EVM-style block cap. Disabled limits are explicitly configured; runtime cost snapshots measure supported shapes. |

Supported FHE types cover Bool and Uint8 through Uint128. Signed, larger and string types
are outside this PoC. Coprocessor and KMS signature checks use the same EIP-712 trust roots
as EVM; threshold checks count distinct registered signers. Unknown signatures do not raise
the count (the accepted behavior tracked in #1888 differs from EVM's rejection).

## Confidential token and vault composition

| Outcome | Solana interface and behavior |
|---|---|
| Transfer fresh encrypted amount | Owner-authorized `confidential_transfer` consumes an attestation, computes the conditional transfer amount, and updates sender/receiver balance slots. |
| Transfer an existing handle | `confidential_transfer_from_value` uses a current slot or scratch grant through `TransferInput`, with the required authority signatures. |
| Receiver contract composes with transferred result | Receiver app exposes its own entrypoint. Token returns the transferred handle and grants the consuming State through scratch. Batcher updates its own contribution; each JoinRecord owns that State. No transfer receipt accumulator or permanent transferred-amount register. |
| Self-transfer | Plain self-transfer is a no-op; requesting a result grant on that path is rejected. |
| Operator/delegated transfer | Deliberately absent; tracked in fhevm-internal#1692. Owner-signed transfer is supported. |
| Balance / total supply | Read the corresponding slot in the token-account or total-supply-authority State. |
| Mint | `wrap_usdc` escrows underlying SPL tokens and increases encrypted balance and supply. This is wrapping rather than arbitrary ERC7984 minting. |
| Burn / underlying redemption | Burn reduces balance/supply, records its result in a slot and creates one PendingBurn per token account. Redemption verifies the exact result/certificate and consumes PendingBurn before another burn can start. |
| Cancel pending burn | Restores encrypted balance and supply; cannot also redeem that same pending burn. Batcher cancellation requires the join mint authority and opens user refunds. |
| Public disclosure | `disclose_secp` verifies State binding, exact-handle public proof and KMS certificate, then emits State/handle/cleartext. It does not certify a caller-supplied token-kind label. Original token events identify transfer/burn provenance. |
| Later viewers | Authority-gated balance/supply wrappers produce a new output with the requested viewers. They do not grant a new viewer access to an arbitrary history-only handle. |

Classic Token and extension-free Token-2022 are supported. Mint extensions fail closed;
`ImmutableOwner` is the only accepted token-account extension. Frozen-account checks cover
the canonical underlying ATAs, with the existing no-ATA/closed-ATA limitation tracked in #1981.
Wrap and redeem also validate the accounts they move; cancellation is not freeze-gated.

A dust deposit can fail vault settlement with `ZeroShares`. Recovery is available through
mint-authority cancellation followed by user `quit`; it is not permissionless. The vault and
batcher tests cover settlement rollback and the cancellation/refund lifecycle.

## Gateway, coprocessor and KMS

The existing Gateway V2 route carries typed Solana user-decrypt requests. The relayer handles
request shape and routing; each connector independently verifies the signed permit, scope,
window, revocation watermark, deployment identity, KMS context and each State's canonical
address/owner/shape. Delegation uses an expiring PDA and a fresh deciding snapshot. The State's
authority scopes delegation; its app `(program, scope)` scopes the permit.

The listener reconstructs all executed operations and permission leaves from transaction bytes,
independently of return selection. Compute records, leaves and checkpoint commit atomically.
It serves inclusion proofs via `/v1/solana/leaf-proofs`; consumers verify them against their own
chain snapshot. Missing retained history fails closed and needs recovery beyond an ordinary
retry. The listener's confirmed-chain scheduling does not itself authorize plaintext release.

Public consumers verify the KMS certificate and MMR proof on-chain. History growth can make
a fetched proof stale before submission; rejection is atomic and a refreshed proof can reuse
the same exact-handle certificate. Shared-history proofs are not assumed to have fixed depth;
large proofs remain subject to Solana packet limits (#1750).

Ciphertext material uses the shared gateway/coprocessor infrastructure, without a Solana
HandleMaterialCommitment account. Gateway material registration and copro attestations are
separate concerns; supporting an attestation authorization mode does not imply coprocessors
stop posting ciphertext material to the gateway.

The shared gateway retains its cleartext bit budget and Solana request handle cap. KMS core
performs threshold decryption; each party's connector trusts its configured host RPC, as it
does for EVM ACL reads. Confirmed rather than finalized authorization is an explicit trust
choice, not a listener guarantee.

## Remaining delivery and security boundaries

Production provider deployment, key/registry lifecycle, payment and infrastructure operations
still require their owning teams. Confirmed-chain reorg recovery is distinct from authorization
and from provider-retention recovery. ABI/account changes require all consumers and generated
IDLs to agree; there is no backward-compatibility requirement for the retired Solana PoC model.

Tests establish the exercised behavior, not a blanket security verdict. The outstanding work
and accepted assumptions belong in [INVARIANTS.md](INVARIANTS.md) and the linked issues;
a deferred feature must not conceal a defect in an already supported flow.
