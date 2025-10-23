import { existsSync, readFileSync } from 'fs';
import * as path from 'path';
import { isDeepStrictEqual } from 'util';

import { validateFHETypes, validateOperators } from './common.js';
import {
  type TestGroup,
  debugLog,
  generatePrettierConfig,
  getUserConfig,
  getUserOverloadsFile,
  isDebug,
  mkDir,
  resolveUserConfig,
  toAbsoluteJsonFile,
  toAbsulteConfig,
  toImportsCode,
  writeFile,
} from './config.js';
import { type OverloadTests, generateOverloads } from './generateOverloads.js';
import { generateSolidityHCULimit } from './hcuLimitGenerator.js';
import { ALL_OPERATORS } from './operators.js';
import operatorsPrices from './operatorsPrices.json' with { type: 'json' };
import { generateSolidityFHELib, generateSolidityFheType, generateSolidityImplLib } from './templates.js';
import {
  type TypescriptTestGroupImports,
  generateSolidityOverloadTestFiles,
  generateSolidityUnitTestContracts,
  generateTypeScriptTestCode,
  splitOverloadsToShards,
} from './testgen.js';
import { ALL_FHE_TYPES } from './types.js';
import { fromDirToFile, fromFileToFile } from './utils/paths.js';

export function validate() {
  // Validate the FHE types
  validateFHETypes(ALL_FHE_TYPES);
  // Validate the operators
  validateOperators(ALL_OPERATORS);
}

function toBigInt(x: any): bigint | undefined {
  if (typeof x === 'bigint') return x;
  if (typeof x === 'number') return BigInt(x);
  if (typeof x === 'string') {
    const s = x.endsWith('n') ? x.slice(0, -1) : x; // strip trailing 'n'
    try {
      return BigInt(s);
    } catch {
      return undefined;
    }
  }
  return undefined;
}

export function readOverloads(overloadsJsonFile: string): OverloadTests | undefined {
  if (!existsSync(overloadsJsonFile)) {
    return undefined;
  }

  console.log(`Read existing overloads file at ${overloadsJsonFile}`);
  const json = readFileSync(overloadsJsonFile, 'utf8');

  return JSON.parse(json, (_key, value) => {
    if (typeof value !== 'string') {
      return value;
    }
    if (value === 'true') {
      return true;
    }
    if (value === 'false') {
      return false;
    }
    const bn = toBigInt(value);
    if (bn !== undefined) {
      return bn;
    }
    return value;
  });
}

export async function writeOverloadsIfChanged(
  newOverloads: OverloadTests,
  existingOverloads: OverloadTests,
  overloadsFile: string,
): Promise<void> {
  if (!isDeepStrictEqual(newOverloads, existingOverloads)) {
    console.log(`Save new overloads file at ${overloadsFile}`);
    await writeFile(
      overloadsFile,
      JSON.stringify(newOverloads, (_key, value) => {
        return typeof value === 'bigint' ? value.toString() + 'n' : value;
      }),
    );
  } else {
    console.log(`Overloads is unchanged.`);
  }
}

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
export async function generateAllFiles() {
  const userConfig = getUserConfig();
  const userOverloadsJson = getUserOverloadsFile();
  const config = resolveUserConfig(userConfig);
  const absConfig = toAbsulteConfig(config);

  generatePrettierConfig(absConfig.directories.baseDir);

  debugLog(JSON.stringify(absConfig, null, 2));
  const numberOfTestSplits = config.numberOfTestSplits;

  validate();

  const fheTypesDotSol = `${path.join(absConfig.directories.fheTypeDir, 'FheType.sol')}`;
  const implDotSol = `${path.join(absConfig.directories.libDir, 'Impl.sol')}`;
  const fheDotSol = `${path.join(absConfig.directories.libDir, 'FHE.sol')}`;
  const hcuLimitDotSol = `${path.join(absConfig.directories.contractsDir, 'HCULimit.sol')}`;
  const overloadsJsonFile = `${path.join(absConfig.directories.overloadsDir, 'overloads.json')}`;
  const existingOverloadsJsonFile = userOverloadsJson
    ? toAbsoluteJsonFile(userOverloadsJson, process.cwd())
    : overloadsJsonFile;

  const implRelFheTypesDotSol = fromFileToFile(implDotSol, fheTypesDotSol);
  const fheRelFheTypesDotSol = fromFileToFile(fheDotSol, fheTypesDotSol);
  const fheRelImplDotSol = fromFileToFile(fheDotSol, implDotSol);

  debugLog(`numberOfTestSplits: ${absConfig.numberOfTestSplits}`);
  debugLog(`basePath:           ${absConfig.directories.baseDir}`);
  debugLog(`fheTypeDir:         ${absConfig.directories.fheTypeDir}`);
  debugLog(`libDir:             ${absConfig.directories.libDir}`);
  debugLog(`Impl.sol:           ${implDotSol}`);
  debugLog(`FHE.sol:            ${fheDotSol}`);
  debugLog(`FheType.sol (absolute):             ${fheTypesDotSol}`);
  debugLog(`FheType.sol (relative to Impl.sol): ${implRelFheTypesDotSol}`);
  debugLog(`HCULimit.sol:       ${hcuLimitDotSol}`);
  debugLog(`solidityDir:        ${absConfig.solidity?.outDir ?? 'N/A'}`);
  debugLog(`typescriptDir:      ${absConfig.typescript?.outDir ?? 'N/A'}`);

  debugLog(`overloads.json:          ${overloadsJsonFile}`);
  debugLog(`existing overloads.json: ${existingOverloadsJsonFile}`);

  const fheTypesCode = generateSolidityFheType(ALL_FHE_TYPES);
  const implCode = generateSolidityImplLib(ALL_OPERATORS, implRelFheTypesDotSol);
  const fheCode = generateSolidityFHELib(ALL_OPERATORS, ALL_FHE_TYPES, fheRelFheTypesDotSol, fheRelImplDotSol);
  const hcuCode = generateSolidityHCULimit(operatorsPrices);

  if (isDebug()) {
    console.log(`FheType.sol:  size=${fheTypesCode.length}`);
    console.log(`Impl.sol:     size=${implCode.length}`);
    console.log(`FHE.sol:      size=${fheCode.length}`);
    console.log(`HCULimit.sol: size=${hcuCode.length}`);
  }

  if (config.noLib !== true) {
    mkDir(path.dirname(fheTypesDotSol));
    mkDir(path.dirname(implDotSol));
    mkDir(path.dirname(fheDotSol));

    /// Generate core Solidity contract files.
    await writeFile(`${fheTypesDotSol}`, fheTypesCode);
    await writeFile(`${implDotSol}`, implCode);
    await writeFile(`${fheDotSol}`, fheCode);
  }

  if (config.generateHCULimit === true) {
    await writeFile(`${hcuLimitDotSol}`, hcuCode);
  }

  const existingOverloadTests: OverloadTests = readOverloads(existingOverloadsJsonFile) ?? {};
  const overloadTests: OverloadTests = generateOverloads(ALL_FHE_TYPES, existingOverloadTests);
  if (!userOverloadsJson) {
    await writeOverloadsIfChanged(overloadTests, existingOverloadTests, overloadsJsonFile);
  } else {
    if (!isDeepStrictEqual(overloadTests, existingOverloadTests)) {
      throw new Error(`Invalid overloads.json file at ${existingOverloadsJsonFile}`);
    }
  }

  const overloadTestFilesCode = generateSolidityOverloadTestFiles(ALL_OPERATORS, ALL_FHE_TYPES);
  const overloadShards = splitOverloadsToShards(overloadTestFilesCode, config);

  // Solidity
  if (absConfig.solidity) {
    const solidityTestGroup = absConfig.solidity;

    const imports = [];

    // add FHE.sol import
    if (!solidityTestGroup.imports?.includes('@fhevm/solidity/lib/FHE.sol')) {
      const importFhe = fromDirToFile(solidityTestGroup.outDir, fheDotSol);
      imports.push(importFhe);
    }
    // concat imports
    if (solidityTestGroup.imports) {
      solidityTestGroup.imports.forEach((i) => imports.push(i));
    }
    // convert to solidity code
    const importsCode = toImportsCode(imports);

    mkDir(solidityTestGroup.outDir);
    /*
        library-solidity
        ================
        import "../../lib/FHE.sol";
        import {CoprocessorSetup} from "../CoprocessorSetup.sol";

        host-contracts
        ==============
        import "../../lib/FHE.sol";
        import {CoprocessorSetup} from "../../lib/CoprocessorSetup.sol";
    */
    await Promise.all(
      overloadShards.map((os) =>
        writeFile(
          path.join(solidityTestGroup.outDir, `FHEVMTestSuite${os.shardNumber}.sol`),
          generateSolidityUnitTestContracts(
            os,
            importsCode,
            solidityTestGroup.parentContract?.name,
            config.publicDecrypt,
          ),
        ),
      ),
    );
  }

  // Operations
  if (absConfig.typescript) {
    const typescriptTestGroup = absConfig.typescript;
    const typescriptTestGroupImports = computeOperationsTestsGroupImports(typescriptTestGroup);

    mkDir(typescriptTestGroup.outDir);
    const tsSplits: string[] = generateTypeScriptTestCode(
      overloadShards,
      numberOfTestSplits,
      overloadTests,
      typescriptTestGroupImports,
      config,
    );
    tsSplits.forEach((split, splitIdx) =>
      writeFile(path.join(typescriptTestGroup.outDir, `fhevmOperations${splitIdx + 1}.ts`), split),
    );
  }
}

function computeOperationsTestsGroupImports(operationsTestGroup: TestGroup): TypescriptTestGroupImports {
  const imports: TypescriptTestGroupImports = {
    signers: '',
    instance: '',
    typechain: '',
  };

  const importModuleNames = Object.keys(imports) as Array<keyof TypescriptTestGroupImports>;
  const importModuleNamesSet = new Set(importModuleNames);

  if (operationsTestGroup.imports) {
    for (let i = 0; i < operationsTestGroup.imports.length; ++i) {
      const imp = operationsTestGroup.imports[i];
      if (typeof imp === 'string') {
        continue;
      }
      const k = imp[0] as keyof TypescriptTestGroupImports;
      if (importModuleNamesSet.has(k)) {
        imports[k] = imp[1];
      } else {
        throw new Error(`Unkown operations import module name ${k}`);
      }
    }
  } else {
    throw new Error('Missing operations test group imports declaration.');
  }

  importModuleNames.forEach((k) => {
    if (imports[k].length === 0) {
      throw new Error(`Missing operations test group '${k}' import declaration.`);
    }
  });

  return imports;
}
