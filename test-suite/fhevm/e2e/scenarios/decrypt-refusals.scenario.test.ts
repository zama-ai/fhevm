// Scenario: what relayer-http and the KMS Connectors refuse, and who refuses it.
//
// One holder wraps tokens, which writes a balance the holder may user-decrypt. Each case then sends
// one user-decrypt request to relayer-http exactly once (src/solana/relayer-http-request.ts), so it
// sees the answer to its own request and not to a resubmission.
//
//   [answered]           the holder's own request -> 200; the SDK's client decrypts the balance.
//   [duplicate]          the same bytes again -> 200 with byte-identical shares: the Connectors served
//                        the answer they stored instead of decrypting again.
//   [store substitution] the handle named in another store -> 403 acl_denied, from the Connectors.
//   [owner substitution] the entry names another owner -> 403 acl_denied, from the Connectors.
//   [changed field]      a field changed after signing -> 403 user_signature_rejected, from relayer-http.
//   [other program]      a permit signed for another zama-host -> 403 user_signature_rejected, from
//                        relayer-http.
//   [expired]            a permit whose window closed -> 400 malformed, from relayer-http.
//   [revoked]            a permit opened before its signer's `revoke_permits` -> 403
//                        user_signature_rejected, from the Connectors.
//   [proof catch-up]     (local) a balance written while the Merkle indexer is paused -> 403 acl_denied;
//                        once it resumes, the same request is answered.
//
// Who refused is read from relayer-http's log, on the local stack only: a request it refuses itself
// has no `request forwarded` line and no decryption id. On another target the cases assert the status
// and the code.
import { describe, expect, test } from 'bun:test';
import { address, getAddressEncoder, type Address } from '@solana/kit';
import { hexToBytes } from '@fhevm/sdk/base';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';

import {
  type DecryptionCall,
  loggedDecryptionId,
  relayerHttpLines,
  wasForwarded,
} from '../../src/solana/decryption-trace';
import {
  createConfidentialMint,
  createSplMint,
  generateSolanaKeypair,
  initializeConfidentialTokenAccount,
  mintSplTo,
  readTokenBalanceStore,
  wrapUnderlying,
} from '../../src/solana/provision';
import {
  buildRevokePermitsInstruction,
  buildSolanaUserDecryptRequest,
  decodeSolanaPermitFields,
  decryptionChain,
  decryptionClient,
  fetchSolanaPermitInvalidation,
  permitWireFields,
  signSolanaPermit,
  solanaPermitWalletFromSecretKey,
  submitUserDecryptOnce,
  type SolanaUserDecryptHandleEntry,
} from '../../src/solana/relayer-http-request';
import { totalSupplyStore } from '../../src/solana/token-vertical';
import { run } from '../../src/utils/process';
import { until } from '../../src/utils/until';
import { loadEnv } from '../harness/loadEnv';
import { verticalSetup } from '../harness/solana/vertical';

const SCENARIO_TIMEOUT_MS = 20 * 60_000;
const WRAP_AMOUNT = 1000n;
const env = loadEnv();
/** The local stack's container logs say who refused; other targets are asserted on the wire only. */
const readsLogs = env.source === 'local';

const addressBytes = (value: Address): Uint8Array => new Uint8Array(getAddressEncoder().encode(value));

/** The shares of a 200, each serialized, in a stable order. */
const shares = (call: DecryptionCall): string[] =>
  (JSON.parse(call.responseBody) as { result: { result: unknown[] } }).result.result.map((share) => JSON.stringify(share)).sort();

/** relayer-http's log lines for the request behind `call`. */
const linesOf = (call: DecryptionCall) => {
  if (call.requestId === undefined) throw new Error(`a ${call.status} answer carried no x-request-id`);
  return relayerHttpLines(call.requestId);
};

/** Asserts the status and the code, and on the local stack whether relayer-http asked the Connectors. */
const expectRefusal = async (
  call: DecryptionCall,
  expected: { readonly status: number; readonly code: string; readonly refusedBy: 'relayer-http' | 'connectors' },
) => {
  expect({ status: call.status, code: call.code }).toEqual({ status: expected.status, code: expected.code });
  if (!readsLogs) return;
  const lines = await linesOf(call);
  expect(wasForwarded(lines)).toBe(expected.refusedBy === 'connectors');
  expect(loggedDecryptionId(lines) !== undefined).toBe(expected.refusedBy === 'connectors');
};

describe.skipIf(!env.capabilities.protocolServices)('solana decrypt refusals through relayer-http', () => {
  test(
    'each refusal comes back with its code, from the service that refuses it',
    async () => {
      const { stack, context, wallets, wallet, config, secretKey } = await verticalSetup();
      const chain = decryptionChain(config);
      const client = decryptionClient(config);
      const permitWallet = solanaPermitWalletFromSecretKey(hexToBytes(secretKey));
      const submit = (body: unknown) => submitUserDecryptOnce(chain, body);

      // The holder's balance, written by a wrap, which needs no input proof.
      const underlyingMint = await createSplMint(context, { authority: wallet.signer, decimals: 9 });
      await mintSplTo(context, {
        authority: wallet.signer,
        mint: underlyingMint,
        recipient: wallet.signer.address,
        baseUnits: 2n * WRAP_AMOUNT,
      });
      const mint = await createConfidentialMint(context, { authority: wallet.signer, underlyingMint });
      await initializeConfidentialTokenAccount(context, { payer: wallet.signer, owner: wallet.signer.address, mint });
      await wrapUnderlying(context, { owner: wallet.signer, mint, underlyingMint, amount: WRAP_AMOUNT });
      const balance = await readTokenBalanceStore(context, { mint, owner: wallet.signer.address });
      await stack.waitForSnsCommit(balance.currentHandle);

      const entry: SolanaUserDecryptHandleEntry = {
        handle: hexToBytes(balance.currentHandle),
        encryptedStore: addressBytes(address(balance.encryptedStore)),
        ownerAddress: addressBytes(wallet.signer.address),
      };
      const session = await client.signPermit({ wallet: permitWallet, durationSeconds: 3_600n });
      const body = buildSolanaUserDecryptRequest({ signedPermit: session.signedPermit, entries: [entry] });

      // [answered] The request every case below varies is one the stack answers.
      const [decrypted] = await client.decryptValues({ session, entries: [entry] });
      expect(BigInt(decrypted!.value as bigint)).toBe(WRAP_AMOUNT);
      const answered = await submit(body);
      expect(answered.status).toBe(200);

      // [duplicate] ML-KEM encapsulation is randomized, so two signcryptions of one share differ.
      // Byte-identical shares therefore mean each Connector served the answer it stored for this
      // decryption id instead of asking its KMS again.
      const repeated = await submit(body);
      expect(repeated.status).toBe(200);
      expect(shares(repeated)).toEqual(shares(answered));
      if (readsLogs) {
        expect(repeated.requestId).not.toBe(answered.requestId);
        expect(loggedDecryptionId(await linesOf(repeated))).toBe(loggedDecryptionId(await linesOf(answered)));
      }

      // [store substitution] The holder's handle, named in the mint's total-supply store.
      const storeSubstituted = buildSolanaUserDecryptRequest({
        signedPermit: session.signedPermit,
        entries: [{ ...entry, encryptedStore: addressBytes(await totalSupplyStore(mint)) }],
      });
      await expectRefusal(await submit(storeSubstituted), { status: 403, code: 'acl_denied', refusedBy: 'connectors' });

      // [owner substitution] The entry names an owner with no allow on the handle and no delegation
      // to the holder.
      const stranger = await generateSolanaKeypair();
      const ownerSubstituted = buildSolanaUserDecryptRequest({
        signedPermit: session.signedPermit,
        entries: [{ ...entry, ownerAddress: addressBytes(stranger.signer.address) }],
      });
      await expectRefusal(await submit(ownerSubstituted), { status: 403, code: 'acl_denied', refusedBy: 'connectors' });

      // [changed field] The validity window grows by one second after the wallet signed it.
      const changed = {
        ...body,
        payload: {
          ...body.payload,
          requestValidity: { ...body.payload.requestValidity, durationSeconds: body.payload.requestValidity.durationSeconds + 1 },
        },
      };
      await expectRefusal(await submit(changed), { status: 403, code: 'user_signature_rejected', refusedBy: 'relayer-http' });

      // [other program] A genuine signature over a permit naming another zama-host program.
      const otherProgram = await signSolanaPermit(
        permitWallet,
        decodeSolanaPermitFields({ ...permitWireFields(session), verifyingProgramId: addressBytes(stranger.signer.address) }),
      );
      await expectRefusal(await submit(buildSolanaUserDecryptRequest({ signedPermit: otherProgram, entries: [entry] })), {
        status: 403,
        code: 'user_signature_rejected',
        refusedBy: 'relayer-http',
      });

      // [expired] A genuine signature over a one-minute window that closed an hour ago. relayer-http
      // checks the window against its own clock before forwarding (endpoint/validate.rs
      // `request_window`), so the Connectors never see it. Their own window check is defense in
      // depth for a caller that reaches them directly; through relayer-http it is unreachable.
      const anHourAgo = (BigInt(Math.floor(Date.now() / 1000)) / 60n - 60n) * 60n;
      const expired = await signSolanaPermit(
        permitWallet,
        decodeSolanaPermitFields({ ...permitWireFields(session), startTimestamp: anHourAgo, durationSeconds: 60n }),
      );
      await expectRefusal(await submit(buildSolanaUserDecryptRequest({ signedPermit: expired, entries: [entry] })), {
        status: 400,
        code: 'malformed',
        refusedBy: 'relayer-http',
      });

      // [revoked] A second wallet signs a permit, then revokes every permit it opened. The Connectors
      // check the revocation before the ACL (kms-worker solana/pipeline.rs), so its request for the
      // holder's handle is refused on the revocation. A permit signed in the minutes after a revoke
      // opens before it too, so the holder's own wallet stays unrevoked for the case below.
      const revoker = await wallets.fresh(env.funding.secondarySol);
      const revokedSession = await client.signPermit({
        wallet: solanaPermitWalletFromSecretKey(revoker.bytes.subarray(0, 32)),
        durationSeconds: 3_600n,
      });
      await (await context.client(revoker.signer)).sendTransaction([
        await buildRevokePermitsInstruction({ user: revoker.signer, programAddress: ZAMA_HOST_PROGRAM_ADDRESS }),
      ]);
      await until(
        async () =>
          // The Connectors revoke a permit whose start is before the watermark (kms-worker watermark.rs).
          (await fetchSolanaPermitInvalidation(context.rpc, revoker.signer.address, { programAddress: ZAMA_HOST_PROGRAM_ADDRESS })) >
          revokedSession.signedPermit.fields.startTimestamp,
        { description: 'the revocation watermark at finalized', timeoutMs: 60_000 },
      );
      const revoked = buildSolanaUserDecryptRequest({
        signedPermit: revokedSession.signedPermit,
        entries: [{ ...entry, ownerAddress: addressBytes(revoker.signer.address) }],
      });
      await expectRefusal(await submit(revoked), { status: 403, code: 'user_signature_rejected', refusedBy: 'connectors' });

      // [proof catch-up] The Connectors prove the holder's allow leaf against the coprocessors' Merkle
      // record, which the indexer writes. A balance written while the indexer is paused has no leaf
      // yet: refused as acl_denied, the code a client retries. Once the indexer resumes and records
      // it, the same request is answered. The indexer is a host process of the local stack.
      if (readsLogs) {
        await run(['pkill', '-STOP', '-f', 'solana_merkle_indexer']);
        try {
          await wrapUnderlying(context, { owner: wallet.signer, mint, underlyingMint, amount: WRAP_AMOUNT });
          const grown = await readTokenBalanceStore(context, { mint, owner: wallet.signer.address });
          await stack.waitForSnsCommit(grown.currentHandle);
          const pending = buildSolanaUserDecryptRequest({
            signedPermit: session.signedPermit,
            entries: [{ ...entry, handle: hexToBytes(grown.currentHandle) }],
          });
          await expectRefusal(await submit(pending), { status: 403, code: 'acl_denied', refusedBy: 'connectors' });
          await run(['pkill', '-CONT', '-f', 'solana_merkle_indexer']);
          await until(async () => (await submit(pending)).status === 200, {
            description: 'the request answered once the indexer recorded the leaf',
            timeoutMs: 120_000,
            intervalMs: 5_000,
          });
        } finally {
          await run(['pkill', '-CONT', '-f', 'solana_merkle_indexer'], { allowFailure: true });
        }
      }
      await wallets.sweep();
    },
    SCENARIO_TIMEOUT_MS,
  );
});
