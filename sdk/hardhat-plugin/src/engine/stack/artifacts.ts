import * as fs from "fs";

import type { InterfaceAbi } from "ethers";

import { ADDRESS_REFERENCES, type HostContractName } from "./addresses";

/**
 * The cleartext engine's single source of truth. ABIs and compiled bytecode both come from the package's
 * `./abi/*.json` and `./templates/*.json` subpath exports, so a rebuild of the contracts (`npm run
 * build:templates` in host-contracts-cleartext) is picked up immediately. Nothing is vendored here.
 *
 * Note we consume the JSON exports only — not the package's `./ts` entry. That library deploys the stack
 * with real CREATE transactions at nonce-derived addresses, which can never land on the addresses
 * `ZamaConfig` pins (see addresses.ts). We reuse its templates and do the placement ourselves.
 */
export const CLEARTEXT_PACKAGE = "@fhevm/host-contracts-cleartext";

/**
 * The contract placed at each host address. Only the contracts that must observe or expose cleartext have
 * a `Cleartext*` variant; the rest are the plain host contracts. `CleartextArithmetic` and `CleartextDB`
 * are new in the current engine: the executor outgrew EIP-170, so the arithmetic moved into its own contract, which
 * writes plaintexts to a shared DB.
 */
export const CONTRACT_NAMES: Record<HostContractName, string> = {
  ACL: "ACL",
  FHEVMExecutor: "CleartextFHEVMExecutor",
  KMSVerifier: "CleartextKMSVerifier",
  InputVerifier: "CleartextInputVerifier",
  HCULimit: "HCULimit",
  ProtocolConfig: "ProtocolConfig",
  KMSGeneration: "KMSGeneration",
  PauserSet: "PauserSet",
  CleartextArithmetic: "CleartextArithmetic",
  CleartextDB: "CleartextDB",
};

/** A `templates/*.json`: compiled bytecode plus the byte offsets of every host-address reference. */
interface ContractTemplate {
  readonly contractName: string;
  /** "proxy" contracts have an `initializeFromEmptyProxy`; "non-proxy" ones (PauserSet) do not. */
  readonly kind: "proxy" | "non-proxy";
  readonly bytecode: string;
  readonly deployedBytecode: string;
  readonly addressReferences: Record<
    string,
    { readonly placeholder: string; readonly bytecodeOffsets: number[]; readonly deployedBytecodeOffsets: number[] }
  >;
}

export interface CleartextArtifact {
  readonly contractName: string;
  readonly kind: "proxy" | "non-proxy";
  readonly abi: InterfaceAbi;
  /** Runtime bytecode, with every host-address placeholder rewritten to our address map. */
  readonly deployedBytecode: string;
}

const cache = new Map<HostContractName, CleartextArtifact>();

function load(subpath: string, what: string): unknown {
  let resolved: string;
  try {
    resolved = require.resolve(`${CLEARTEXT_PACKAGE}/${subpath}`);
  } catch (e) {
    throw new Error(
      `Unable to resolve ${what} from ${CLEARTEXT_PACKAGE} (${subpath}). Is the package installed, and have its ` +
        `templates been generated (\`npm run build:templates\`)?\n${e}`,
    );
  }
  return JSON.parse(fs.readFileSync(resolved, "utf8"));
}

/**
 * Rewrites every host-address reference in the runtime bytecode to the address we actually place that
 * contract at. This is the package's own mechanism (`ts/utils.ts: patchTemplateBytecode`) reimplemented against the
 * runtime field: the package does not re-export it, and it patches CREATION bytecode for a CREATE deploy,
 * whereas we write runtime code directly.
 *
 * Every offset is asserted to actually hold the placeholder before substitution, so templates regenerated
 * with different offsets fail loudly instead of silently corrupting the bytecode.
 */
function patchDeployedBytecode(template: ContractTemplate): string {
  let hex = template.deployedBytecode.replace(/^0x/, "").toLowerCase();

  for (const [referenceName, reference] of Object.entries(template.addressReferences)) {
    const target = ADDRESS_REFERENCES[referenceName];
    if (target === undefined) {
      throw new Error(
        `${template.contractName} references the unknown host address '${referenceName}'. ` +
          `Add it to ADDRESS_REFERENCES in engine/addresses.ts.`,
      );
    }

    const placeholder = reference.placeholder.replace(/^0x/, "").toLowerCase();
    const replacement = target.replace(/^0x/, "").toLowerCase();
    if (replacement.length !== placeholder.length) {
      throw new Error(`${referenceName}: replacement and placeholder differ in length`);
    }

    for (const byteOffset of reference.deployedBytecodeOffsets) {
      const at = byteOffset * 2;
      if (hex.slice(at, at + placeholder.length) !== placeholder) {
        throw new Error(
          `${template.contractName}.deployedBytecode: ${referenceName} offset ${byteOffset} does not point at its ` +
            `placeholder. The templates are out of sync — regenerate them.`,
        );
      }
      hex = hex.slice(0, at) + replacement + hex.slice(at + placeholder.length);
    }
  }

  return `0x${hex}`;
}

export function getCleartextArtifact(name: HostContractName): CleartextArtifact {
  const cached = cache.get(name);
  if (cached) {
    return cached;
  }

  const contractName = CONTRACT_NAMES[name];
  const template = load(`templates/${contractName}.json`, `the template for '${contractName}'`) as ContractTemplate;
  const abi = load(`abi/${contractName}.json`, `the ABI for '${contractName}'`) as InterfaceAbi;

  if (typeof template.deployedBytecode !== "string" || template.deployedBytecode.length <= 2) {
    throw new Error(`Template '${contractName}' has no deployedBytecode.`);
  }

  const artifact: CleartextArtifact = {
    contractName,
    kind: template.kind,
    abi,
    deployedBytecode: patchDeployedBytecode(template),
  };
  cache.set(name, artifact);
  return artifact;
}

/** Every contract the engine places. */
export const ALL_CONTRACTS = Object.keys(CONTRACT_NAMES) as HostContractName[];
