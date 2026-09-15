# QA KMS context — scenario 4: the Gateway reverts directly submitted corrupted extraData

Implementation report for the fourth scenario of the `kms-context-qa-tests` profile, implemented as
the `extradata-gateway-rejection` case.

**Status:** delivered and green against a live stack.
**Scope:** the `Decryption` contract's own validation of the KMS-context `extraData`, reached by
bypassing the Relayer entirely.

> Scenario 3 proves the **Relayer** refuses a corrupted extraData over HTTP. This one proves the
> **contract** does too. The Relayer is a service that can be replaced, misconfigured, or simply
> skipped by anyone willing to send their own transaction; a check that lives only there is a
> convenience, not a guarantee.

---

## 1. The scenario: as proposed, and as implemented

### 1.1 As originally proposed

```gherkin
Feature: Gateway handling of directly submitted corrupted extraData

  Scenario: The Gateway reverts calldata with corrupted extraData
    Given ProtocolConfig returns a valid active pair "C1, E1"
    And an instrumented SDK has built a valid decryption request
    And the original Gateway calldata contains extraData "0x02 || C1 || E1"
    And the test harness changes only the extraData version byte to an unsupported version
    When the altered ABI calldata is submitted directly to the Gateway Decryption contract
    Then the transaction must revert
    And no decryption request event must be emitted
```

### 1.2 As implemented

```gherkin
Scenario: The Gateway reverts calldata with corrupted extraData
  Given ProtocolConfig reports an active pair "C1, E1"
  And globalThis.fetch is instrumented so the SDK's signed request is captured, never sent
  And the captured extraData is exactly "0x02 || C1 || E1"
  And the envelope's fields are mapped onto the UNIFIED userDecryptionRequest arguments
  When only the extraData version byte is changed to 0x03
  And the calldata is eth_call'd against the Decryption contract
  Then the call must revert with "UnsupportedExtraDataVersion(3)"
  When the same calldata is submitted as a real transaction
  Then the receipt status must be 0
  And the transaction must carry no logs
  And the Decryption contract must have emitted nothing in that block

Scenario: The same calldata, untouched, is not rejected for its extraData
  Given the same capture, with its extraData intact
  When the calldata is eth_call'd against the Decryption contract
  Then it must not revert with "UnsupportedExtraDataVersion"
```

### 1.3 Why it changed

**"The original Gateway calldata" does not exist.** The SDK never builds Gateway calldata: it POSTs
to the Relayer over HTTP, and the **Relayer** submits the transaction, with its own key. There is
nothing of the SDK's to capture at the calldata level.

What does exist is the SDK's signed unified envelope — already captured by scenario 3 — whose fields
map one-to-one onto the unified `userDecryptionRequest` arguments, because the Relayer forwards
exactly those values to exactly that function. The case reuses that capture and re-addresses the
request from the Relayer to the Gateway. It is genuinely the SDK's request, sent somewhere else.

**"Must revert" became "must revert with `UnsupportedExtraDataVersion(3)`".** A corrupted request can
revert for several reasons; §2 explains why naming the one that matters is not pedantry here.

**The clause is asserted twice, by two mechanisms.** An `eth_call` yields the ABI-encoded custom
error, which is the precise assertion; a real transaction then yields a mined, status-0 receipt,
which is the scenario's literal wording and the only way to say anything about events.

**A control was added**, deliberately weak — see §5.

### 1.4 Clause-by-clause coverage

| Clause | Covered by | Where |
|---|---|---|
| active pair is `C1, E1` | `readCurrentPair`, injected and cross-checked | both |
| SDK has built a valid request | the capture, asserted to be `0x02 \|\| C1 \|\| E1` | container |
| only the version byte changes | `expect(corrupted.slice(4)).to.equal(original.slice(4))` | container |
| submitted directly to the Gateway | `wallet.sendTransaction` to `DECRYPTION_ADDRESS` | container |
| the transaction must revert | `eth_call` revert data **and** `receipt.status === 0` | container |
| no decryption request event | `receipt.logs` empty **and** no logs from the contract in that block | container |

---

## 2. Two overloads, opposite orderings — and only one is testable

`Decryption.sol` carries **two** `userDecryptionRequest` functions. Which one the scenario targets is
not a detail; it decides whether the scenario is expressible at all.

| | legacy `(CtHandleContractPair[], …)` | **unified `(HandleEntry[], …)`** |
|---|---|---|
| EIP-712 signature | verified **on chain**, line 506 | **not verified on chain at all** |
| `_extractContextId` | line 542, *after* the signature | line 690, the **fourth** statement |
| fee collection | line 548 | line 704, *after* the version check |
| tampered-after-signing reverts with | a **signature** error | `UnsupportedExtraDataVersion` |

On the legacy path the version check is never reached, so the scenario would be untestable there: the
request is rejected, but for the other defect. On the unified path there is no competing check at
all — authorization moved to the KMS Connector, and the signature is merely packed into the payload
and emitted. The contract's own comment states the intent:

> *Reject an unknown or destroyed context before charging the fee, so a request that could never be
> answered is not opened and paid for.*

The case targets the unified overload: it is what the SDK and the Relayer use, and the legacy one is
marked `@custom:deprecated`.

**This is the mirror of scenario 3.** There, the format check beats the signature check by *order*.
Here, it beats it by the signature check not existing on chain.

---

## 3. What the contract validates

`_extractContextId` (`gateway-contracts/contracts/Decryption.sol:976`):

| extraData | Result |
|---|---|
| empty, or version `0x00` | current KMS context id |
| version `0x01` / `0x02`, ≥ 33 bytes | contextId from bytes 1..33 |
| version `0x01` / `0x02`, < 33 bytes | `InvalidExtraDataLength(length, 33)` |
| contextId `0` | `InvalidNullContextId()` |
| **any other version** | **`UnsupportedExtraDataVersion(version)`** |

Then `_validateContextId` rejects a context the Gateway does not know, with
`InvalidKmsContext(contextId)`.

The two are distinguishable on the wire, which is what lets the case say *format*, not merely
*rejected*:

```
extraData 0x03…                        -> 0x2139cc2c  UnsupportedExtraDataVersion(3)
extraData 0x02… with an unknown context -> 0x77ddbe81  InvalidKmsContext(…)
```

Both observed directly against the running Gateway, with a garbage signature and a nonexistent
handle — which is itself the proof that the version check precedes everything that could mask it.

---

## 4. What the case does

1. Read the active pair from `ProtocolConfig` and hand it to the container.
2. Instrument `globalThis.fetch`; let the real SDK build and sign a unified request; capture it
   without sending (the same harness as scenario 3, reused unchanged).
3. Assert the captured extraData is exactly `0x02 || C1 || E1`.
4. Map the envelope onto the unified `userDecryptionRequest` arguments.
5. Change only the version byte to `0x03`; assert every other byte is identical.
6. `eth_call` the corrupted calldata → must revert `UnsupportedExtraDataVersion`, argument `3`.
7. Submit it as a real transaction with an explicit gas limit → mined, `status === 0`, no logs, and
   no logs from the Decryption contract anywhere in that block.
8. `eth_call` the **untouched** calldata → must not revert with `UnsupportedExtraDataVersion`.

An explicit `gasLimit` is what makes step 7 possible: without it, ethers' `estimateGas` preflight
fails client-side and no transaction is ever mined, so there would be no receipt and no block to
inspect for events.

---

## 5. Why the control is deliberately weak

`_collectUserDecryptionFee` transfers a fee token from `msg.sender` (`userDecryptionPrice` is `1e18`
on this stack), so a fully successful submission would need the test account funded **and** approved
for that token — a payment setup that varies by stack and has nothing to do with extraData.

The corrupted case is unaffected: it reverts at line 690, before the fee at line 704. So the control
asserts only that the **untouched** calldata is not rejected *for its extraData version*. Observed:

```
control: untouched calldata reverted with 0xfb8f41b2 (not UnsupportedExtraDataVersion)
```

That is the fee failing, exactly as expected on an unfunded account — and it does the control's whole
job, which is to establish that the corrupted case's revert is attributable to the byte that was
changed rather than to the harness having built calldata the contract would refuse either way.

Strengthening it to a full positive control is possible later by minting and approving the fee token;
nothing in the case's structure would change.

---

## 6. How to run

```bash
cd test-suite/fhevm

# this scenario only
KMS_QA_CASES=extradata-gateway-rejection ./fhevm-cli test kms-context-qa-tests

# the container half on its own
./fhevm-cli test kms-context-extradata-gateway
```

Standalone, against a running stack (the orchestrator cross-check skips):

```bash
cd test-suite/e2e
npx hardhat test --grep "KMS context extraData gateway rejection" --network staging
```

Requires `GATEWAY_RPC_URL`, `DECRYPTION_ADDRESS` and `GATEWAY_DEPLOYER_PRIVATE_KEY`; the suite skips
itself, loudly, when any is missing rather than reporting a configuration gap as a failure.

**Non-disruptive**, like scenario 3: `mutatesLifecycle: false`, no topology requirements, no lifecycle
transaction, no context or epoch advanced. It does land one reverted transaction on the gateway chain,
costing gas and writing no state, so it can be rerun freely.

---

## 7. Verification

**Static.** `test-suite/fhevm` `bun run check` clean, `bun test src` → 477 pass, 0 fail.
`test-suite/e2e` `npm run tsc` reports for the new file only the pre-existing
`signUnifiedDecryptionPermit` error caused by the locally installed SDK build lagging the specs —
the same one already present on its two siblings in this directory.

**Live.** `PASS (61s)`, 5 evidence steps, driven through the profile.

```
[kms-context-qa][extradata-gateway-rejection][note]  active pair … contextId=ctx#11 epochId=epoch#24
[kms-context-qa][extradata-gateway-rejection][probe] the Gateway reverts calldata carrying a
                                                     corrupted extraData version, and emits
                                                     nothing  ok 60.7s
  ✔ agrees with the orchestrator
  ✔ reverts calldata with a corrupted version byte
  ✔ does not reject the same calldata when it is untouched      (112ms)
  3 passing
```

### The first run failed, and the test framework was at fault, not the protocol

The protocol behaved correctly on the first attempt — the receipt showed `status: 0` and `logs: []`,
exactly the scenario's two clauses. The suite failed because **ethers v6 throws from `wait()` when a
receipt reports a revert**, so the assertions were never reached. Here a revert is the expected
outcome, not an error: the rejection is now swallowed and the receipt read back with
`provider.getTransactionReceipt`.

Worth keeping: the failure output carried the full receipt, so the protocol's correct behaviour was
visible in the same message that reported the test as failing.

---

## 8. Related files

| Path | Role |
|---|---|
| `test-suite/fhevm/src/kms-qa/cases/case-extradata-gateway-rejection.ts` | the case, host side |
| `test-suite/e2e/test/kmsContextExtraData/extraDataGatewayRejection.ts` | the container half |
| `test-suite/e2e/test/kmsContextExtraData/extraDataRejectionHttp.ts` | the capture harness, shared with scenario 3 |
| `qa-kms-context-scenario-3-extradata-rejection.md` | the Relayer-level sibling |
| `gateway-contracts/contracts/Decryption.sol` | `_extractContextId`, and the two overloads |
| `gateway-contracts/contracts/interfaces/IDecryption.sol` | `UnsupportedExtraDataVersion` and friends |
| `gateway-contracts/contracts/ProtocolPayment.sol` | the user-decryption fee the control runs into |
