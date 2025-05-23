import { mkdirSync, writeFileSync } from 'fs';
import path from 'path';

import { validateFHETypes, validateOperators } from './common';
import { generateOverloads } from './generateOverloads';
import { ALL_OPERATORS } from './operators';
import {
  generateSolidityOverloadTestFiles,
  generateSolidityUnitTestContracts,
  generateTypeScriptTestCode,
  splitOverloadsToShards,
} from './testgen';
import { ALL_FHE_TYPES } from './types';

/**
 * Generates all necessary files including Solidity contracts and TypeScript test files.
 *
 * This function performs the following steps:
 * 1. Generates FHE types from a JSON file.
 * 2. Validates and processes the list of operators.
 * 3. Generates Solidity source code for FHE and implementation contracts.
 * 4. Splits the generated overloads into multiple shards to avoid exceeding Solidity's contract size limit.
 * 5. Writes the generated Solidity contracts and test files to the appropriate directories.
 * 6. Generates TypeScript test code for the split overloads and writes them to the test directory.
 *
 */
function generateAllFiles() {
  const numberOfTestSplits = 12;

  // Validate the FHE types
  validateFHETypes(ALL_FHE_TYPES);
  // Validate the operators
  validateOperators(ALL_OPERATORS);

  // TODO: For now, the testgen only supports automatically generated tests for euintXX.
  /// Generate overloads, split them into shards, and generate Solidity contracts to be used for TypeScript unit test files.
  writeFileSync(
    `${path.resolve(__dirname)}/overloads.json`,
    JSON.stringify(generateOverloads(ALL_FHE_TYPES), (_key, value) =>
      typeof value === 'bigint' ? value.toString() : value,
    ),
  );
  const overloadShards = splitOverloadsToShards(generateSolidityOverloadTestFiles(ALL_OPERATORS, ALL_FHE_TYPES));
  mkdirSync('contracts/operations', { recursive: true });
  overloadShards.forEach((os) => {
    writeFileSync(`contracts/operations/FHEVMTestSuite${os.shardNumber}.sol`, generateSolidityUnitTestContracts(os));
  });

  const tsSplits: string[] = generateTypeScriptTestCode(overloadShards, numberOfTestSplits);
  tsSplits.forEach((split, splitIdx) => writeFileSync(`test/fhevmOperations/fhevmOperations${splitIdx + 1}.ts`, split));
}

generateAllFiles();
