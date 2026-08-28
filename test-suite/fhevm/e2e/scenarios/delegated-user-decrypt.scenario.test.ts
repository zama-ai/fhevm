// Scenario: delegated user-decrypt — the #1690 evidence pack, live.
//
// Two arcs over the same protocol surface:
//
//   [headless]  A wallet delegator: grant through the SDK builders -> the *delegate* decrypts the
//               delegator's value (permit signed by the delegate, entry subject = delegator) ->
//               identical repeat coalesces into one relayer job -> revoke -> the next request is
//               terminally refused. The revocation lever is the delegator's; the delegate's own
//               permit stays valid throughout.
//
//   [squads]    A program-controlled delegator — the real Squads v4 program at its mainnet id: a
//               2-of-3 multisig votes a proposal whose inner instruction is the delegation grant,
//               with the multisig's vault PDA as the delegator. Below the threshold the execution
//               refuses (one member cannot grant); at quorum the vault `invoke_signed`s the grant.
//               The delegate then decrypts the DAO's value; a second proposal revokes it.
//
// What the connector authorizes is identical in both arcs — a live delegation row for
// (delegator, delegate, encrypted value account authority) — which is the point: a record
// granted by a vault PDA is indistinguishable from a wallet's.

import { describe, expect, test } from "bun:test";
import { Connection } from "@solana/web3.js";
import { getAddressEncoder, type Address, type Instruction, type TransactionSigner } from "@solana/kit";
import type { SolanaDecryptTrust } from "@sdk-src/solana/index.js";

import { paddedLabel, trivialEncryptPersistent } from "../../src/solana/fhe-vertical";
import { runSolanaCurrentUserDecrypt } from "../../src/solana/current-user-decrypt";
import { generateSolanaKeypair } from "../../src/solana/provision";
import { squadsGenesisExtras } from "../../src/solana/squads";
import { solanaUserDecryptContext } from "../../src/solana/two-holder-transfer";
import {
  approveProposal,
  assertSquadsDeployed,
  createSquad,
  executeVaultTransaction,
  proposeThroughSquad,
  web3KeypairFromBytes,
} from "../harness/solana/squads";
import { verticalSetup, type VerticalTestSetup } from "../harness/solana/vertical";

// Each arc does its own compute + SNS commit wait (up to ~3min) + KMS round-trips, plus the
// governance transactions in the squads arc.
const SCENARIO_TIMEOUT_MS = 15 * 60_000;

const hex = (bytes: Uint8Array): string => `0x${Buffer.from(bytes).toString("hex")}`;
const addressHex = (address: Address): string => hex(new Uint8Array(getAddressEncoder().encode(address)));

/** How far beyond the current slot every grant here lives. Hours of localnet, minutes of test. */
const EXPIRATION_SLOTS_AHEAD = 100_000n;

// The literal specifier stays opaque to tsc (the suite-wide pattern): CI type-checks against
// the SDK *sources* via the `@sdk-src` alias and never builds the package this resolves to.
type SdkSolanaModule = typeof import("@sdk-src/solana/index.js");
const sdkSolana = async (): Promise<SdkSolanaModule> => {
  const solanaModule = "@fhevm/sdk/solana";
  return (await import(solanaModule)) as SdkSolanaModule;
};

/**
 * The SDK's delegation builders return unsigned instructions whose signer metas are noop
 * placeholders (a Squads proposal renderer never signs). To send one directly with the kit
 * harness, the placeholders of the addresses we actually hold keys for are swapped for the
 * real signers.
 */
const withSigners = (instruction: Instruction, signers: readonly TransactionSigner[]): Instruction => ({
  ...instruction,
  accounts: instruction.accounts?.map((account) => {
    const signer = signers.find((candidate) => candidate.address === account.address);
    return signer !== undefined && "signer" in account ? { ...account, signer } : account;
  }),
});

/** The env-shaped inputs of one delegated decrypt: the delegate signs, the subject names whose. */
const delegatedDecryptEnvironment = (
  setup: VerticalTestSetup,
  params: {
    readonly handle: Uint8Array;
    readonly encryptedValueId: Uint8Array;
    readonly delegateSecretKey: string;
    readonly subjectHex: string;
    readonly domainHex: string;
  },
): Record<string, string> => ({
  UD_RELAYER_URL: setup.config.relayerUrl,
  UD_RPC_URL: setup.config.rpcUrl,
  UD_PROOF_SERVICE_URL: setup.config.proofServiceUrl,
  UD_CONTRACTS_CHAIN_ID: setup.config.chainId.toString(),
  UD_HANDLE: hex(params.handle),
  UD_SECRET_KEY: params.delegateSecretKey,
  UD_SUBJECT: params.subjectHex,
  UD_CONTEXT_ID: solanaUserDecryptContext(setup.config.userDecryptContextId),
  UD_EPOCH_ID: setup.config.kmsEpochId,
  UD_ALLOWED_DOMAIN_KEYS: params.domainHex,
  UD_ACL_VALUE_KEY: hex(params.encryptedValueId),
  UD_VERIFYING_PROGRAM_ID: setup.config.verifyingProgramId,
  UD_KMS_SIGNERS: setup.config.kmsSigners.join(","),
  UD_FHE_PARAMETER: setup.config.fheParameter,
  UD_GATEWAY_CHAIN_ID: setup.config.gatewayChainId,
  UD_GATEWAY_DECRYPTION_CONTRACT: setup.config.gatewayDecryptionContract,
  UD_EXPECTED: "42",
});

const currentSlot = async (setup: VerticalTestSetup): Promise<bigint> =>
  await setup.context.rpc.getSlot({ commitment: "confirmed" }).send();

// The Squads fixtures are fetched, not committed (no executable binaries in the repo); when they
// are absent the validator booted without the program and the squads arc cannot run.
const squadsAvailable = (await squadsGenesisExtras()) !== undefined;

describe("solana delegated user-decrypt", () => {
  test(
    "[headless] grant -> delegate decrypts 42 -> identical repeat is one job -> revoke -> refused",
    async () => {
      const setup = await verticalSetup();
      const { stack, context, wallet, config, walletHex } = setup;
      const solana = await sdkSolana();

      // Three distinct roles, because the host program refuses any overlap in the delegation
      // tuple (delegator, delegate, authority): the provisioning wallet is the value's
      // *authority* (it encrypts), the delegator is a separate keypair that merely *owns* the
      // value (a subject is named, not signed), and the delegate holds no access of its own —
      // its key only signs the decrypt permits below.
      const delegator = await generateSolanaKeypair();
      await context.airdropSol(delegator.signer.address, 5n);
      const delegate = await generateSolanaKeypair();
      const delegateSecretKey = hex(delegate.bytes.subarray(0, 32));

      // The delegator's value: 42, owned by the delegator alone.
      const result = await trivialEncryptPersistent(context, {
        payer: wallet.signer,
        value: 42n,
        label: paddedLabel("delegated-headless"),
        subjects: [delegator.signer.address],
      });
      await stack.waitForSnsCommit(hex(result.handle));

      // Grant: delegator -> delegate, scoped to the value's authority (the encrypting wallet).
      const grant = await solana.buildDelegateForUserDecryptionInstruction({
        payer: delegator.signer.address,
        delegator: delegator.signer.address,
        delegate: delegate.signer.address,
        encryptedValueAccountAuthority: wallet.signer.address,
        expirationSlot: (await currentSlot(setup)) + EXPIRATION_SLOTS_AHEAD,
      });
      await context.sendTransaction(delegator.signer, [withSigners(grant, [delegator.signer])]);

      // The rows the connector will read, checked the way a dapp would before paying for a job.
      const rows = await solana.fetchSolanaUserDecryptionDelegation(context.rpc, {
        delegator: delegator.signer.address,
        delegate: delegate.signer.address,
        encryptedValueAccountAuthority: wallet.signer.address,
      });
      expect(rows.exact).not.toBeNull();
      expect(solana.isSolanaUserDecryptionDelegationLiveAt(rows.exact!, await currentSlot(setup))).toBe(true);

      // The delegate decrypts the delegator's value: the permit is the delegate's, the entry
      // subject is the delegator.
      const environment = delegatedDecryptEnvironment(setup, {
        handle: result.handle,
        encryptedValueId: result.target.encryptedValueId,
        delegateSecretKey,
        subjectHex: addressHex(delegator.signer.address),
        domainHex: walletHex,
      });
      expect(await runSolanaCurrentUserDecrypt(environment)).toBe(42n);

      // Dedup: the same session, the same bytes, twice — the relayer coalesces them into one
      // job. Asserted at the wire: both POST bodies are byte-identical and both answers carry
      // the same job id. The interception passes everything through untouched.
      type Bytes32Hex = SolanaDecryptTrust["kmsContextId"];
      const chain = solana.defineFhevmSolanaChain({
        id: BigInt(config.chainId),
        fhevm: {
          relayerUrl: config.relayerUrl,
          acl: { domainKeys: [walletHex as Bytes32Hex] },
          rpcUrl: config.rpcUrl,
          proofServiceUrl: config.proofServiceUrl,
          verifyingProgramId: config.verifyingProgramId as Bytes32Hex,
        },
      });
      solana.setFhevmRuntimeConfig({ auth: { type: "ApiKeyHeader", value: "local" } });
      const trust: SolanaDecryptTrust = {
        kmsSigners: config.kmsSigners.map((address, index) => ({ partyId: index + 1, address })),
        kmsContextId: solanaUserDecryptContext(config.userDecryptContextId) as SolanaDecryptTrust["kmsContextId"],
        kmsEpochId: config.kmsEpochId as SolanaDecryptTrust["kmsEpochId"],
        fheParameter: config.fheParameter,
        gatewayEip712Domain: {
          name: "Decryption",
          version: "1",
          chainId: BigInt(config.gatewayChainId),
          verifyingContract: config.gatewayDecryptionContract,
        } as SolanaDecryptTrust["gatewayEip712Domain"],
      };
      const client = solana.createFhevmDecryptClient({ chain, trust });
      const session = await client.signPermit({
        wallet: solana.solanaPermitWalletFromSecretKey(delegate.bytes.subarray(0, 32)),
        durationSeconds: 3_600n,
      });
      const submissions: { body: string; jobId: string }[] = [];
      const realFetch = globalThis.fetch;
      globalThis.fetch = (async (input: RequestInfo | URL, init?: RequestInit) => {
        const url = String(input instanceof Request ? input.url : input);
        const response = await realFetch(input as never, init);
        if (url.includes("/v3/user-decrypt") && (init?.method ?? "GET") === "POST") {
          const answer = (await response.clone().json()) as { result?: { jobId?: string } };
          submissions.push({ body: String(init?.body), jobId: answer.result?.jobId ?? "<no job id>" });
        }
        return response;
      }) as typeof fetch;
      try {
        const entries = [
          {
            handle: result.handle,
            encryptedValueId: result.target.encryptedValueId,
            subject: new Uint8Array(getAddressEncoder().encode(delegator.signer.address)),
          },
        ];
        const first = await client.userDecrypt({ session, entries });
        const second = await client.userDecrypt({ session, entries });
        expect(BigInt(first[0]!.value as bigint)).toBe(42n);
        expect(BigInt(second[0]!.value as bigint)).toBe(42n);
      } finally {
        globalThis.fetch = realFetch;
      }
      expect(submissions).toHaveLength(2);
      expect(submissions[1]!.body).toBe(submissions[0]!.body);
      expect(submissions[1]!.jobId).toBe(submissions[0]!.jobId);

      // Revoke — the delegator's lever. The delegate's permit is untouched. The record's
      // revoked state is the on-chain fact; the fast client-visible refusal is the relayer's
      // advisory pre-check (`not_allowed_on_host_acl`) — the connector's own terminal
      // rejection has no channel back (see 09-rejection-path-findings).
      const revoke = await solana.buildRevokeDelegationForUserDecryptionInstruction({
        delegator: delegator.signer.address,
        delegate: delegate.signer.address,
        encryptedValueAccountAuthority: wallet.signer.address,
      });
      await context.sendTransaction(delegator.signer, [withSigners(revoke, [delegator.signer])]);
      const revokedRows = await solana.fetchSolanaUserDecryptionDelegation(context.rpc, {
        delegator: delegator.signer.address,
        delegate: delegate.signer.address,
        encryptedValueAccountAuthority: wallet.signer.address,
      });
      expect(revokedRows.exact?.revoked).toBe(true);
      await expect(runSolanaCurrentUserDecrypt(environment)).rejects.toThrow(
        /not_allowed_on_host_acl/,
      );
    },
    SCENARIO_TIMEOUT_MS,
  );

  test.skipIf(!squadsAvailable)(
    "[squads] a 2-of-3 multisig grants through its vault -> delegate decrypts the DAO's 42 -> revokes",
    async () => {
      const setup = await verticalSetup();
      const { stack, context, wallet, walletHex } = setup;
      const solana = await sdkSolana();
      const connection = new Connection(setup.config.rpcUrl, "confirmed");
      // The skip condition above proved only that the fixtures are on disk; this proves the
      // RUNNING validator was booted with them, failing legibly instead of deep in createSquad.
      await assertSquadsDeployed(connection);

      // Three members, threshold two: no single member can grant. Each pays their own fees.
      const memberKeys = [await generateSolanaKeypair(), await generateSolanaKeypair(), await generateSolanaKeypair()];
      const members = memberKeys.map((keypair) => web3KeypairFromBytes(keypair.bytes));
      for (const keypair of memberKeys) {
        await context.airdropSol(keypair.signer.address, 5n);
      }
      const squad = await createSquad(connection, { members, threshold: 2 });
      const vaultAddress = squad.vaultPda.toBase58() as Address;
      // The vault pays the record's rent inside the proposal execution — a member's outer
      // signature never crosses the CPI boundary.
      await context.airdropSol(vaultAddress, 1n);

      const delegate = await generateSolanaKeypair();
      const delegateSecretKey = hex(delegate.bytes.subarray(0, 32));

      // The DAO's value: encrypted by a provisioning wallet, owned by the vault alone.
      const result = await trivialEncryptPersistent(context, {
        payer: wallet.signer,
        value: 42n,
        label: paddedLabel("delegated-squads"),
        subjects: [vaultAddress],
      });
      await stack.waitForSnsCommit(hex(result.handle));

      // The proposal's inner instruction: vault -> delegate, the vault paying its own rent.
      const grant = await solana.buildDelegateForUserDecryptionInstruction({
        payer: vaultAddress,
        delegator: vaultAddress,
        delegate: delegate.signer.address,
        encryptedValueAccountAuthority: wallet.signer.address,
        expirationSlot: (await currentSlot(setup)) + EXPIRATION_SLOTS_AHEAD,
      });
      const grantIndex = await proposeThroughSquad(connection, squad, members[0]!, grant);
      await approveProposal(connection, squad, members[0]!, grantIndex);
      // One approval of three is below the threshold: the execution — and with it the grant —
      // must refuse. This is what makes it the multisig's delegation, not any member's. The
      // Squads program refuses the not-yet-approved proposal, not some transport layer.
      await expect(executeVaultTransaction(connection, squad, members[0]!, grantIndex)).rejects.toThrow(
        /Invalid proposal status/,
      );
      await approveProposal(connection, squad, members[1]!, grantIndex);
      await executeVaultTransaction(connection, squad, members[0]!, grantIndex);

      // The record the quorum produced is a perfectly ordinary delegation row of the vault's.
      const rows = await solana.fetchSolanaUserDecryptionDelegation(context.rpc, {
        delegator: vaultAddress,
        delegate: delegate.signer.address,
        encryptedValueAccountAuthority: wallet.signer.address,
      });
      expect(rows.exact?.delegator).toBe(vaultAddress);

      // The delegate decrypts the DAO's value: subject = the vault PDA.
      const environment = delegatedDecryptEnvironment(setup, {
        handle: result.handle,
        encryptedValueId: result.target.encryptedValueId,
        delegateSecretKey,
        subjectHex: addressHex(vaultAddress),
        domainHex: walletHex,
      });
      expect(await runSolanaCurrentUserDecrypt(environment)).toBe(42n);

      // The DAO takes it back: a second proposal revokes, and the next request is refused.
      const revoke = await solana.buildRevokeDelegationForUserDecryptionInstruction({
        delegator: vaultAddress,
        delegate: delegate.signer.address,
        encryptedValueAccountAuthority: wallet.signer.address,
      });
      const revokeIndex = await proposeThroughSquad(connection, squad, members[0]!, revoke);
      await approveProposal(connection, squad, members[0]!, revokeIndex);
      await approveProposal(connection, squad, members[1]!, revokeIndex);
      await executeVaultTransaction(connection, squad, members[1]!, revokeIndex);
      const revokedRows = await solana.fetchSolanaUserDecryptionDelegation(context.rpc, {
        delegator: vaultAddress,
        delegate: delegate.signer.address,
        encryptedValueAccountAuthority: wallet.signer.address,
      });
      expect(revokedRows.exact?.revoked).toBe(true);
      await expect(runSolanaCurrentUserDecrypt(environment)).rejects.toThrow(
        /not_allowed_on_host_acl/,
      );
    },
    SCENARIO_TIMEOUT_MS,
  );
});
