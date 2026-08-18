// Scenario: the fhe_execute decrypt vertical — the compute/decrypt phases ported from
// `solana/scripts/e2e/full-vertical.sh` (phases [compute], [public-decrypt], [user-decrypt],
// [historical-user-decrypt], [input-flow]). Every step is typed: the executions go through the
// Codama zama-host client (`src/solana/fhe-execute.ts`), the proofs through solana-proof-service
// clients, and the decrypts through the SDK actions — no live-client, no stdout scraping.
//
// Assertion map — bash phase -> this scenario:
//   [compute] "result handle" + "allow_for_decryption" greps
//     -> typed returns: `trivialEncryptPersistent` yields the on-chain current handle;
//        `allowForPublicDecryption` (inside `releaseAndDecrypt`) throws if either release
//        instruction fails.
//   [compute] SNS commit poll -> `stack.waitForSnsCommit(handle)`.
//   [public-decrypt] cleartext == VALUE -> `releaseAndDecrypt` cleartext asserted here (live peaks +
//        proof-service inclusion proof + KMS certificate through the SDK public-decrypt action).
//   [user-decrypt] pure-SDK cleartext == VALUE -> `runSolanaCurrentUserDecrypt` (ML-KEM keygen,
//        v3 ed25519 request, in-SDK de-signcryption) with UD_EXPECTED = VALUE.
//   [historical-user-decrypt] H_OLD2 == H_OLD and aclValueKey unchanged across the update
//     -> typed equality: the updated encrypted value account keeps its encrypted value id, and the
//        decrypt targets the ORIGINAL handle object (no stdout re-extraction to disagree with
//        itself); the scenario asserts the update really produced a different current handle.
//   [historical-user-decrypt] old-handle decrypt == old cleartext -> `historicalUserDecryptExpect`
//        with the proof-service access proof (leaf resolved by the service, `verified: true` gated).
//   [input-flow] decrypt(VerifiedInput(V) + ADD) == V + ADD -> in-process SDK input proof ->
//        one Binary Add fhe_execute (attestation re-verified on-chain) -> public decrypt == V+ADD.
//   [proof-service] restart gate (readiness -> docker restart -> post-restart exact-replay)
//     -> `stack.restartProofService()` before the first arc, so every proof the suite consumes
//        is served from a restarted, ledger-replayed service (#1682/#3215).
//
// Consciously dropped from the bash: the in-binary `zama_solana_acl::mmr_verify` pre-check of the
// historical proof (the proof service's `verified: true` gate plus the decrypt itself failing on a
// bad proof cover it; the local re-verification was scaffolding for the stdout protocol).

import { describe, expect, test } from "bun:test";

import {
  ExecutionDictionary,
  FHE_TYPE,
  FheBinaryOpCode,
  persistentOutput,
  persistentValueTarget,
  scalarBytes,
  sendFheExecute,
} from "../../src/solana/fhe-execute";
import {
  currentHandle,
  fetchHistoricalAccessProof,
  historicalUserDecryptExpect,
  paddedLabel,
  releaseAndDecrypt,
  trivialEncryptPersistent,
} from "../../src/solana/fhe-vertical";
import { runSolanaCurrentUserDecrypt } from "../../src/solana/current-user-decrypt";
import { solanaUserDecryptContext } from "../../src/solana/two-holder-transfer";
import { submitUint64InputProof } from "../harness/solana/sdkEncrypt";
import { verticalSetup } from "../harness/solana/vertical";

// Each phase does its own compute + SNS commit wait (up to ~3min) + KMS round-trips.
const SCENARIO_TIMEOUT_MS = 15 * 60_000;

const hex = (bytes: Uint8Array): string => `0x${Buffer.from(bytes).toString("hex")}`;
const hexToBytes = (value: string): Uint8Array => Uint8Array.from(Buffer.from(value.replace(/^0x/, ""), "hex"));

describe("solana fhe_execute decrypt vertical", () => {
  test(
    "trivial-encrypt 42 -> allow -> public-decrypt == 42 -> pure-SDK user-decrypt == 42",
    async () => {
      const { stack, context, wallet, config, secretKey, walletHex } = await verticalSetup();

      // Restart gate (full-vertical.sh "[proof-service] decisive vertical gates", #1682/#3215):
      // bounce the proof service before any proof is consumed, so every inclusion proof this
      // suite fetches is served from an exact-inclusive ledger replay, not warm process state.
      await stack.restartProofService();

      const result = await trivialEncryptPersistent(context, {
        payer: wallet.signer,
        value: 42n,
        label: paddedLabel("vertical-trivial"),
      });
      const { cleartext } = await releaseAndDecrypt(context, config, stack, {
        payer: wallet.signer,
        result,
      });
      expect(cleartext).toBe(42n);

      // Pure-SDK current user-decrypt of the same handle: the wallet is the sole ACL subject and
      // the value's domain, so its pubkey is both the signer identity and the allowed domain key.
      const decrypted = await runSolanaCurrentUserDecrypt({
        UD_RELAYER_URL: config.relayerUrl,
        UD_RPC_URL: config.rpcUrl,
        UD_PROOF_SERVICE_URL: config.proofServiceUrl,
        UD_CONTRACTS_CHAIN_ID: config.chainId.toString(),
        UD_HANDLE: hex(result.handle),
        UD_SECRET_KEY: secretKey,
        UD_CONTEXT_ID: solanaUserDecryptContext(config.userDecryptContextId),
        UD_EPOCH_ID: config.kmsEpochId,
        UD_ALLOWED_DOMAIN_KEYS: walletHex,
        UD_ACL_VALUE_KEY: hex(result.target.encryptedValueId),
        UD_VERIFYING_PROGRAM_ID: config.verifyingProgramId,
        UD_KMS_SIGNERS: config.kmsSigners.join(","),
        UD_FHE_PARAMETER: config.fheParameter,
        UD_GATEWAY_CHAIN_ID: config.gatewayChainId,
        UD_GATEWAY_DECRYPTION_CONTRACT: config.gatewayDecryptionContract,
        UD_EXPECTED: "42",
      });
      expect(decrypted).toBe(42n);
    },
    SCENARIO_TIMEOUT_MS,
  );

  test(
    "historical decrypt: update the value, then user-decrypt the OLD handle via a live MMR access proof",
    async () => {
      const { stack, context, wallet, config, secretKey, walletHex } = await verticalSetup();
      const label = paddedLabel("vertical-historical");

      const original = await trivialEncryptPersistent(context, { payer: wallet.signer, value: 42n, label });
      // The old handle must be SNS-committed before the update buries it.
      await stack.waitForSnsCommit(hex(original.handle));

      const updated = await trivialEncryptPersistent(context, { payer: wallet.signer, value: 7n, label });
      // The update really replaced the current handle, and the encrypted value account kept its
      // identity (the bash asserted `aclValueKey` stable across the update).
      expect(hex(updated.handle)).not.toBe(hex(original.handle));
      expect(hex(updated.target.encryptedValueId)).toBe(hex(original.target.encryptedValueId));

      const proof = await fetchHistoricalAccessProof(config, {
        encryptedValue: original.target.encryptedValue,
        oldHandle: original.handle,
        subject: wallet.signer.address,
      });
      await historicalUserDecryptExpect(context, config, {
        target: original.target,
        oldHandle: original.handle,
        subject: wallet.signer.address,
        secretKey,
        allowedDomainKey: walletHex,
        expected: 42n,
        proof,
      });
    },
    SCENARIO_TIMEOUT_MS,
  );

  test(
    "input flow: fhe_execute VerifiedInput(56) + scalar 2 -> persistent under the attested domain -> public-decrypt == 58",
    async () => {
      const { env, stack, context, wallet, config, walletHex } = await verticalSetup();
      const INPUT_VALUE = 56n;
      const ADDEND = 2n;

      // Real ZK input proof through the public SDK encrypt client, in-process (the same pattern
      // the deposit-arc scenario proved under bun). The wallet is both the attested user and the
      // attested contract identity: the attested contract IS the input's ACL domain, and the
      // persistent output below must bind exactly that domain (enforced on-chain).
      const submission = await submitUint64InputProof({
        chainId: config.chainId,
        relayerUrl: config.relayerUrl,
        domainKey: walletHex,
        aclProgramAddress: env.aclProgram,
        contractAddress: walletHex,
        userAddress: walletHex,
        value: INPUT_VALUE,
      });
      const inputHandle = hexToBytes(submission.handles[0].bytes32Hex);
      expect(inputHandle).toHaveLength(32);

      // Output label keyed on the input handle tail so distinct inputs derive distinct records
      // (the live-client's scheme, with a readable prefix instead of a bare marker byte).
      const label = paddedLabel("vertical-input");
      label.set(inputHandle.subarray(24, 32), 24);
      const target = await persistentValueTarget(wallet.signer.address, wallet.signer.address, label);
      const dictionary = new ExecutionDictionary();
      const output = await persistentOutput(context, dictionary, {
        target,
        encryptedValueIndex: 0,
        subjects: [wallet.signer.address],
      });
      await sendFheExecute(context, {
        payer: wallet.signer,
        dictionary,
        steps: [
          {
            __kind: "Binary",
            op: FheBinaryOpCode.Add,
            lhs: {
              __kind: "VerifiedInput",
              attestation: {
                inputHandle,
                ctHandles: submission.handles.map((handle) => hexToBytes(handle.bytes32Hex)),
                handleIndex: 0,
                userAddress: hexToBytes(walletHex),
                contractAddress: hexToBytes(walletHex),
                contractChainId: config.chainId,
                extraData: hexToBytes(submission.extraData),
                signatures: submission.signatures.map((signature) => hexToBytes(signature)),
              },
            },
            rhs: { __kind: "Scalar", valueIndex: dictionary.intern(scalarBytes(ADDEND)) },
            outputFheType: FHE_TYPE.euint64,
            output,
          },
        ],
        remainingAccounts: [{ address: target.encryptedValue, writable: true }],
      });

      const handle = await currentHandle(context, target.encryptedValue);
      const { cleartext } = await releaseAndDecrypt(context, config, stack, {
        payer: wallet.signer,
        result: { target, handle },
      });
      expect(cleartext).toBe(INPUT_VALUE + ADDEND);
    },
    SCENARIO_TIMEOUT_MS,
  );
});
