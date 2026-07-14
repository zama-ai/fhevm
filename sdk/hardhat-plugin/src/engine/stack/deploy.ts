import { type Eip1193Provider, FhevmNode } from "../node";
import { ADDRESSES, type HostContractName } from "./addresses";
import { ALL_CONTRACTS, getCleartextArtifact } from "./artifacts";
import {
  ACL_OWNER,
  type EngineSigners,
  GATEWAY_CHAIN_ID,
  GATEWAY_DECRYPTION_ADDRESS,
  GATEWAY_INPUT_VERIFICATION_ADDRESS,
  HCU_CAP_PER_BLOCK,
  HCU_MAX_DEPTH_PER_TX,
  HCU_MAX_PER_TX,
  INPUT_VERIFIER_THRESHOLD,
  KMS_THRESHOLD,
  KMS_TX_SENDER,
  defaultSigners,
} from "./config";
import { readCleartext, sendCleartext } from "./rpc";

export interface DeployedEngine {
  readonly node: FhevmNode;
  readonly signers: EngineSigners;
  readonly addresses: typeof ADDRESSES;
}

/**
 * Places the cleartext host stack on the chain.
 *
 * WHY setCode RATHER THAN A REAL DEPLOY. host-contracts-cleartext ships a full deployment library
 * (`deploy()` in its `./ts` entry) that stands up real ERC-1967 proxies and materializes them atomically
 * through a standing `ACLOwner`. We cannot use it: it deploys with CREATE, so every address is derived from
 * the deployer's nonce, while `ZamaConfig._getLocalConfig()` pins ACL / FHEVMExecutor / KMSVerifier to
 * fixed addresses that are compiled into the user's contracts (see addresses.ts). No choice of deployer or
 * nonce lands on them.
 *
 * So we write the implementations' runtime code straight to the pinned addresses and run the same
 * initializers the real deploy would. Two things make that sound:
 *
 *  - The contracts find each other. Their cross-references are compile-time constants
 *    (`config/addresses.sol`), which the templates expose as patchable byte offsets. `artifacts.ts`
 *    rewrites every one to our address map, so the stack is internally consistent at whatever addresses we
 *    pick.
 *  - The initializers still run. `initializeFromEmptyProxy` is guarded by `onlyFromEmptyProxy`, which
 *    requires `_getInitializedVersion() == 1` — the state a real `EmptyUUPSProxy.initialize()` would leave
 *    behind. We write that slot directly instead.
 *
 * What we give up is the proxy machinery itself (upgradeability, the `ACLOwner` admin). A test mock never
 * upgrades, so this costs nothing and skips ~20 transactions per test run.
 */
export async function deployCleartextEngine(provider: Eip1193Provider): Promise<DeployedEngine> {
  const node = await FhevmNode.create(provider);
  const signers = defaultSigners();

  // The initializers are sent from ACL_OWNER, so it must exist as a spendable, impersonated account.
  await node.setBalance(ACL_OWNER, 10n ** 20n);
  await node.impersonate(ACL_OWNER);

  // 1. Place every contract's runtime code, cross-references already patched to our address map.
  for (const name of ALL_CONTRACTS) {
    await node.setCode(ADDRESSES[name], getCleartextArtifact(name).deployedBytecode);
  }

  // 2. Fake the post-`EmptyUUPSProxy.initialize()` state that `onlyFromEmptyProxy` checks for. PauserSet is
  //    immutable (kind "non-proxy") and has no initializer, so it is skipped.
  for (const name of ALL_CONTRACTS) {
    if (getCleartextArtifact(name).kind === "proxy") {
      await node.setInitializableStorage(ADDRESSES[name], 1n, false);
    }
  }

  // 3. ACL's owner must exist BEFORE its initializer runs: `ACL.initializeFromEmptyProxy()` takes no owner
  //    argument, it calls `__Ownable_init(owner())` and so preserves whatever is already in the slot. It is
  //    also the ownership root for the whole stack — `ACLOwnable` resolves every other contract's owner
  //    through `ACL.owner()`.
  await node.setOwnableStorage(ADDRESSES.ACL, ACL_OWNER);

  // 4. Run the initializers. ProtocolConfig goes first: it holds the KMS signer set and thresholds that
  //    KMSVerifier and KMSGeneration read back through it.
  //    The node entry carries MPC connection metadata (partyId, mpcIdentity, caCert, storagePrefix) and the
  //    initializer takes a software version and enclave PCR values — none of which the cleartext stack
  //    consults, so they are present-but-empty rather than meaningful.
  await sendCleartext(node, ACL_OWNER, "ProtocolConfig", "initializeFromEmptyProxy", [
    [[KMS_TX_SENDER, signers.kms.address, "127.0.0.1", "https://kms.local", 1, "kms-1", "0x", ""]],
    [KMS_THRESHOLD, KMS_THRESHOLD, KMS_THRESHOLD, KMS_THRESHOLD],
    "0.0.0-mock",
    [],
  ]);

  await sendCleartext(node, ACL_OWNER, "ACL", "initializeFromEmptyProxy");
  await sendCleartext(node, ACL_OWNER, "FHEVMExecutor", "initializeFromEmptyProxy");
  await sendCleartext(node, ACL_OWNER, "KMSGeneration", "initializeFromEmptyProxy");

  // The gateway identity these two are initialized with is the domain we must later SIGN against — it is
  // an init argument, not a baked-in constant. See config.ts.
  await sendCleartext(node, ACL_OWNER, "KMSVerifier", "initializeFromEmptyProxy", [
    GATEWAY_DECRYPTION_ADDRESS,
    GATEWAY_CHAIN_ID,
  ]);
  await sendCleartext(node, ACL_OWNER, "InputVerifier", "initializeFromEmptyProxy", [
    GATEWAY_INPUT_VERIFICATION_ADDRESS,
    GATEWAY_CHAIN_ID,
    [signers.coprocessor.address],
    INPUT_VERIFIER_THRESHOLD,
  ]);

  await sendCleartext(node, ACL_OWNER, "HCULimit", "initializeFromEmptyProxy", [
    HCU_CAP_PER_BLOCK,
    HCU_MAX_DEPTH_PER_TX,
    HCU_MAX_PER_TX,
  ]);

  // 5. The cleartext layer. CleartextDB's initial writer is CleartextArithmetic: the executor delegates
  //    every op to the arithmetic contract, which is the only account allowed to persist plaintexts.
  await sendCleartext(node, ACL_OWNER, "CleartextArithmetic", "initializeFromEmptyProxy");
  await sendCleartext(node, ACL_OWNER, "CleartextDB", "initializeFromEmptyProxy", [ADDRESSES.CleartextArithmetic]);

  await assertEngineReady(node, signers);
  return { node, signers, addresses: ADDRESSES };
}

/**
 * Verifies a placed stack is actually wired up, rather than merely present. Used both after a fresh deploy
 * and to decide whether an already-populated chain can be adopted as-is.
 */
export async function assertEngineReady(node: FhevmNode, signers: EngineSigners): Promise<void> {
  const [kmsSigners] = await readCleartext(node, "KMSVerifier", "getKmsSigners");
  const configured = (kmsSigners as string[]).map((s) => s.toLowerCase());
  if (!configured.includes(signers.kms.address.toLowerCase())) {
    throw new Error(
      `The cleartext engine is not initialized: KMSVerifier.getKmsSigners() does not include the mock KMS signer ` +
        `(got [${configured.join(", ")}]).`,
    );
  }

  // The executor persists plaintexts only through CleartextArithmetic; if the DB does not recognise it as a
  // writer, every FHE op silently records nothing and decryption returns zeros.
  const [isWriter] = await readCleartext(node, "CleartextDB", "isWriter", [ADDRESSES.CleartextArithmetic]);
  if (isWriter !== true) {
    throw new Error("The cleartext engine is not initialized: CleartextDB does not have CleartextArithmetic as a writer.");
  }
}

/** True when `name`'s placed runtime code is already exactly what we would write. */
export async function isPlaced(node: FhevmNode, name: HostContractName): Promise<boolean> {
  return (await node.getCode(ADDRESSES[name])) === getCleartextArtifact(name).deployedBytecode;
}
