// Parity case: A holds `fund` confidential tokens, pays B `amount`, and lets C read A's balance.
// B may read its own balance but not A's. A transfer above the balance moves nothing on both
// chains: EncryptedERC20 selects 0 when the balance is short, and so does the confidential-token
// program (solana/docs/INVARIANTS.md, `mollusk_overdrawn_confidential_transfer_succeeds_and_moves_an_encrypted_zero`).

import { SolanaUserDecryptRunError } from "@fhevm/sdk/solana";
import { parseAbi, type Address, type Hex } from "viem";

import { generateSolanaKeypair } from "../../src/solana/provision";
import { createRealTwoHolderDependencies, type Holder, type TwoHolderScenario } from "../../src/solana/two-holder-transfer";
import { withHostReachableFetch } from "../../src/utils/fs";
import { loadEnv } from "../harness";
import { ensureUp } from "../harness/solana/stack";
import { openEvmHost, type EvmAccount, type EvmHost } from "./evm";
import { parityCase, type RefusalReason } from "./parity";

const STEP = {
  ownerReadsOwn: "A:balance(A)",
  payeeReadsOwn: "B:balance(B)",
  delegateReadsOwner: "C for A:balance(A)",
  payeeReadsOwner: "B:balance(A)",
} as const;

/** How long the grant to C lives, past the host's clock. */
const GRANT_SECONDS = 3_600n;
const RELAYER_NOT_ALLOWED = "not_allowed_on_host_acl";

const ACL_ABI = parseAbi([
  "function delegateForUserDecryption(address delegate, address contractAddress, uint64 expirationDate)",
]);

const evmRefusal: RefusalReason = (error) => {
  // The SDK reads the ACL before it asks the relayer.
  if (error instanceof Error && error.name === "AclUserDecryptionError") return "SDK ACL check: AclUserDecryptionError";
  const label = (error as { relayerApiError?: { label?: string } } | null)?.relayerApiError?.label;
  return label === RELAYER_NOT_ALLOWED ? `relayer: ${label}` : undefined;
};

const solanaRefusal: RefusalReason = (error) => {
  if (!(error instanceof SolanaUserDecryptRunError)) return undefined;
  const { rejection } = error;
  if ("label" in rejection && rejection.label === RELAYER_NOT_ALLOWED) return `relayer: ${rejection.label}`;
  return rejection.kind === "unanswered" ? `unanswered after ${error.attempts} attempt(s)` : undefined;
};

let evmHost: Promise<EvmHost> | undefined;

/** `reader` user-decrypts `handle` of `token`; `delegator` names whose access a delegate uses. */
const evmDecrypt = (host: EvmHost, token: Address, reader: EvmAccount, handle: Hex, delegator?: EvmAccount) => async () => {
  const transportKeyPair = await host.fhevm.generateTransportKeyPair();
  const signedPermit = await host.fhevm.signLegacyDecryptionPermit({
    contractAddresses: [token],
    durationSeconds: 86_400,
    startTimestamp: Math.floor(Date.now() / 1000),
    transportKeyPair,
    signer: reader,
    signerAddress: reader.account.address,
    ...(delegator === undefined ? {} : { delegatorAddress: delegator.account.address }),
  });
  const { value } = await host.fhevm.decryptValue({ contractAddress: token, transportKeyPair, signedPermit, encryptedValue: handle });
  return BigInt(value as bigint | number | boolean);
};

parityCase<{ fund: bigint; amount: bigint }>({
  name: "token transfer and delegated read",
  cases: [
    { fund: 1000n, amount: 400n },
    { fund: 1000n, amount: 0n },
    { fund: 1000n, amount: 1000n },
    { fund: 1000n, amount: 1001n },
  ],
  expect: ({ fund, amount }) => {
    const moved = amount <= fund ? amount : 0n;
    return [
      { step: STEP.ownerReadsOwn, is: fund - moved },
      { step: STEP.payeeReadsOwn, is: moved },
      { step: STEP.delegateReadsOwner, is: fund - moved },
      { step: STEP.payeeReadsOwner, is: "denied" },
    ];
  },
  // A deploys the token, so the owner-only mint is A's; A then transfers and delegates to C.
  evm: async ({ fund, amount }, reads) => {
    const host = await (evmHost ??= openEvmHost(loadEnv()));
    const [a, b, c] = [0, 1, 2].map(host.account) as [EvmAccount, EvmAccount, EvmAccount];
    const token = await host.deploy(a, "EncryptedERC20.sol", "EncryptedERC20", ["Parity", "PAR"]);
    const send = async (from: EvmAccount, functionName: string, args: readonly unknown[]) =>
      host.confirm(await from.writeContract({ address: token.address, abi: token.abi, functionName, args }));
    await send(a, "mint", [fund]);
    const input = await withHostReachableFetch(() =>
      host.fhevm.encryptValue({
        value: { type: "uint64", value: amount },
        contractAddress: token.address,
        userAddress: a.account.address,
      }),
    );
    await send(a, "transfer", [b.account.address, input.encryptedValue, input.inputProof]);
    const hostTime = (await host.publicClient.getBlock()).timestamp;
    await host.confirm(
      await a.writeContract({
        address: host.aclAddress,
        abi: ACL_ABI,
        functionName: "delegateForUserDecryption",
        args: [c.account.address, token.address, hostTime + GRANT_SECONDS],
      }),
    );
    const balanceOf = async (owner: EvmAccount) =>
      (await host.publicClient.readContract({ address: token.address, abi: token.abi, functionName: "balanceOf", args: [owner.account.address] })) as Hex;
    const balanceA = await balanceOf(a);
    const balanceB = await balanceOf(b);
    await reads.value(STEP.ownerReadsOwn, evmDecrypt(host, token.address, a, balanceA));
    await reads.value(STEP.payeeReadsOwn, evmDecrypt(host, token.address, b, balanceB));
    await reads.value(STEP.delegateReadsOwner, evmDecrypt(host, token.address, c, balanceA, a));
    await reads.denied(STEP.payeeReadsOwner, evmDecrypt(host, token.address, b, balanceA), evmRefusal);
  },
  // A and B are fresh run wallets, swept back after the case; C only signs decrypt permits.
  solana: async ({ fund, amount }, reads) => {
    const env = loadEnv();
    const stack = await ensureUp(env);
    const holders = createRealTwoHolderDependencies({
      rpcUrl: env.rpcUrl,
      wsUrl: env.wsUrl,
      relayerUrl: env.relayerUrl,
      hostRpcUrl: env.hostRpcUrl,
      gatewayRpcUrl: env.gatewayRpcUrl,
      aclProgram: env.aclProgram,
      funding: env.funding,
      funderKeypairPath: env.capabilities.faucet ? undefined : env.roots.deployerKeypairPath,
      // Unused here: the reads below retry until the ciphertext is decryptable.
      waitForHandle: stack.waitForSnsCommit,
      userDecryptContextId: env.userDecryptContextId,
    });
    let scenario: TwoHolderScenario | undefined;
    try {
      scenario = await holders.provision(fund);
      const { alice, bob } = scenario;
      await holders.transfer(scenario, await holders.readBalance(scenario, alice), await holders.readBalance(scenario, bob), amount);
      const carol = await generateSolanaKeypair();
      await holders.grantDecryption(scenario, carol.signer.address);
      const delegate: Holder = { owner: carol.signer.address, secretKey: `0x${Buffer.from(carol.bytes.subarray(0, 32)).toString("hex")}` };
      const balanceA = await holders.readBalance(scenario, alice);
      const balanceB = await holders.readBalance(scenario, bob);
      await reads.value(STEP.ownerReadsOwn, () => holders.decryptValue(alice, balanceA));
      await reads.value(STEP.payeeReadsOwn, () => holders.decryptValue(bob, balanceB));
      await reads.value(STEP.delegateReadsOwner, () => holders.decryptValue(delegate, balanceA, alice.owner));
      // The request names the real owner, so it is refused on access rather than on a foreign owner.
      await reads.denied(STEP.payeeReadsOwner, () => holders.decryptValue(bob, balanceA, alice.owner), solanaRefusal);
    } finally {
      await holders.cleanup(scenario);
    }
  },
  timeoutMs: 20 * 60_000,
});
