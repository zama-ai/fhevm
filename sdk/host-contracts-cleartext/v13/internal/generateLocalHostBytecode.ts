// Generates pkg/forge/src/_internal/LocalHostBytecode.sol — bytecode for the whole cleartext stack,
// compiled against the *real* localhost addresses rather than placeholder markers.
//
// Modelled on forge-fhevm's src/generated/HostBytecode.sol: a flat list of file-level
// `bytes constant <NAME>_CREATION_CODE = hex"…"` declarations, so a Foundry consumer can CREATE the
// stack without compiling any Solidity of ours.
//
// Deliberately does NOT use the placeholder/patch technique. It compiles the addresses in:
//
//   1. derive the deployer from MNEMONIC at address index 5, and precompute the addresses its
//      nonce sequence will produce (exactly what `deploy()` does with no arguments)
//   2. write those addresses as a real `fhevm-config-<version>/addresses.sol`, then forge build
//   3. read the creation bytecode straight out of the artifacts
//
// So the emitted bytes are what solc actually emits for those addresses — no offsets, no patching, and
// nothing that can be patched wrongly. The script asserts no placeholder marker survives, which is what
// proves the config injection took effect rather than silently falling back.
//
// Output lives in pkg/forge/, deliberately outside foundry.toml's `src`. Forge therefore never compiles
// these files, which is what keeps them from becoming inputs to the very build that produces them, frees
// their pragma from the harness's pinned solc, and spares a consumer sweeping src/ from compiling ~139 KB
// of hex it may never use. The cost is one remapping in the consuming layer (see README.md, "Consuming
// pkg/forge from Foundry").
//
// Isolation: the fresh config goes to a tmp directory reached by temporarily repointing remappings.txt,
// and the build gets its own --out. internal/placeholders/addresses.sol, the committed templates and the
// normal out/ are all left untouched. `--force` gives a clean recompile without `forge clean` deleting
// the harness's build artifacts as a side effect.

import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { basename, dirname, join, relative } from 'node:path';
import { getContractAddress } from 'viem';
import { mnemonicToAccount } from 'viem/accounts';
import {
  ADDRESS_NAMES,
  CONSTANT_NAMES,
  COPROCESSOR_COUNT,
  COPROCESSOR_THRESHOLD,
  DECRYPTION_ADDRESS,
  GATEWAY_CHAIN_ID,
  HCU_CAP_PER_BLOCK,
  INPUT_VERIFICATION_ADDRESS,
  KMS_NODE_COUNT,
  MAX_HCU_DEPTH_PER_TX,
  MAX_HCU_PER_TX,
  type ContractName,
  DEPLOYER_ADDRESS_INDEX,
  DEPLOYER_START_NONCE,
  FHEVM_CONFIG_REMAPPING_PREFIX,
  MNEMONIC,
  PACKAGE_ROOT_ABS_PATH,
  ZAMA_LOCAL_CONFIG,
  ADDRESSED_NONCE_COUNT,
  NONCE_OFFSET,
  type AddressName,
} from './constants.ts';
import { SIGNER_SETS, deriveSigners } from './generateSigners.ts';
import { TARGET_CONTRACTS } from './generateTemplates.ts';
import { placeholderFor } from './generatePlaceholders.ts';
import { cast, forge, readJson } from './utils.ts';

////////////////////////////////////////////////////////////////////////////////

const TMP_DIR = join(PACKAGE_ROOT_ABS_PATH, 'internal', '.tmp-localhost');
export const OUTPUT_PATH = join(PACKAGE_ROOT_ABS_PATH, 'pkg', 'forge', 'src', '_internal', 'LocalHostBytecode.sol');
export const ADDRESSES_OUTPUT_PATH = join(
  PACKAGE_ROOT_ABS_PATH,
  'pkg',
  'forge',
  'src',
  '_internal',
  'LocalHostAddresses.sol',
);
const BOOTSTRAP_OUTPUT_PATH = join(PACKAGE_ROOT_ABS_PATH, 'pkg', 'forge', 'src', '_internal', 'LocalHostBootstrap.sol');
const REMAPPINGS_PATH = join(PACKAGE_ROOT_ABS_PATH, 'remappings.txt');

////////////////////////////////////////////////////////////////////////////////

/**
 * Which blob each contract ships, mirroring generate.py's CREATION/RUNTIME split.
 *
 * `runtime` is only correct where etching the deployed blob is equivalent to constructing the
 * contract — that needs no constructor side effects and no immutables, since immutables live inside
 * runtime code and would be frozen at whatever the generating build produced. Measured for this stack:
 *
 *   - `PauserSet` has no constructor at all and no immutables, so its runtime blob is complete.
 *   - `ERC1967Proxy` and `ACLOwner` take constructor arguments, and `ACLOwner` also carries an
 *     immutable — neither can be a fixed blob.
 *   - every proxy implementation's constructor calls `_disableInitializers()`. That is a storage write,
 *     so an etched implementation stays directly initializable where a constructed one does not. The
 *     difference is invisible through a proxy but real, so they ship creation code.
 */
export const CODE_KIND: Readonly<Record<ContractName, CodeKind>> = {
  ACL: 'creation',
  ACLOwner: 'creation',
  CleartextArithmetic: 'creation',
  CleartextDB: 'creation',
  CleartextFHEVMExecutor: 'creation',
  CleartextInputVerifier: 'creation',
  CleartextKMSVerifier: 'creation',
  EmptyUUPSProxy: 'creation',
  EmptyUUPSProxyACL: 'creation',
  ERC1967Proxy: 'creation',
  HCULimit: 'creation',
  KMSGeneration: 'creation',
  ProtocolConfig: 'creation',
  PauserSet: 'runtime',
};

/** `FheType` reaches generated interfaces as `type FheType is uint8;`, local to each interface and so
 * incompatible across them. Rewritten to import the one shared enum instead (generate.py does the same). */
const FHE_TYPE_DECLARATION = '    type FheType is uint8;';
const FHE_TYPE_IMPORT = 'import {FheType} from "../../../../src/contracts/shared/FheType.sol";';

/** Contracts deployed at nonces that hold no named address — see renderAddresses. */
const UNNAMED_NONCE_CONTRACTS: Readonly<Record<number, string>> = {
  0: 'EmptyUUPSProxyACL',
  2: 'EmptyUUPSProxy (shared implementation)',
};

type CodeKind = 'creation' | 'runtime';

type Artifact = {
  readonly bytecode: { readonly object: string };
  readonly deployedBytecode: { readonly object: string };
};

/**
 * The address set `deploy()` produces for this mnemonic, keyed by config constant name.
 *
 * Derived from `NONCE_OFFSET` in constants.ts, which restates the ordering in `pkg/ts/addresses.ts`
 * because `internal/` cannot import it. See that constant for why the duplication is deliberate and
 * what catches it going stale.
 */
type LocalHostStack = {
  /** Inputs to the derivation, echoed back so renderers read one object rather than reaching for module
   * constants — a template mixing the two leaves no way to tell why a value came from where. */
  readonly mnemonic: string;
  readonly deployerAddressIndex: number;
  readonly startNonce: bigint;
  /** Outputs of the derivation. */
  readonly deployer: `0x${string}`;
  readonly byName: Record<AddressName, string>;
  /** Address created at each nonce of the deploy sequence, index = nonce. */
  readonly nonceSequence: readonly string[];
};

/** What one run produced, for the caller to report. */
type LocalHostBytecodeResult = {
  readonly stack: LocalHostStack;
  /** Emitted blob per contract, hex without the 0x prefix. */
  readonly code: ReadonlyMap<ContractName, string>;
  /** Names of the interfaces written under pkg/forge/src/_internal/interfaces/. */
  readonly interfaces: readonly string[];
};

/** ZamaConfig field -> the name it corresponds to here, for the assertion below. */
const ZAMA_LOCAL_CONFIG_NAMES: Readonly<Record<keyof typeof ZAMA_LOCAL_CONFIG, AddressName>> = {
  aclAddress: 'ACL_ADDRESS',
  fhevmExecutorAddress: 'FHEVM_EXECUTOR_ADDRESS',
  kmsVerifierAddress: 'KMS_VERIFIER_ADDRESS',
};

////////////////////////////////////////////////////////////////////////////////

export function localHostAddresses(): LocalHostStack {
  const deployer = mnemonicToAccount(MNEMONIC, { addressIndex: DEPLOYER_ADDRESS_INDEX }).address;
  const at = (nonce: bigint): string => getContractAddress({ from: deployer, nonce });

  const byName = Object.fromEntries(
    ADDRESS_NAMES.map((name) => [name, at(DEPLOYER_START_NONCE + NONCE_OFFSET[name])]),
  ) as Record<AddressName, string>;

  // Every nonce the sequence consumes, so the deploy order can be labelled rather than assumed. Indexed
  // by nonce, hence from 0 rather than from the start nonce.
  const nonceSequence: string[] = [];
  for (let nonce = 0n; nonce < ADDRESSED_NONCE_COUNT; nonce++) {
    nonceSequence.push(at(nonce));
  }

  // The check that makes the duplication above safe: these three addresses are compiled into every dApp
  // that uses the FHE library's local config, so a wrong offset is caught here rather than shipping a
  // stack nothing can find.
  for (const [field, expected] of Object.entries(ZAMA_LOCAL_CONFIG)) {
    const name = ZAMA_LOCAL_CONFIG_NAMES[field as keyof typeof ZAMA_LOCAL_CONFIG];
    const actual = byName[name];
    if (actual.toLowerCase() !== expected.toLowerCase()) {
      throw new Error(
        `${field} derived as ${actual}, but ZamaConfig._getLocalConfig() says ${expected} ` +
          `(RULES.md rules 15 and 17). Check MNEMONIC, DEPLOYER_ADDRESS_INDEX, the start nonce and ` +
          `NONCE_OFFSET against pkg/ts/addresses.ts.`,
      );
    }
  }

  return {
    mnemonic: MNEMONIC,
    deployerAddressIndex: DEPLOYER_ADDRESS_INDEX,
    startNonce: DEPLOYER_START_NONCE,
    deployer,
    nonceSequence,
    byName,
  };
}

////////////////////////////////////////////////////////////////////////////////

function _addressConfigSource(byName: Record<AddressName, string>): string {
  const lines = ADDRESS_NAMES.map((name) => `address constant ${name} = address(${byName[name]});`);
  return `// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// GENERATED for LocalHostBytecode.sol — the real localhost address set, not placeholder markers.
${lines.join('\n')}
`;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Everything a Foundry consumer needs to stand the stack up, beyond the bytecode itself: the account the
 * addresses derive from, the addresses, and the deploy order that produces them.
 *
 * Nonces carrying no named address deploy the empty-proxy implementations each proxy is constructed over.
 * They are named from `deploy()`'s own numbered steps (`pkg/ts/deploy.ts`): step 1 deploys
 * `EmptyUUPSProxyACL` at offset 0, step 2 the ACL proxy at 1, step 3 the shared `EmptyUUPSProxy` at 2.
 * An unnamed nonce with no entry throws rather than printing a vague label, so a reordered deploy is a
 * generator failure instead of a misleading comment.
 *
 * The order matters as much as the addresses. Each address is fixed by `CREATE(deployer, nonce)`, so the
 * sequence is not a convenience listing — deploy in a different order, or from a different account or
 * start nonce, and every address moves while the bytecode keeps pointing at the old ones. The nonces
 * carrying no named address are the empty-proxy implementations each proxy is constructed over.
 */
function _renderAddresses(stack: LocalHostStack): string {
  const nameByAddress = new Map(ADDRESS_NAMES.map((name) => [stack.byName[name].toLowerCase(), name] as const));
  const order = stack.nonceSequence
    .map((address, nonce) => {
      const name = nameByAddress.get(address.toLowerCase()) ?? UNNAMED_NONCE_CONTRACTS[nonce];
      if (name === undefined) {
        throw new Error(
          `nonce ${String(nonce)} carries no named address and has no UNNAMED_NONCE_CONTRACTS entry — ` +
            `the deploy sequence in pkg/ts/deploy.ts changed, so record what is deployed there.`,
        );
      }
      return `///   nonce ${String(nonce).padStart(2)}  ${address}  ${name}`;
    })
    .join('\n');
  const constants = ADDRESS_NAMES.map((name) => `address constant ${name} = ${stack.byName[name]};`).join('\n');

  // ^0.8.24, not the model's ^0.8.27: it is the payload's own floor (rule 16), it is what the harness
  // pins so test/FhevmDeploy.t.sol can compile these files, and it accepts every consumer 0.8.27 would.
  return `// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// AUTOGENERATED by \`npm run generate:local-host-bytecode\`.
// DO NOT EDIT — your changes will be overwritten. See internal/generateLocalHostBytecode.ts.
//
// The localhost stack's identity: the account it is deployed from, the addresses that account produces,
// and the order that produces them. Pairs with LocalHostBytecode.sol, which is compiled against exactly
// these addresses — the two files are halves of one artifact and must be regenerated together.
//
// ACL_ADDRESS, FHEVM_EXECUTOR_ADDRESS and KMS_VERIFIER_ADDRESS are the three values
// library-solidity/config/ZamaConfig.sol compiles into every dApp inheriting its localhost config
// (RULES.md rules 15 and 17), which is why none of this may drift.

/// @dev BIP-39 mnemonic the local stack is deployed with.
string constant MNEMONIC = "${stack.mnemonic}";

/// @dev Account index derived from MNEMONIC that deploys the stack (path m/44'/60'/0'/0/${String(stack.deployerAddressIndex)}).
uint32 constant DEPLOYER_ADDRESS_INDEX = ${String(stack.deployerAddressIndex)};

/// @dev The deploying account. Must be at nonce DEPLOYER_START_NONCE when the sequence begins.
address constant DEPLOYER_ADDRESS = ${stack.deployer};

/// @dev Nonce the sequence starts from — the deployer must have sent no transaction yet.
uint64 constant DEPLOYER_START_NONCE = ${String(stack.startNonce)};

/// @dev Contracts created at each nonce, in deploy order.
${order}

${constants}
`;
}

////////////////////////////////////////////////////////////////////////////////

/** Total over ContractName, so a contract without a name is a compile error in CONSTANT_NAMES. */
/**
 * The Solidity mirror of `DEFAUT_BOOTSTRAP_CONFIG_V13` in ts/constants.ts — what the initializers are
 * given when nobody overrides them.
 *
 * The signer pools are re-derived through `deriveSigners`, the same function that writes ts/signers/,
 * rather than copied from its output. That matters: the js-sdk cleartext relayer derives its own signing
 * keys from FHEVM_MNEMONIC at these HD paths and looks them up by on-chain address, so a stack
 * registering any other address is one the SDK holds no key for and cannot sign against.
 */
function _renderBootstrap(): string {
  const pool = (fileName: string): readonly string[] => {
    const set = SIGNER_SETS.find((candidate) => candidate.fileName === fileName);
    if (set === undefined) {
      throw new Error(`No SIGNER_SETS entry named ${fileName} — the signer layout changed.`);
    }
    return deriveSigners(set).map((signer) => signer.address);
  };

  const addressFn = (name: string, values: readonly string[], count: number, path: string): string => {
    if (values.length < count) {
      throw new Error(`${name}: need ${String(count)} signers, derived ${String(values.length)}`);
    }
    const body = values
      .slice(0, count)
      .map((value, index) => `        out[${String(index)}] = ${value};`)
      .join('\n');
    return [
      `    /// @dev ${path}i`,
      `    function ${name}() internal pure returns (address[] memory out) {`,
      `        out = new address[](${String(count)});`,
      body,
      '    }',
    ].join('\n');
  };

  const stringFn = (name: string, values: readonly string[]): string => {
    const body = values.map((value, index) => `        out[${String(index)}] = "${value}";`).join('\n');
    return [
      `    function ${name}() internal pure returns (string[] memory out) {`,
      `        out = new string[](${String(values.length)});`,
      body,
      '    }',
    ].join('\n');
  };

  const ips = Array.from({ length: KMS_NODE_COUNT }, (_unused, index) => `127.0.0.${String(index + 1)}`);
  const urls = Array.from({ length: KMS_NODE_COUNT }, (_unused, index) => `s3://kms-bucket-${String(index + 1)}`);

  return `// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// AUTOGENERATED by \`npm run generate:local-host-bytecode\`.
// DO NOT EDIT — your changes will be overwritten. See internal/generateLocalHostBytecode.ts.
//
// The bootstrap arguments ts/deploy.ts applies when no config is supplied — the Solidity mirror of
// DEFAUT_BOOTSTRAP_CONFIG_V13. The signer pools are derived from FHEVM_MNEMONIC at the same HD paths the
// js-sdk cleartext relayer derives its keys from, so a stack deployed with these is one the SDK can sign
// for. Registering any other signer leaves the relayer with no key for the address the chain reports.

library LocalHostBootstrap {
    uint64 internal constant GATEWAY_CHAIN_ID = ${String(GATEWAY_CHAIN_ID)};

    /// @dev The EIP-712 verifyingContract each proof is bound to. Not deployed anywhere: on a real
    ///      network these are gateway contracts, and in cleartext mode they only need to be a stable
    ///      agreed-upon value, so each is derived from a namespaced string:
    ///        INPUT_VERIFICATION_ADDRESS =
    ///          address(uint160(uint256(keccak256("fhevm.cheat.address cleartext input verification"))))
    ///        DECRYPTION_ADDRESS =
    ///          address(uint160(uint256(keccak256("fhevm.cheat.address cleartext decryption"))))
    ///      Both re-derived and checked against ts/constants.ts.
    address internal constant INPUT_VERIFICATION_ADDRESS = ${INPUT_VERIFICATION_ADDRESS};
    address internal constant DECRYPTION_ADDRESS = ${DECRYPTION_ADDRESS};

    uint256 internal constant COPROCESSOR_THRESHOLD = ${String(COPROCESSOR_THRESHOLD)};
    uint256 internal constant KMS_NODE_COUNT = ${String(KMS_NODE_COUNT)};

    uint48 internal constant HCU_CAP_PER_BLOCK = ${String(HCU_CAP_PER_BLOCK)};
    uint48 internal constant MAX_HCU_DEPTH_PER_TX = ${String(MAX_HCU_DEPTH_PER_TX)};
    uint48 internal constant MAX_HCU_PER_TX = ${String(MAX_HCU_PER_TX)};

${addressFn('coprocessorSigners', pool('defaultCoprocessorSigners.ts'), COPROCESSOR_COUNT, "m/44'/60'/0'/2/")}

${addressFn('kmsSigners', pool('defaultKmsSigners.ts'), KMS_NODE_COUNT, "m/44'/60'/0'/3/")}

${addressFn('kmsTxSenders', pool('defaultKmsTxSenderSigners.ts'), KMS_NODE_COUNT, "m/44'/60'/0'/4/")}

${stringFn('kmsIpAddresses', ips)}

${stringFn('kmsStorageUrls', urls)}
}
`;
}

////////////////////////////////////////////////////////////////////////////////

function _constantFor(contractName: ContractName): string {
  return CONSTANT_NAMES[contractName];
}

////////////////////////////////////////////////////////////////////////////////

function _renderCodeSection(code: ReadonlyMap<ContractName, string>, kind: CodeKind): string {
  const suffix = kind === 'creation' ? 'CREATION_CODE' : 'RUNTIME_CODE';
  return [...code]
    .filter(([contractName]) => CODE_KIND[contractName] === kind)
    .map(
      ([contractName, hex]) =>
        `/// @dev ${contractName} ${kind} bytecode (${String(hex.length / 2)} bytes).\n` +
        `bytes constant ${_constantFor(contractName)}_${suffix} =\n    hex"${hex}";`,
    )
    .join('\n\n');
}

////////////////////////////////////////////////////////////////////////////////

function _render(stack: LocalHostStack, code: ReadonlyMap<ContractName, string>): string {
  const addressComment = ADDRESS_NAMES.map((name) => `///   ${name} = ${stack.byName[name]}`).join('\n');

  // ^0.8.24, not the model's ^0.8.27: it is the payload's own floor (rule 16), it is what the harness
  // pins so test/FhevmDeploy.t.sol can compile these files, and it accepts every consumer 0.8.27 would.
  return `// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// AUTOGENERATED by \`npm run generate:local-host-bytecode\`.
// DO NOT EDIT — your changes will be overwritten. See internal/generateLocalHostBytecode.ts.
//
// Bytecode for the cleartext stack, compiled against the localhost address set below. Deploying the
// creation blobs in the order of RULES.md rule 17 from account index ${String(stack.deployerAddressIndex)} of
// the anvil mnemonic, starting at nonce ${String(stack.startNonce)}, reproduces exactly those addresses — the bytecode and the
// addresses are two halves of one artifact and cannot be mixed with another deployer.
//
// CREATION_CODE must be deployed: the constructor either takes arguments or writes storage.
// RUNTIME_CODE may be etched at its address, being equivalent to constructing the contract.
//
/// Deployer: ${stack.deployer} (address index ${String(stack.deployerAddressIndex)})
${addressComment}

${_renderCodeSection(code, 'creation')}

${_renderCodeSection(code, 'runtime')}
`;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Writes one `I<Contract>.sol` per target from `cast interface`, replacing cast's own SPDX and pragma
 * lines with ours — cast emits `pragma solidity ^0.8.4` and `UNLICENSED`, and these files live inside
 * pkg/src, so both have to match the package.
 */
function _generateInterfaces(tmpOut: string): readonly string[] {
  const interfaceDir = join(dirname(OUTPUT_PATH), 'interfaces');
  rmSync(interfaceDir, { recursive: true, force: true });
  mkdirSync(interfaceDir, { recursive: true });

  const written: string[] = [];
  for (const target of TARGET_CONTRACTS) {
    const artifactPath = join(tmpOut, basename(target.sourcePath), `${target.contractName}.json`);
    const name = `I${target.contractName}`;
    const body = cast(['interface', artifactPath, '--name', name])
      .split('\n')
      .filter((line) => !line.startsWith('// SPDX-License-Identifier') && !line.startsWith('pragma solidity'))
      .join('\n')
      .trim();

    const needsFheType = body.includes(FHE_TYPE_DECLARATION);
    const source = `// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// AUTOGENERATED by \`npm run generate:local-host-bytecode\` via \`cast interface\`.
// DO NOT EDIT — your changes will be overwritten. See internal/generateLocalHostBytecode.ts.
${needsFheType ? `\n${FHE_TYPE_IMPORT}\n` : ''}
${needsFheType ? body.replace(`${FHE_TYPE_DECLARATION}\n`, '') : body}
`;
    writeFileSync(join(interfaceDir, `${name}.sol`), source, 'utf8');
    written.push(name);
  }
  return written;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Compiles the stack against its real localhost addresses and writes the three generated Solidity files
 * plus the interfaces.
 *
 * Mutates remappings.txt for the duration of the build and restores it in a `finally`, so an interrupted
 * run cannot leave the repointed remapping behind.
 */
export function writeLocalHostBytecode(): LocalHostBytecodeResult {
  // Checked here rather than at the call site: every path below depends on it, and the message is what
  // tells you the real problem (a wrong working directory) instead of a bare ENOENT on readFileSync.
  if (!existsSync(REMAPPINGS_PATH)) {
    throw new Error(`Missing ${REMAPPINGS_PATH} — run from the package root.`);
  }

  const stack = localHostAddresses();
  const originalRemappings = readFileSync(REMAPPINGS_PATH, 'utf8');
  const tmpOut = join(TMP_DIR, 'out');
  const code = new Map<ContractName, string>();
  const interfaces: string[] = [];

  try {
    mkdirSync(TMP_DIR, { recursive: true });
    writeFileSync(join(TMP_DIR, 'addresses.sol'), _addressConfigSource(stack.byName), 'utf8');
    writeFileSync(
      REMAPPINGS_PATH,
      originalRemappings.replace(
        // replaceAll, not replace: the prefix has three dots and replace() only escapes the first, leaving
        // `.` free to match any character.
        new RegExp(`^${FHEVM_CONFIG_REMAPPING_PREFIX.replaceAll('.', '\\.')}=.*$`, 'm'),
        `${FHEVM_CONFIG_REMAPPING_PREFIX}=${relative(PACKAGE_ROOT_ABS_PATH, TMP_DIR)}/`,
      ),
      'utf8',
    );

    // --force rather than `forge clean`: a clean recompile without deleting the harness's out/.
    // --skip test: the forge test suite imports pkg/forge, which this script *produces*. Without the
    // skip the build depends on its own previous output, and a stale or absent file wedges the generator
    // that would replace it. Only pkg/src is needed here.
    forge(['build', '--force', '--skip', 'test', '--out', tmpOut]);

    // Any surviving marker means the remapping did not take effect and we would be shipping
    // placeholder addresses as if they were real ones.
    const markers = ADDRESS_NAMES.map((name) => placeholderFor(name).slice(2).toLowerCase());

    for (const target of TARGET_CONTRACTS) {
      const artifactPath = join(tmpOut, basename(target.sourcePath), `${target.contractName}.json`);
      const artifact = readJson<Artifact>(artifactPath);
      const kind = CODE_KIND[target.contractName];
      const field = kind === 'creation' ? artifact.bytecode : artifact.deployedBytecode;
      const hex = field.object.replace(/^0x/, '').toLowerCase();

      const survivor = markers.find((marker) => hex.includes(marker));
      if (survivor !== undefined) {
        throw new Error(`${target.contractName}: placeholder marker 0x${survivor} survived the build.`);
      }
      code.set(target.contractName, hex);
    }

    mkdirSync(dirname(OUTPUT_PATH), { recursive: true });
    writeFileSync(OUTPUT_PATH, _render(stack, code), 'utf8');
    writeFileSync(ADDRESSES_OUTPUT_PATH, _renderAddresses(stack), 'utf8');
    writeFileSync(BOOTSTRAP_OUTPUT_PATH, _renderBootstrap(), 'utf8');
    interfaces.push(..._generateInterfaces(tmpOut));
  } finally {
    writeFileSync(REMAPPINGS_PATH, originalRemappings, 'utf8');
    rmSync(TMP_DIR, { recursive: true, force: true });
  }

  return { stack, code, interfaces };
}
