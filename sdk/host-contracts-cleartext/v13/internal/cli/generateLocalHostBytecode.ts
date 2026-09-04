// Run: npm run generate:local-host-bytecode
//
// Emits both cleartext variants every time: the standard blobs and the CLEARTEXT_FORGE_* ones. There
// is no mode to choose — DeployLocalStack.s.sol broadcasts and takes the standard blobs, FhevmDeploy.sol
// runs in-process under forge and takes the Forge ones. See FORGE_VARIANTS in the module.
//
// Must run from the package root, and after `forge build`'s inputs exist: it repoints remappings.txt at a
// temporary config, builds with its own --out, and restores the file afterwards. See the module for why
// the addresses are compiled in rather than patched.

import { dirname, join, relative } from 'node:path';
import { ADDRESS_NAMES } from '../constants.ts';
import { ADDRESSES_OUTPUT_PATH, CODE_KIND, OUTPUT_PATH, writeLocalHostBytecode } from '../generateLocalHostBytecode.ts';

const { stack, code, interfaces } = writeLocalHostBytecode();

let total = 0;
for (const [contractName, hex] of code) {
  total += hex.length / 2;
  console.log(
    `  ${contractName.padEnd(24)} ${CODE_KIND[contractName].padEnd(9)} ${String(hex.length / 2).padStart(6)} B`,
  );
}
console.log(
  `\n  deployer ${stack.deployer} (index ${String(stack.deployerAddressIndex)}), ACL at ${stack.byName.ACL_ADDRESS}`,
);
console.log(`  ${String(total)} B of bytecode -> ${relative(process.cwd(), OUTPUT_PATH)}`);
console.log(
  `  ${String(stack.nonceSequence.length)} nonces, ${String(ADDRESS_NAMES.length)} addresses -> ${relative(process.cwd(), ADDRESSES_OUTPUT_PATH)}`,
);
console.log(
  `  ${String(interfaces.length)} interfaces -> ${relative(process.cwd(), join(dirname(OUTPUT_PATH), 'interfaces'))}`,
);
