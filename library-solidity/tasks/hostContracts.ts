// Host contracts used by the library-solidity tests and examples, copied from @fhevm/host-contracts into fhevmTemp/.
import fs from 'node:fs';
import path from 'node:path';

const LIBRARY_SOLIDITY_DIR = path.resolve(import.meta.dirname, '..');

/** Where the host contracts are copied (a source folder of hardhat.config.ts). */
export const FHEVM_TEMP_CONTRACTS_DIR = path.join(LIBRARY_SOLIDITY_DIR, 'fhevmTemp/contracts');

/** Host contract addresses, imported by examples/CoprocessorSetup.sol and by the host contracts themselves. */
export const FHEVM_HOST_ADDRESSES_SOL = path.join(LIBRARY_SOLIDITY_DIR, 'fhevmTemp/addresses/FHEVMHostAddresses.sol');

/** `true` when fhevmTemp/ holds everything needed to compile fhevmTemp/contracts and examples/. */
export function isFhevmTempPrepared(): boolean {
  return fs.existsSync(FHEVM_TEMP_CONTRACTS_DIR) && fs.existsSync(FHEVM_HOST_ADDRESSES_SOL);
}

/**
 * Copies the host contracts from @fhevm/host-contracts into fhevmTemp/contracts.
 *
 * The host bridge contracts pull in LayerZero dependencies that library-solidity does not ship, so they are excluded
 * from the compile set. ACL.sol, however, imports the dependency-free IConfidentialBridge interface (to resync the LZ
 * delegate on ownership transfer), so that single file is kept while the rest of bridge/ is dropped.
 */
export function copyHostContracts(): void {
  const hostContractsSrcDir = path.resolve(LIBRARY_SOLIDITY_DIR, '../node_modules/@fhevm/host-contracts/contracts');
  if (!fs.existsSync(hostContractsSrcDir)) {
    throw new Error(`${hostContractsSrcDir} does not exist: run npm install from the repository root.`);
  }
  fs.mkdirSync(FHEVM_TEMP_CONTRACTS_DIR, { recursive: true });
  fs.cpSync(hostContractsSrcDir, FHEVM_TEMP_CONTRACTS_DIR, { recursive: true, dereference: true });

  const bridgeDstDir = path.join(FHEVM_TEMP_CONTRACTS_DIR, 'bridge');
  const keptBridgeInterface = path.join(bridgeDstDir, 'interfaces/IConfidentialBridge.sol');
  const keptBridgeInterfaceContents = fs.readFileSync(keptBridgeInterface);
  fs.rmSync(bridgeDstDir, { recursive: true, force: true });
  fs.mkdirSync(path.dirname(keptBridgeInterface), { recursive: true });
  fs.writeFileSync(keptBridgeInterface, keptBridgeInterfaceContents);
}
