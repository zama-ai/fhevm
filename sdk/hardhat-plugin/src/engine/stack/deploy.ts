// eslint-disable-next-line @typescript-eslint/no-var-requires
import { deployAt } from "@fhevm/host-contracts-cleartext/ts";

import { type Eip1193Provider, FhevmNode } from "../node";
import { ADDRESSES, FIXED_ADDRESSES, type HostContractName } from "./addresses";
import { createAdminSigner, createEthProvider } from "./adapters";
import { type EngineSigners, bootstrapConfig, defaultSigners } from "./config";
import { readCleartext } from "./rpc";

export interface DeployedEngine {
  readonly node: FhevmNode;
  readonly signers: EngineSigners;
  readonly addresses: typeof ADDRESSES;
}

/**
 * Stands up the cleartext stack on the current node.
 *
 * Deployment is NOT this plugin's job — `@fhevm/host-contracts-cleartext` owns it, for every target: a real
 * chain (`deploy`, CREATE-based) or a dev node at fixed addresses (`deployAt`). The plugin only supplies
 * three things the package cannot know: how to reach this node (the adapters), WHERE the stack has to live
 * (the addresses `ZamaConfig` compiles into the contract under test), and what to initialize it with.
 */
export async function deployCleartextEngine(provider: Eip1193Provider): Promise<DeployedEngine> {
  const node = await FhevmNode.create(provider);
  const signers = defaultSigners();

  await deployAt({
    ethProvider: createEthProvider(node),
    admin: await createAdminSigner(node),
    addresses: FIXED_ADDRESSES,
    config: bootstrapConfig(signers),
  });

  await assertEngineReady(node, signers);
  return { node, signers, addresses: ADDRESSES };
}

/**
 * Verifies a placed stack is actually wired up, rather than merely present. Used after a deploy, and to
 * decide whether an already-populated chain can be adopted as-is.
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
    throw new Error(
      "The cleartext engine is not initialized: CleartextDB does not have CleartextArithmetic as a writer.",
    );
  }
}

/** True when `name` already carries code — i.e. a previous run placed the stack on this chain. */
export async function isPlaced(node: FhevmNode, name: HostContractName): Promise<boolean> {
  const code = await node.getCode(ADDRESSES[name]);
  return code.length > 2;
}
