// Step 1.0 of the Hardhat 3 migration: a minimal script that checks the toolchain (ESM, tsc, ESLint, prettier,
// `hardhat run`). It is deleted in step 1.6.
import { ethers } from 'ethers';
import hre from 'hardhat';
import * as fs from 'node:fs';
import * as path from 'node:path';

const configFile = path.join(hre.config.paths.root, 'hardhat.config.ts');

console.log('Hello from Hardhat 3!');
console.log(`- project root:   ${hre.config.paths.root}`);
console.log(`- config exists:  ${fs.existsSync(configFile)}`);
console.log(`- solc version:   ${hre.config.solidity.profiles.default.compilers[0]?.version ?? 'none'}`);
console.log(`- ethers version: ${ethers.version}`);
