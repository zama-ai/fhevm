import { recordRunWallet } from "./recovery";

import { address, createSolanaRpcSubscriptions, getAddressEncoder, type Address, type TransactionSigner } from "@solana/kit";

import { bytes32HexFromId } from "./addresses";
import { vaultModule } from "./lazy-modules";
import {
  createConfidentialMint,
  createProvisioningContext,
  createSplMint,
  generateSolanaKeypair,
  hostConfigAddress,
  initializeConfidentialTokenAccount,
  loadKeypairSigner,
  mintSplTo,
  readTokenBalanceStore,
  wrapUnderlying,
  type BalanceStore,
  type SolanaProvisioningContext,
} from "./provision";
import { loadSolanaSdk, readDecryptTrustInputs } from "./target";
import { withHostReachableFetch } from "../utils/fs";
import { timed } from "../utils/timing";

export type { BalanceStore };

export type Holder = { owner: string; secretKey: string };
export type TwoHolderScenario = {
  mint: string;
  underlyingMint: string;
  alice: Holder;
  bob: Holder;
};

/** The stack endpoints and identities the real transfer arc binds to, injected from `loadEnv()`. */
export type TwoHolderConfig = {
  readonly rpcUrl: string;
  readonly wsUrl: string;
  readonly relayerUrl: string;
  readonly gatewayRpcUrl: string;
  readonly hostRpcUrl: string;
  readonly aclProgram: `0x${string}`;
  /**
   * Explicit user-decrypt context override, as an unsigned decimal. When absent, the decrypts use
   * the active KMS context read live from the deployed `ProtocolConfig` — the pair the KMS
   * Connector actually serves.
   */
  readonly userDecryptContextId: string | undefined;
  /** SOL each holder starts with: Alice pays every provisioning rent and the arc's fees, Bob his own account. */
  readonly funding: { readonly primarySol: number; readonly secondarySol: number };
  /** Resolves once a handle's ciphertext is decryptable: the SNS commit on the real stack. */
  readonly waitForHandle: (handle: string) => Promise<void>;
  /** Wallet the holders are funded from by transfer; absent, they are airdropped (local validators). */
  readonly funderKeypairPath: string | undefined;
};

export type TwoHolderDependencies = {
  provision(): Promise<TwoHolderScenario>;
  readBalance(scenario: TwoHolderScenario, holder: Holder): Promise<BalanceStore>;
  waitForHandle(handle: string): Promise<void>;
  transfer(scenario: TwoHolderScenario, alice: BalanceStore, bob: BalanceStore): Promise<void>;
  decrypt(scenario: TwoHolderScenario, holder: Holder, state: BalanceStore, expected: bigint): Promise<bigint>;
  cleanup(scenario: TwoHolderScenario | undefined): Promise<void>;
};

const addressHex = (value: string): `0x${string}` =>
  `0x${Buffer.from(getAddressEncoder().encode(address(value))).toString("hex")}`;

export const createRealTwoHolderDependencies = (cfg: TwoHolderConfig): TwoHolderDependencies => {
  let provisioned: SolanaProvisioningContext | undefined;
  const context = (): SolanaProvisioningContext => {
    if (!provisioned) throw new Error("two-holder transfer: provision() must run first");
    return provisioned;
  };
  // The holders' signers by owner: Alice signs the transfer, and cleanup returns their SOL to the
  // funder on a live cluster.
  const signers = new Map<string, TransactionSigner>();
  let funderAddress: Address | undefined;
  return {
    async provision() {
      const funder = cfg.funderKeypairPath === undefined ? undefined : await loadKeypairSigner(cfg.funderKeypairPath);
      funderAddress = funder?.address;
      provisioned = createProvisioningContext(cfg.rpcUrl, cfg.wsUrl, { funder });
      const context = provisioned;
      // Fresh keypairs; the user-decrypt secret is the 32-byte seed.
      const createHolder = async () => {
        const { signer, bytes } = await generateSolanaKeypair();
        if (funder) await recordRunWallet({ signer, bytes });
        signers.set(signer.address, signer);
        const holder: Holder = { owner: signer.address, secretKey: `0x${Buffer.from(bytes.subarray(0, 32)).toString("hex")}` };
        return { signer, holder };
      };
      const alice = await createHolder();
      const bob = await createHolder();
      // Alice pays every provisioning rent + fee (mints, escrow, wrap, later the transfer itself);
      // Bob only pays his own confidential token account.
      await context.fundSol(alice.signer.address, cfg.funding.primarySol);
      await context.fundSol(bob.signer.address, cfg.funding.secondarySol);

      // The public underlying: a fresh 9-decimals SPL mint with Alice as mint authority, funded
      // well past the 1000 base units the wrap below rotates into her confidential balance.
      const underlyingMint = await createSplMint(context, { authority: alice.signer, decimals: 9 });
      await mintSplTo(context, {
        authority: alice.signer,
        mint: underlyingMint,
        recipient: alice.signer.address,
        baseUnits: 1_000_000n,
      });
      const mint = await createConfidentialMint(context, { authority: alice.signer, underlyingMint });
      await initializeConfidentialTokenAccount(context, { payer: alice.signer, owner: alice.signer.address, mint });
      await wrapUnderlying(context, { owner: alice.signer, mint, underlyingMint, amount: 1000n });
      await initializeConfidentialTokenAccount(context, { payer: bob.signer, owner: bob.signer.address, mint });
      return { mint, underlyingMint, alice: alice.holder, bob: bob.holder };
    },
    async readBalance(scenario, holder) {
      return readTokenBalanceStore(context(), { mint: address(scenario.mint), owner: address(holder.owner) });
    },
    waitForHandle: cfg.waitForHandle,
    async transfer(scenario, alice, bob) {
      if (alice.chainId !== bob.chainId) throw new Error("Alice and Bob balance handles disagree on chain id");
      const owner = signers.get(scenario.alice.owner);
      if (owner === undefined) throw new Error("two-holder transfer: Alice's signer was not provisioned here");
      const [solana, vault, { asBytes32Hex }] = await Promise.all([
        loadSolanaSdk(),
        vaultModule(),
        import("@fhevm/sdk/base"),
      ]);
      const aclProgramAddress = asBytes32Hex(cfg.aclProgram);
      const chain = solana.defineFhevmSolanaChain({
        id: BigInt(alice.chainId),
        fhevm: { relayerUrl: cfg.relayerUrl, programs: { host: { address: aclProgramAddress } } },
      });
      solana.setFhevmRuntimeConfig({ auth: { type: "ApiKeyHeader", value: process.env.ZAMA_FHEVM_API_KEY ?? "local" } });
      const rpc = context().rpc;
      // The attestation binds the amount to (user = owner, contract = the confidential-token
      // program), the contract identity the token requires for a transfer amount.
      const { inputProof } = await withHostReachableFetch(() =>
        solana.createFhevmEncryptClient({ chain, rpc }).encryptValues({
          contractAddress: asBytes32Hex(addressHex(vault.CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS)),
          userAddress: asBytes32Hex(addressHex(owner.address)),
          values: [{ type: "uint64", value: 400n }],
        }),
      );
      await vault.confidentialTransfer(
        { solanaChain: chain, aclProgramAddress },
        {
          rpc,
          rpcSubscriptions: createSolanaRpcSubscriptions(cfg.wsUrl),
          inputProof,
          inputIndex: 0,
          owner,
          feePayer: owner,
          mint: address(scenario.mint),
          underlyingMint: address(scenario.underlyingMint),
          tokenProgram: vault.TOKEN_PROGRAM_ADDRESS,
          fromAccount: address(alice.tokenAccount),
          toAccount: address(bob.tokenAccount),
          toOwner: address(scenario.bob.owner),
          fromStore: address(alice.encryptedStore),
          toStore: address(bob.encryptedStore),
          hostConfig: await hostConfigAddress(),
        },
      );
    },
    decrypt: async (_scenario, holder, state, expected) => {
      // Loaded here, not at the top: `bun test src` runs this module's orchestration test without
      // the SDK installed.
      const { userDecryptExpect } = await import("./fhe-vertical");
      const trust = await readDecryptTrustInputs(cfg);
      return userDecryptExpect(
        {
          rpcUrl: cfg.rpcUrl,
          relayerUrl: cfg.relayerUrl,
          chainId: BigInt(state.chainId),
          userDecryptContextId: cfg.userDecryptContextId ?? trust.kmsContextId.toString(),
          verifyingProgramId: cfg.aclProgram,
          kmsSigners: trust.kmsSigners,
          kmsEpochId: bytes32HexFromId(trust.kmsEpochId),
          fheParameter: "test",
          gatewayChainId: trust.gatewayChainId.toString(),
          gatewayDecryptionContract: trust.decryptionContract,
        },
        {
          // The balance account the probe derived and verified; the Connector reads it and proves
          // the owner's allow leaf itself.
          encryptedStore: address(state.encryptedStore),
          handle: Uint8Array.from(Buffer.from(state.currentHandle.slice(2), "hex")),
          secretKey: holder.secretKey,
          expected,
        },
      );
    },
    async cleanup() {
      // Transfer-funded holders give their unspent SOL back; airdropped ones keep it (it is free).
      if (funderAddress !== undefined && provisioned !== undefined) {
        for (const holder of signers.values()) {
          await provisioned.sweepSol(holder, funderAddress).catch((error: unknown) => {
            console.warn(`sweeping ${holder.address} back to the funder failed: ${String(error)}`);
          });
        }
        signers.clear();
      }
    },
  };
};

/** Runs one real two-holder transfer and proves both current balances through independent SDK decrypts. */
export const runSolanaTwoHolderTransfer = async (dependencies: TwoHolderDependencies) => {
  let scenario: TwoHolderScenario | undefined;
  try {
    const provisioned = await timed("provision two holders (mints, wrap 1000)", () => dependencies.provision());
    scenario = provisioned;
    const initialAlice = await dependencies.readBalance(provisioned, provisioned.alice);
    const initialBob = await dependencies.readBalance(provisioned, provisioned.bob);
    await dependencies.waitForHandle(initialAlice.currentHandle);
    await dependencies.waitForHandle(initialBob.currentHandle);
    await timed("user decrypt alice=1000", () => dependencies.decrypt(provisioned, provisioned.alice, initialAlice, 1000n));
    await timed("user decrypt bob=0", () => dependencies.decrypt(provisioned, provisioned.bob, initialBob, 0n));

    await timed("encrypt + input proof + confidential transfer(400)", () =>
      dependencies.transfer(provisioned, initialAlice, initialBob),
    );
    const finalAlice = await dependencies.readBalance(provisioned, provisioned.alice);
    const finalBob = await dependencies.readBalance(provisioned, provisioned.bob);
    if (finalAlice.currentHandle === initialAlice.currentHandle || finalBob.currentHandle === initialBob.currentHandle) {
      throw new Error("confidential transfer did not rotate both current balance handles");
    }
    await dependencies.waitForHandle(finalAlice.currentHandle);
    await dependencies.waitForHandle(finalBob.currentHandle);
    await timed("user decrypt alice=600", () => dependencies.decrypt(provisioned, provisioned.alice, finalAlice, 600n));
    await timed("user decrypt bob=400", () => dependencies.decrypt(provisioned, provisioned.bob, finalBob, 400n));
    console.log("[two-holder-transfer] Alice=600 Bob=400");
  } finally {
    await dependencies.cleanup(scenario);
  }
};
