import type { ContractTemplate } from './artifacts/types.js';
import type {
  AddressReplacement,
  ContractUpgradeSpec,
  DeployedBytecodeCheck,
  HexString,
  DeployedImplementation,
  TemplateBytecodeField,
  UpgradeTarget,
} from './types/private.js';
import type {
  AbstractEthereumProvider,
  AbstractEthereumSigner,
  AbstractEthereumUtils,
  CleartextAddresses,
  FhevmAddressesV14,
} from './types/public.js';

////////////////////////////////////////////////////////////////////////////////

function normalizeHex(value: string, label: string): string {
  if (!/^0x[0-9a-fA-F]*$/.test(value)) {
    throw new Error(`${label} is not a hex string`);
  }

  const hex = value.slice(2).toLowerCase();
  if (hex.length % 2 !== 0) {
    throw new Error(`${label} has an odd hex length`);
  }

  return hex;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Checks whether a contract is deployed at `address` and, if so, whether its runtime code matches
 * `expectedDeployedBytecode`. Comparison is case-insensitive over the raw hex.
 */
export async function checkDeployedBytecode(parameters: {
  readonly ethProvider: AbstractEthereumProvider;
  readonly address: string;
  readonly expectedDeployedBytecode: string;
}): Promise<DeployedBytecodeCheck> {
  const onChainCode = await parameters.ethProvider.getCodeAt({ address: parameters.address });
  const actual = normalizeHex(onChainCode, `on-chain code at ${parameters.address}`);
  if (actual.length === 0) {
    return { status: 'not-deployed' };
  }

  const expected = normalizeHex(parameters.expectedDeployedBytecode, 'expected deployed bytecode');
  if (actual === expected) {
    return { status: 'match' };
  }

  return { status: 'mismatch', actualDeployedBytecode: `0x${actual}`, expectedDeployedBytecode: `0x${expected}` };
}

////////////////////////////////////////////////////////////////////////////////

export function assertDeployedAddress(parameters: {
  readonly contractName: string;
  readonly expectedAddress: string;
  readonly actualAddress: string;
}): void {
  const expectedAddress = normalizeHex(parameters.expectedAddress, `${parameters.contractName} expected address`);
  const actualAddress = normalizeHex(parameters.actualAddress, `${parameters.contractName} deployed address`);

  if (actualAddress !== expectedAddress) {
    throw new Error(
      `${parameters.contractName} deployed at ${parameters.actualAddress}, expected ${parameters.expectedAddress}`,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Asserts that no contract code is deployed at `address`. Used before deploying to a precomputed
 * address to fail fast if the slot is already occupied (e.g. a partial or repeated deployment).
 */
export async function assertNoCodeAt(parameters: {
  readonly ethProvider: AbstractEthereumProvider;
  readonly contractName: string;
  readonly address: string;
}): Promise<void> {
  const code = normalizeHex(
    await parameters.ethProvider.getCodeAt({ address: parameters.address }),
    `${parameters.contractName} address code`,
  );
  if (code.length !== 0) {
    throw new Error(`${parameters.contractName} address ${parameters.address} already has code deployed`);
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Asserts that none of the precomputed host addresses already have code deployed. Run as a
 * precondition before `deployEmptyProxies(...)` to fail fast if any target slot is occupied
 * (wrong start nonce, or a partial/repeated deployment).
 */
export async function assertNoCodeAtTargets(parameters: {
  readonly ethProvider: AbstractEthereumProvider;
  readonly targets: ReadonlyArray<{ readonly contractName: string; readonly address: string }>;
}): Promise<void> {
  for (const target of parameters.targets) {
    await assertNoCodeAt({
      ethProvider: parameters.ethProvider,
      contractName: target.contractName,
      address: target.address,
    });
  }
}

////////////////////////////////////////////////////////////////////////////////

export function patchTemplateBytecode(parameters: {
  readonly template: ContractTemplate;
  readonly field: TemplateBytecodeField;
  readonly replacements: readonly AddressReplacement[];
}): HexString {
  const offsetField = parameters.field === 'bytecode' ? 'bytecodeOffsets' : 'deployedBytecodeOffsets';
  let hex = normalizeHex(
    parameters.template[parameters.field],
    `${parameters.template.contractName}.${parameters.field}`,
  );

  for (const replacement of parameters.replacements) {
    const reference = parameters.template.addressReferences[replacement.referenceName];
    if (reference === undefined) {
      throw new Error(`${parameters.template.contractName} template is missing ${replacement.referenceName}`);
    }

    const placeholder = normalizeHex(
      reference.placeholder,
      `${parameters.template.contractName}.${replacement.referenceName}.placeholder`,
    );
    const replacementHex = normalizeHex(replacement.replacement, `${replacement.referenceName} replacement`);

    if (replacementHex.length !== placeholder.length) {
      throw new Error(`${replacement.referenceName} replacement must have the same length as its placeholder`);
    }

    for (const byteOffset of reference[offsetField]) {
      const hexOffset = byteOffset * 2;
      if (hex.slice(hexOffset, hexOffset + placeholder.length) !== placeholder) {
        throw new Error(
          `${parameters.template.contractName}.${parameters.field} ${replacement.referenceName} offset ${byteOffset} does not point to the placeholder`,
        );
      }

      hex = `${hex.slice(0, hexOffset)}${replacementHex}${hex.slice(hexOffset + placeholder.length)}`;
    }
  }

  return `0x${hex}`;
}

////////////////////////////////////////////////////////////////////////////////

/** Minimal ABI fragment for the UUPS upgrade entrypoint shared by every host proxy. */
const UPGRADE_TO_AND_CALL_ABI = [
  {
    type: 'function',
    name: 'upgradeToAndCall',
    stateMutability: 'payable',
    inputs: [
      { name: 'newImplementation', type: 'address' },
      { name: 'data', type: 'bytes' },
    ],
    outputs: [],
  },
] as const;

/**
 * Deploys one real implementation and encodes the calldata to point its proxy at it. Sends no
 * owner-gated transaction — returns a `DeployedImplementation` for a caller to execute.
 */
export async function deployImplementation(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly deployer: AbstractEthereumSigner;
  readonly contractName: string;
  readonly proxyAddress: string;
  readonly template: ContractTemplate;
  readonly abi: readonly unknown[];
  readonly addressReplacements: readonly AddressReplacement[];
  readonly spec: ContractUpgradeSpec;
}): Promise<DeployedImplementation> {
  // 1. Patch the creation bytecode with the real host addresses, then deploy the implementation.
  //    (No constructor args: implementations use `constructor() { _disableInitializers(); }`.)
  const bytecode = patchTemplateBytecode({
    template: parameters.template,
    field: 'bytecode',
    replacements: parameters.addressReplacements,
  });
  const { contractAddress: implementationAddress } = await parameters.deployer.deploy({ bytecode });

  // 2. Encode the initializer (bootstrap `initializeFromEmptyProxy` or live `reinitializeVX`).
  const initData = await parameters.ethUtils.encodeCall({
    abi: parameters.abi,
    functionName: parameters.spec.initFn,
    args: parameters.spec.initArgs,
  });

  // 3. Encode upgradeToAndCall(newImplementation, data) — the owner-gated call, left unsent.
  const upgradeCalldata = await parameters.ethUtils.encodeCall({
    abi: UPGRADE_TO_AND_CALL_ABI,
    functionName: 'upgradeToAndCall',
    args: [implementationAddress, initData],
  });

  return {
    contractName: parameters.contractName,
    proxyAddress: parameters.proxyAddress,
    implementationAddress,
    initData,
    upgradeCalldata,
  };
}

/** Deploys each target's implementation and encodes its `upgradeToAndCall` (Phase 1; sends nothing). */
export async function deployImplementations(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly deployer: AbstractEthereumSigner;
  readonly addressReplacements: readonly AddressReplacement[];
  readonly targets: readonly UpgradeTarget[];
}): Promise<readonly DeployedImplementation[]> {
  const implementations: DeployedImplementation[] = [];
  for (const target of parameters.targets) {
    implementations.push(
      await deployImplementation({
        ethUtils: parameters.ethUtils,
        deployer: parameters.deployer,
        contractName: target.contractName,
        proxyAddress: target.proxyAddress,
        template: target.template,
        abi: target.abi,
        addressReplacements: parameters.addressReplacements,
        spec: target.spec,
      }),
    );
  }
  return implementations;
}

/**
 * The eight host addresses baked into every real implementation's bytecode (via `FHEVMHostAddresses.sol`).
 * References whose offsets are empty for a given template patch as no-ops, so passing all eight is safe.
 */
export function buildHostAddressReplacementsV14(parameters: {
  readonly fhevmAddresses: FhevmAddressesV14;
  readonly pauserSetAddress: string;
  // Optional: only the fresh `deploy` materializes the cleartext-infra contracts. The (deferred)
  // v12→v13 update path omits them until the cleartext-v12 fixture lands (see plan Decision #4).
  readonly cleartextAddresses?: CleartextAddresses;
}): AddressReplacement[] {
  const replacements: AddressReplacement[] = [
    // v0.12.0
    { referenceName: 'ACL_ADDRESS', replacement: parameters.fhevmAddresses.aclAddress },
    { referenceName: 'FHEVM_EXECUTOR_ADDRESS', replacement: parameters.fhevmAddresses.fhevmExecutorAddress },
    { referenceName: 'KMS_VERIFIER_ADDRESS', replacement: parameters.fhevmAddresses.kmsVerifierAddress },
    { referenceName: 'INPUT_VERIFIER_ADDRESS', replacement: parameters.fhevmAddresses.inputVerifierAddress },
    { referenceName: 'HCU_LIMIT_ADDRESS', replacement: parameters.fhevmAddresses.hcuLimitAddress },
    // v0.13.0
    { referenceName: 'PROTOCOL_CONFIG_ADDRESS', replacement: parameters.fhevmAddresses.protocolConfigAddress },
    { referenceName: 'KMS_GENERATION_ADDRESS', replacement: parameters.fhevmAddresses.kmsGenerationAddress },
    { referenceName: 'PAUSER_SET_ADDRESS', replacement: parameters.pauserSetAddress },
  ];

  if (parameters.cleartextAddresses !== undefined) {
    replacements.push(
      {
        referenceName: 'CLEARTEXT_ARITHMETIC_ADDRESS',
        replacement: parameters.cleartextAddresses.cleartextArithmeticAddress,
      },
      { referenceName: 'CLEARTEXT_DB_ADDRESS', replacement: parameters.cleartextAddresses.cleartextDbAddress },
    );
  }

  return replacements;
}
