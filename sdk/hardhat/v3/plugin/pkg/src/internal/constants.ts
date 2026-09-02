// Plugin-local constants. Addresses never live here: they come from the generated chain constants
// and from @fhevm/host-contracts-cleartext.

export const PLUGIN_ID = 'fhevm';

/** Chain id of every development node (hardhat's and anvil's default); the cleartext stack targets it. */
export const DEVELOPMENT_CHAIN_ID = 31337;

/** Sibling npm packages the plugin locates in the USER's project, never in its own tree. */
export const FHEVM_SOLIDITY_PACKAGE_NAME = '@fhevm/solidity';
export const FHEVM_SOLIDITY_CONFIG_FILE = 'config/ZamaConfig.sol';
export const FHEVM_SDK_PACKAGE_NAME = '@fhevm/sdk';

/** The plugin's OWN dependency (not a consumer sibling): ABIs and the deploy sequence of the cleartext stack. */
export const FHEVM_HOST_CONTRACTS_CLEARTEXT_PACKAGE_NAME = '@fhevm/host-contracts-cleartext';
