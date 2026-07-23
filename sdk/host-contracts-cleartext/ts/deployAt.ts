import { abi as aclAbi, template as aclTemplate } from './artifacts/ACL.js';
import { abi as fhevmExecutorAbi, template as fhevmExecutorTemplate } from './artifacts/CleartextFHEVMExecutor.js';
import { abi as kmsVerifierAbi, template as kmsVerifierTemplate } from './artifacts/CleartextKMSVerifier.js';
import { abi as inputVerifierAbi, template as inputVerifierTemplate } from './artifacts/CleartextInputVerifier.js';
import { abi as hcuLimitAbi, template as hcuLimitTemplate } from './artifacts/HCULimit.js';
import { abi as protocolConfigAbi, template as protocolConfigTemplate } from './artifacts/ProtocolConfig.js';
import { abi as kmsGenerationAbi, template as kmsGenerationTemplate } from './artifacts/KMSGeneration.js';
import {
  abi as cleartextArithmeticAbi,
  template as cleartextArithmeticTemplate,
} from './artifacts/CleartextArithmetic.js';
import { abi as cleartextDbAbi, template as cleartextDbTemplate } from './artifacts/CleartextDB.js';
import { abi as pauserSetAbi, template as pauserSetTemplate } from './artifacts/PauserSet.js';
import type { ContractTemplate } from './artifacts/types.js';
import type { AddressReplacement } from './types/private.js';
import type {
  AbstractEthereumProvider,
  AbstractEthereumSigner,
  BootstrapConfigV14,
  FixedAddressesV14,
} from './types/public.js';
import { patchTemplateBytecode } from './utils.js';
import { bootstrapInitCalls } from './deploy.js';

/**
 * ERC-7201 storage locations (OpenZeppelin upgradeable). These are layout facts about the contracts we
 * write into, not configuration.
 */
const INITIALIZABLE_STORAGE_SLOT = '0xf0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00';
const OWNABLE_STORAGE_SLOT = '0x9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300';

type PlacedContract = {
  readonly contractName: string;
  readonly address: string;
  readonly template: ContractTemplate;
  readonly abi: readonly unknown[];
};

/**
 * Deploy the cleartext stack at CALLER-CHOSEN addresses, on a dev node.
 *
 * WHY THIS EXISTS, ALONGSIDE `deploy`. `deploy` is the real thing: genuine ERC-1967 proxies, materialized
 * atomically through a standing `ACLOwner`. But it deploys with CREATE, so every address is a function of
 * the deployer's nonce. That is fine on a real chain, where the addresses are whatever they come out as —
 * and useless in a test harness, where they are not free: a contract under test compiles the addresses from
 * `ZamaConfig` into its own bytecode, so the stack has to meet it at those exact addresses. No choice of
 * deployer or nonce lands on them.
 *
 * So this places the implementations' runtime code directly, with `setCodeAt`, and runs the same
 * initializers `deploy` would. Two things make that sound:
 *
 *  - The contracts still find each other. Their cross-references are compile-time constants
 *    (`config/addresses.sol`), which the templates expose as patchable byte offsets. Every one is rewritten
 *    to `addresses`, so the stack is internally consistent at whatever addresses the caller picked.
 *  - The initializers still run for real. `initializeFromEmptyProxy` is guarded by `onlyFromEmptyProxy`,
 *    which requires `_getInitializedVersion() == 1` — the state a real `EmptyUUPSProxy.initialize()` would
 *    have left behind. `setCodeAt` runs no constructor, so we write that slot ourselves; likewise ACL's
 *    owner, which `ACL.initializeFromEmptyProxy()` preserves rather than sets.
 *
 * What is given up is the proxy machinery itself — upgradeability, and the `ACLOwner` admin. A test stack
 * never upgrades, so this costs nothing and skips ~20 transactions.
 *
 * `admin` becomes ACL's owner (and therefore the whole stack's, via `ACLOwnable`) and sends every
 * initializer. It must be a funded account on the node.
 */
export async function deployAt(parameters: {
  // Only the cheat-code subset: `deployAt` places code and writes slots, it never reads contracts or
  // nonces. Keeps thin dev-node adapters (e.g. the hardhat plugin's) off the read-API hook.
  readonly ethProvider: Pick<AbstractEthereumProvider, 'setCodeAt' | 'setStorageAt' | 'getCodeAt'>;
  readonly admin: AbstractEthereumSigner;
  readonly addresses: FixedAddressesV14;
  readonly config: BootstrapConfigV14;
}): Promise<FixedAddressesV14> {
  const { addresses, ethProvider, admin } = parameters;
  const adminAddress = await admin.getAddress();

  const replacements = addressReplacements(addresses);
  const contracts = placedContracts(addresses);

  // 1. Place every contract's runtime code, cross-references already patched to the address map.
  for (const contract of contracts) {
    await ethProvider.setCodeAt({
      address: contract.address,
      bytecode: patchTemplateBytecode({
        template: contract.template,
        field: 'deployedBytecode',
        replacements,
      }),
    });
  }

  // 2. Fake the post-`EmptyUUPSProxy.initialize()` state that `onlyFromEmptyProxy` checks for. PauserSet is
  //    immutable (kind 'non-proxy') and has no initializer, so it is skipped.
  for (const contract of contracts) {
    if (contract.template.kind === 'proxy') {
      await ethProvider.setStorageAt({
        address: contract.address,
        slot: INITIALIZABLE_STORAGE_SLOT,
        value: encodeInitializable(1n),
      });
    }
  }

  // 3. ACL's owner must exist BEFORE its initializer runs: `ACL.initializeFromEmptyProxy()` takes no owner,
  //    it calls `__Ownable_init(owner())` and so preserves whatever is already in the slot. It is also the
  //    ownership root for the whole stack — `ACLOwnable` resolves every other contract's owner through it.
  await ethProvider.setStorageAt({
    address: addresses.fhevmAddresses.aclAddress,
    slot: OWNABLE_STORAGE_SLOT,
    value: encodeAddress(adminAddress),
  });

  // 4. Run the real initializers, in the order `deploy` uses.
  for (const call of bootstrapInitCalls({ addresses, config: parameters.config })) {
    await admin.writeContract({
      address: call.address,
      abi: call.abi,
      functionName: 'initializeFromEmptyProxy',
      args: call.args,
    });
  }

  return addresses;
}

/** Every host-address reference the templates can carry, pointed at the caller's map. */
function addressReplacements(addresses: FixedAddressesV14): AddressReplacement[] {
  const { fhevmAddresses: fhevm, cleartextAddresses: cleartext } = addresses;
  return [
    { referenceName: 'ACL_ADDRESS', replacement: fhevm.aclAddress },
    { referenceName: 'FHEVM_EXECUTOR_ADDRESS', replacement: fhevm.fhevmExecutorAddress },
    { referenceName: 'KMS_VERIFIER_ADDRESS', replacement: fhevm.kmsVerifierAddress },
    { referenceName: 'INPUT_VERIFIER_ADDRESS', replacement: fhevm.inputVerifierAddress },
    { referenceName: 'HCU_LIMIT_ADDRESS', replacement: fhevm.hcuLimitAddress },
    { referenceName: 'PROTOCOL_CONFIG_ADDRESS', replacement: fhevm.protocolConfigAddress },
    { referenceName: 'KMS_GENERATION_ADDRESS', replacement: fhevm.kmsGenerationAddress },
    { referenceName: 'PAUSER_SET_ADDRESS', replacement: addresses.pauserSetAddress },
    { referenceName: 'CLEARTEXT_ARITHMETIC_ADDRESS', replacement: cleartext.cleartextArithmeticAddress },
    { referenceName: 'CLEARTEXT_DB_ADDRESS', replacement: cleartext.cleartextDbAddress },
  ];
}

function placedContracts(addresses: FixedAddressesV14): readonly PlacedContract[] {
  const { fhevmAddresses: fhevm, cleartextAddresses: cleartext } = addresses;
  return [
    { contractName: 'ACL', address: fhevm.aclAddress, template: aclTemplate, abi: aclAbi },
    {
      contractName: 'FHEVMExecutor',
      address: fhevm.fhevmExecutorAddress,
      template: fhevmExecutorTemplate,
      abi: fhevmExecutorAbi,
    },
    {
      contractName: 'KMSVerifier',
      address: fhevm.kmsVerifierAddress,
      template: kmsVerifierTemplate,
      abi: kmsVerifierAbi,
    },
    {
      contractName: 'InputVerifier',
      address: fhevm.inputVerifierAddress,
      template: inputVerifierTemplate,
      abi: inputVerifierAbi,
    },
    { contractName: 'HCULimit', address: fhevm.hcuLimitAddress, template: hcuLimitTemplate, abi: hcuLimitAbi },
    {
      contractName: 'ProtocolConfig',
      address: fhevm.protocolConfigAddress,
      template: protocolConfigTemplate,
      abi: protocolConfigAbi,
    },
    {
      contractName: 'KMSGeneration',
      address: fhevm.kmsGenerationAddress,
      template: kmsGenerationTemplate,
      abi: kmsGenerationAbi,
    },
    {
      contractName: 'CleartextArithmetic',
      address: cleartext.cleartextArithmeticAddress,
      template: cleartextArithmeticTemplate,
      abi: cleartextArithmeticAbi,
    },
    {
      contractName: 'CleartextDB',
      address: cleartext.cleartextDbAddress,
      template: cleartextDbTemplate,
      abi: cleartextDbAbi,
    },
    { contractName: 'PauserSet', address: addresses.pauserSetAddress, template: pauserSetTemplate, abi: pauserSetAbi },
  ];
}

/**
 * OpenZeppelin `Initializable`, one slot packed from the LSB:
 *   bytes 24..31 = uint64 _initialized
 *   byte  23     = bool   _initializing
 */
function encodeInitializable(initialized: bigint): string {
  return `0x${initialized.toString(16).padStart(64, '0')}`;
}

function encodeAddress(address: string): string {
  return `0x${address.replace(/^0x/, '').toLowerCase().padStart(64, '0')}`;
}
