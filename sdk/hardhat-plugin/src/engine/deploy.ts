import {
  ACL_OWNER,
  ADDRESSES,
  EIP712_DECRYPTION_DOMAIN_NAME,
  EIP712_DOMAIN_VERSION,
  EIP712_INPUT_VERIFICATION_DOMAIN_NAME,
  GATEWAY_CHAIN_ID,
  GATEWAY_DECRYPTION_ADDRESS,
  GATEWAY_INPUT_VERIFICATION_ADDRESS,
  HCU_CAP_PER_BLOCK,
  HCU_MAX_DEPTH_PER_TX,
  HCU_MAX_PER_TX,
  INPUT_VERIFIER_THRESHOLD,
  KMS_THRESHOLD,
  KMS_TX_SENDER,
  type EngineSigners,
  defaultSigners,
} from "./addresses";
import { type HostContractName, getCleartextArtifact } from "./artifacts";
import { type Eip1193Provider, FhevmNode } from "./node";
import { readCleartext, sendCleartext } from "./rpc";

/** 10_000 ether, as wei — funds the impersonated deployer without pulling in a units helper. */
const DEPLOYER_FUNDING_WEI = 10_000n * 10n ** 18n;

export interface DeployedEngine {
  readonly node: FhevmNode;
  readonly signers: EngineSigners;
  readonly addresses: typeof ADDRESSES;
}

/** Contracts whose runtime code we place before any initialization runs. */
const SET_CODE_ORDER: HostContractName[] = ["FHEVMExecutor", "ACL", "KMSVerifier", "InputVerifier", "HCULimit"];

function assertEq(actual: unknown, expected: unknown, what: string): void {
  const a = String(actual).toLowerCase();
  const e = String(expected).toLowerCase();
  if (a !== e) {
    throw new Error(`FHEVM cleartext deploy: ${what} mismatch.\n  expected: ${expected}\n  actual:   ${actual}`);
  }
}

async function placeCode(node: FhevmNode, name: HostContractName): Promise<void> {
  const artifact = getCleartextArtifact(name);
  const existing = await node.getCode(ADDRESSES[name]);
  if (existing === artifact.deployedBytecode) {
    return; // idempotent: already placed by a previous deploy on this chain
  }
  if (existing !== "0x") {
    throw new Error(`FHEVM cleartext deploy: ${name} address ${ADDRESSES[name]} already holds foreign code.`);
  }
  await node.setCode(ADDRESSES[name], artifact.deployedBytecode);
}

/**
 * The cleartext contracts bake the canonical addresses into their bytecode. If we placed them at the
 * wrong addresses they would silently fail to find one another later, so read the addresses back out of
 * the deployed code and check them. This is also why address discovery needs no `@fhevm/host-contracts`.
 */
async function assertBakedInAddresses(node: FhevmNode): Promise<void> {
  assertEq((await readCleartext(node, "FHEVMExecutor", "getACLAddress"))[0], ADDRESSES.ACL, "FHEVMExecutor.getACLAddress()");
  assertEq(
    (await readCleartext(node, "FHEVMExecutor", "getHCULimitAddress"))[0],
    ADDRESSES.HCULimit,
    "FHEVMExecutor.getHCULimitAddress()",
  );
  assertEq(
    (await readCleartext(node, "FHEVMExecutor", "getInputVerifierAddress"))[0],
    ADDRESSES.InputVerifier,
    "FHEVMExecutor.getInputVerifierAddress()",
  );
  assertEq(
    (await readCleartext(node, "ACL", "getFHEVMExecutorAddress"))[0],
    ADDRESSES.FHEVMExecutor,
    "ACL.getFHEVMExecutorAddress()",
  );
}

/**
 * Deploys the v0.14 cleartext split-set onto an in-process Hardhat network or an anvil node, driving the
 * chain entirely through the provider-neutral {@link FhevmNode} (EIP-1193): `setCode` places the runtime
 * bytecode, storage pokes fake the proxy-initialized state, and `initializeFromEmptyProxy` runs from the
 * impersonated deployer (the node signs — no signing library involved).
 *
 * Ordering is load-bearing in two places:
 *  - `ProtocolConfig` MUST be initialized before `KMSVerifier`, because at v0.14 the KMS signer set lives
 *    in ProtocolConfig and KMSVerifier reads it through there. Reversed, `getKmsSigners()` silently
 *    returns an empty set.
 *  - Every `initializeFromEmptyProxy` must be preceded by `setInitializableStorage(1, false)`.
 */
export async function deployCleartextEngine(
  provider: Eip1193Provider,
  signers: EngineSigners = defaultSigners(),
): Promise<DeployedEngine> {
  const node = await FhevmNode.create(provider);

  for (const name of SET_CODE_ORDER) {
    await placeCode(node, name);
  }

  // The ACL owner sends every initializer transaction.
  await node.impersonate(ACL_OWNER);
  await node.setBalance(ACL_OWNER, DEPLOYER_FUNDING_WEI);

  // ACL and FHEVMExecutor are never initialized. ACL's only required state is its owner, written
  // directly; the executor needs none.
  await node.setOwnableStorage(ADDRESSES.ACL, ACL_OWNER);

  await assertBakedInAddresses(node);
  assertEq((await readCleartext(node, "ACL", "owner"))[0], ACL_OWNER, "ACL.owner()");

  // HCULimit — the 3-arg uint48 init is mandatory at v0.14 and easy to omit; without it every FHE op
  // reverts with HCUBlockLimitExceeded.
  await node.setInitializableStorage(ADDRESSES.HCULimit, 1n, false);
  await sendCleartext(node, ACL_OWNER, "HCULimit", "initializeFromEmptyProxy", [
    HCU_CAP_PER_BLOCK,
    HCU_MAX_DEPTH_PER_TX,
    HCU_MAX_PER_TX,
  ]);

  // PauserSet is immutable and registers no pauser: ACL's requireNotPaused() then passes by default.
  await placeCode(node, "PauserSet");

  // ProtocolConfig — 4-arg at v0.14. Carries the KMS signer set, so it must precede KMSVerifier.
  await placeCode(node, "ProtocolConfig");
  await node.setInitializableStorage(ADDRESSES.ProtocolConfig, 1n, false);
  await sendCleartext(node, ACL_OWNER, "ProtocolConfig", "initializeFromEmptyProxy", [
    [
      {
        txSenderAddress: KMS_TX_SENDER,
        signerAddress: signers.kms.address,
        ipAddress: "",
        storageUrl: "",
        partyId: 1,
        mpcIdentity: "",
        caCert: "0x",
        storagePrefix: "",
      },
    ],
    { publicDecryption: KMS_THRESHOLD, userDecryption: KMS_THRESHOLD, kmsGen: KMS_THRESHOLD, mpc: KMS_THRESHOLD },
    "1",
    [],
  ]);

  await placeCode(node, "KMSGeneration");
  await node.setInitializableStorage(ADDRESSES.KMSGeneration, 1n, false);
  await sendCleartext(node, ACL_OWNER, "KMSGeneration", "initializeFromEmptyProxy", []);

  // KMSVerifier — 2-arg at v0.14 (signers moved to ProtocolConfig).
  await node.setInitializableStorage(ADDRESSES.KMSVerifier, 1n, false);
  await sendCleartext(node, ACL_OWNER, "KMSVerifier", "initializeFromEmptyProxy", [
    GATEWAY_DECRYPTION_ADDRESS,
    GATEWAY_CHAIN_ID,
  ]);

  await node.setInitializableStorage(ADDRESSES.InputVerifier, 1n, false);
  await sendCleartext(node, ACL_OWNER, "InputVerifier", "initializeFromEmptyProxy", [
    GATEWAY_INPUT_VERIFICATION_ADDRESS,
    GATEWAY_CHAIN_ID,
    [signers.coprocessor.address],
    INPUT_VERIFIER_THRESHOLD,
  ]);

  await assertEngineReady(node, signers);

  return { node, signers, addresses: ADDRESSES };
}

/**
 * The M1 gate. Signer sets prove ProtocolConfig was wired before KMSVerifier; the EIP-712 domains prove
 * the cross-chain (gateway) domain parameters landed, which is what input proofs and decryption proofs
 * are signed against.
 */
export async function assertEngineReady(node: FhevmNode, signers: EngineSigners): Promise<void> {
  const kmsSigners = (await readCleartext(node, "KMSVerifier", "getKmsSigners"))[0] as string[];
  if (kmsSigners.length !== 1) {
    throw new Error(`KMSVerifier.getKmsSigners() returned ${kmsSigners.length} signers, expected 1.`);
  }
  assertEq(kmsSigners[0], signers.kms.address, "KMSVerifier.getKmsSigners()[0]");

  const coprocessorSigners = (await readCleartext(node, "InputVerifier", "getCoprocessorSigners"))[0] as string[];
  if (coprocessorSigners.length !== 1) {
    throw new Error(`InputVerifier.getCoprocessorSigners() returned ${coprocessorSigners.length}, expected 1.`);
  }
  assertEq(coprocessorSigners[0], signers.coprocessor.address, "InputVerifier.getCoprocessorSigners()[0]");

  const kmsDomain = await readCleartext(node, "KMSVerifier", "eip712Domain");
  assertEq(kmsDomain.name, EIP712_DECRYPTION_DOMAIN_NAME, "KMSVerifier EIP-712 domain name");
  assertEq(kmsDomain.version, EIP712_DOMAIN_VERSION, "KMSVerifier EIP-712 domain version");
  assertEq(kmsDomain.chainId, GATEWAY_CHAIN_ID, "KMSVerifier EIP-712 domain chainId");
  assertEq(kmsDomain.verifyingContract, GATEWAY_DECRYPTION_ADDRESS, "KMSVerifier EIP-712 verifyingContract");

  const inputDomain = await readCleartext(node, "InputVerifier", "eip712Domain");
  assertEq(inputDomain.name, EIP712_INPUT_VERIFICATION_DOMAIN_NAME, "InputVerifier EIP-712 domain name");
  assertEq(inputDomain.version, EIP712_DOMAIN_VERSION, "InputVerifier EIP-712 domain version");
  assertEq(inputDomain.chainId, GATEWAY_CHAIN_ID, "InputVerifier EIP-712 domain chainId");
  assertEq(inputDomain.verifyingContract, GATEWAY_INPUT_VERIFICATION_ADDRESS, "InputVerifier EIP-712 verifyingContract");
}
