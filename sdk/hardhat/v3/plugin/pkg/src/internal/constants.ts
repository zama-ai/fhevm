// Plugin-local constants. Addresses never live here: they come from the generated chain constants
// and from @fhevm/host-contracts-cleartext.

export const PLUGIN_ID = 'fhevm';

/** Chain id of every development node (hardhat's and anvil's default); the cleartext stack targets it. */
export const DEVELOPMENT_CHAIN_ID = 31337;

/** Sibling npm packages the plugin locates in the USER's project, never in its own tree. */
export const FHEVM_SOLIDITY_PACKAGE_NAME = '@fhevm/solidity';
export const FHEVM_SOLIDITY_CONFIG_FILE = 'config/ZamaConfig.sol';
/** The config a consumer contract inherits for the Ethereum-family addresses (and the local stack, which mirrors them). */
export const FHEVM_SOLIDITY_CONFIG_CONTRACT_NAME = 'ZamaEthereumConfig';
export const FHEVM_SDK_PACKAGE_NAME = '@fhevm/sdk';

/** The plugin's OWN dependency (not a consumer sibling): ABIs and the deploy sequence of the cleartext stack. */
export const FHEVM_HOST_CONTRACTS_CLEARTEXT_PACKAGE_NAME = '@fhevm/host-contracts-cleartext';

// The account the local cleartext stack is deployed from. COPIED from sdk/cleartext-config.json's
// `localhost` block (the published package faces every value of that block except these); a generated
// face replaces this literal. Every stack address is CREATE(deployer, startNonce + k), so all four move
// together or not at all.
export const LOCALHOST_DEPLOYER = {
  mnemonic: 'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer',
  path: "m/44'/60'/0'/0/5",
  address: '0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4',
  startNonce: 0,
} as const;
