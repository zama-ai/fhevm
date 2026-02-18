import { existsSync, readFileSync } from 'fs';
import * as path from 'path';
import { isDeepStrictEqual } from 'util';

import { OverloadSignature } from './common';
import {
  type TestGroup,
  debugLog,
  errorLog,
  formatAndWriteFile,
  generatePrettierConfig,
  getUserConfig,
  getUserOverloadsFile,
  mkDir,
  resolveUserConfig,
  toAbsoluteFileWithExtension,
  toAbsulteConfig,
  toImportsCode,
} from './config';
import { ALL_FHE_TYPE_INFOS } from './fheTypeInfos';
import { type OverloadTests, generateOverloads } from './generateOverloads';
import { generateSolidityHCULimit } from './hcuLimitGenerator';
import { ALL_OPERATORS } from './operators';
import { ALL_OPERATORS_PRICES } from './operatorsPrices';
import { fromDirToFile, fromFileToFile, isDirectory } from './paths';
import { generateFhevmECDSALib, generateSolidityFHELib } from './templateFHEDotSol';
import { generateSolidityFheType } from './templateFheTypeDotSol';
import { generateSolidityImplLib } from './templateImpDotSol';
import {
  type TypescriptTestGroupImports,
  generateSolidityOverloadTestFiles,
  generateSolidityUnitTestContracts,
  generateTypeScriptTestCode,
  splitOverloadsToShards,
} from './testgen';
import { toBigInt } from './utils';
import { validate } from './validate';

export function readOverloads(overloadsJsonFile: string): OverloadTests | undefined {
  if (!existsSync(overloadsJsonFile)) {
    return undefined;
  }

  debugLog(`Read existing overloads file at ${overloadsJsonFile}`);
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
    debugLog(`Save new overloads file at ${overloadsFile}`);
    await formatAndWriteFile(
      overloadsFile,
      JSON.stringify(newOverloads, (_key, value) => {
        return typeof value === 'bigint' ? value.toString() + 'n' : value;
      }),
    );
  } else {
    debugLog(`Overloads file is unchanged.`);
  }
}

export async function commandRegenerateOverloads(outputFile: string, options: any) {
  if (isDirectory(outputFile)) {
    outputFile = path.join(outputFile, 'overloads.json');
  }

  const userConfig = getUserConfig();
  const userOverloadsJson = outputFile;
  const config = resolveUserConfig(userConfig);
  const absConfig = toAbsulteConfig(config);

  const defaultOverloadsJsonFile = absConfig.tests.overloads;
  const resolvedOverloadsJsonFile = userOverloadsJson
    ? toAbsoluteFileWithExtension(userOverloadsJson, '.json', process.cwd())
    : defaultOverloadsJsonFile;

  debugLog(`overloads.json (default):  ${defaultOverloadsJsonFile}`);
  debugLog(`overloads.json (resolved): ${resolvedOverloadsJsonFile}`);

  if (existsSync(resolvedOverloadsJsonFile)) {
    if (options.force !== true) {
      errorLog(`File ${resolvedOverloadsJsonFile} already exists. Use the --force option to overwrite it.`);
      process.exit(1);
    }
  }

  const update = options.update === true;
  const existingOverloadTests: OverloadTests = update ? (readOverloads(resolvedOverloadsJsonFile) ?? {}) : {};
  const overloadTests: OverloadTests = generateOverloads(ALL_FHE_TYPE_INFOS, existingOverloadTests);

  await writeOverloadsIfChanged(overloadTests, existingOverloadTests, resolvedOverloadsJsonFile);
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
export async function commandGenerateAllFiles(options: any) {
  const userConfig = getUserConfig();
  const userOverloadsJson = getUserOverloadsFile(options);
  const config = resolveUserConfig(userConfig);

  if (!options.test) {
    config.noTest = true;
  }
  if (!options.lib) {
    config.noLib = true;
  }

  const absConfig = toAbsulteConfig(config);

  generatePrettierConfig(absConfig.baseDir);

  debugLog(JSON.stringify(absConfig, null, 2));
  const numberOfTestSplits = config.tests.numberOfTestSplits;

  validate();

  const fheTypesDotSol = `${path.join(absConfig.lib.fheTypeDir, 'FheType.sol')}`;
  const implDotSol = `${path.join(absConfig.lib.outDir, 'Impl.sol')}`;
  const ecdsaDotSol = `${path.join(absConfig.lib.outDir, 'cryptography', 'FhevmECDSA.sol')}`;
  const fheDotSol = `${path.join(absConfig.lib.outDir, 'FHE.sol')}`;
  const hcuLimitDotSol = `${path.join(absConfig.hostContracts.outDir, 'HCULimit.sol')}`;

  const defaultOverloadsJsonFile = absConfig.tests.overloads;
  const resolvedOverloadsJsonFile = userOverloadsJson
    ? toAbsoluteFileWithExtension(userOverloadsJson, '.json', process.cwd())
    : defaultOverloadsJsonFile;

  if (!existsSync(resolvedOverloadsJsonFile)) {
    throw new Error(`Missing overloads file: ${resolvedOverloadsJsonFile}`);
  }

  const implRelFheTypesDotSol = fromFileToFile(implDotSol, fheTypesDotSol);
  const fheRelFheTypesDotSol = fromFileToFile(fheDotSol, fheTypesDotSol);
  const fheRelImplDotSol = fromFileToFile(fheDotSol, implDotSol);
  const fheRelEcdsaDotSol = fromFileToFile(fheDotSol, ecdsaDotSol);

  debugLog(`============ Config ============`);
  debugLog(`basePath:           ${absConfig.baseDir}`);
  debugLog(`noLib:              ${absConfig.noLib}`);
  debugLog(`noHostContracts:    ${absConfig.noHostContracts}`);
  debugLog(`noTest:             ${absConfig.noTest}`);
  debugLog(`============= Lib =============`);
  debugLog(`libDir:             ${absConfig.lib.outDir}`);
  debugLog(`FhevmECDSA.sol:     ${ecdsaDotSol}`);
  debugLog(`Impl.sol:           ${implDotSol}`);
  debugLog(`FHE.sol:            ${fheDotSol}`);
  debugLog(`fheTypeDir:         ${absConfig.lib.fheTypeDir}`);
  debugLog(`FheType.sol (absolute):             ${fheTypesDotSol}`);
  debugLog(`FheType.sol (relative to Impl.sol): ${implRelFheTypesDotSol}`);

  if (!absConfig.noHostContracts) {
    debugLog(`============= Host Contracts =============`);
    debugLog(`hostContractsDir:   ${absConfig.hostContracts.outDir}`);
    debugLog(`HCULimit.sol:       ${hcuLimitDotSol}`);
  }

  debugLog(`============ Tests ============`);
  debugLog(`numberOfTestSplits: ${absConfig.tests.numberOfTestSplits}`);
  debugLog(`publicDecrypt:      ${absConfig.tests.publicDecrypt}`);
  debugLog(`solidityDir:        ${absConfig.tests?.solidity?.outDir ?? 'N/A'}`);
  debugLog(`typescriptDir:      ${absConfig.tests?.typescript?.outDir ?? 'N/A'}`);
  debugLog(`parentContractName: ${absConfig.tests?.solidity?.parentContractName ?? 'N/A'}`);

  debugLog(`overloads.json (default):  ${defaultOverloadsJsonFile}`);
  debugLog(`overloads.json (resolved): ${resolvedOverloadsJsonFile}`);

  debugLog(`===============================`);

  if (config.noLib !== true) {
    const fheTypesCode = generateSolidityFheType(ALL_FHE_TYPE_INFOS);
    const implCode = generateSolidityImplLib(ALL_OPERATORS, implRelFheTypesDotSol);
    const fheCode = generateSolidityFHELib({
      operators: ALL_OPERATORS,
      fheTypes: ALL_FHE_TYPE_INFOS,
      fheTypeDotSol: fheRelFheTypesDotSol,
      implDotSol: fheRelImplDotSol,
      ecdsaDotSol: fheRelEcdsaDotSol,
    });
    const ecdsaCode = generateFhevmECDSALib();

    mkDir(path.dirname(fheTypesDotSol));
    mkDir(path.dirname(implDotSol));
    mkDir(path.dirname(ecdsaDotSol));
    mkDir(path.dirname(fheDotSol));

    // Generate core Solidity contract files.
    await formatAndWriteFile(`${fheTypesDotSol}`, fheTypesCode);
    await formatAndWriteFile(`${implDotSol}`, implCode);
    await formatAndWriteFile(`${ecdsaDotSol}`, ecdsaCode);
    await formatAndWriteFile(`${fheDotSol}`, fheCode);
  } else {
    debugLog(`Skipping lib generation.`);
  }

  if (config.noHostContracts !== true) {
    const hcuCode = generateSolidityHCULimit(ALL_OPERATORS_PRICES);
    // host contracts directory must exist.
    // Generate Host contracts contract files.
    await formatAndWriteFile(`${hcuLimitDotSol}`, hcuCode);
  } else {
    debugLog(`Skipping host contracts generation.`);
  }

  if (config.noTest === true) {
    debugLog(`Skipping test generation.`);
    return;
  }

  const existingOverloadTests: OverloadTests = readOverloads(resolvedOverloadsJsonFile) ?? {};
  // Generates a list of Overload Tests.
  // 1. if one test one test already exists, keep it.
  // 2. if one test does not exist, generate it.
  // overloadTests === { existing tests } U { missing tests }
  const overloadTests: OverloadTests = generateOverloads(ALL_FHE_TYPE_INFOS, existingOverloadTests);
  if (!userOverloadsJson) {
    // No `--overloads` option: save the new overloads tests if changed
    await writeOverloadsIfChanged(overloadTests, existingOverloadTests, defaultOverloadsJsonFile);
  } else {
    // With `--overloads` option: we expect Card({ missing tests }) == 0
    if (!isDeepStrictEqual(overloadTests, existingOverloadTests)) {
      throw new Error(
        `Invalid overloads.json file at ${resolvedOverloadsJsonFile}. Please regenerate 'overloads.json'. Type "codegen overloads --help" for more info.`,
      );
    }
  }

  const overloadTestFilesCode: OverloadSignature[] = generateSolidityOverloadTestFiles(
    ALL_OPERATORS,
    ALL_FHE_TYPE_INFOS,
  );
  const overloadShards = splitOverloadsToShards(overloadTestFilesCode, config.tests);

  // Solidity
  if (absConfig.tests?.solidity) {
    const solidityTestGroup = absConfig.tests.solidity;

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
        formatAndWriteFile(
          path.join(solidityTestGroup.outDir, `FHEVMTestSuite${os.shardNumber}.sol`),
          generateSolidityUnitTestContracts(
            os,
            importsCode,
            solidityTestGroup.parentContractName,
            config.tests.publicDecrypt,
          ),
        ),
      ),
    );
  } else {
    debugLog(`No Solidity tests.`);
  }

  // Operations
  if (absConfig.tests?.typescript) {
    const typescriptTestGroup = absConfig.tests.typescript;
    const typescriptTestGroupImports = computeOperationsTestsGroupImports(typescriptTestGroup);

    mkDir(typescriptTestGroup.outDir);
    const tsSplits: string[] = generateTypeScriptTestCode(
      overloadShards,
      numberOfTestSplits,
      overloadTests,
      typescriptTestGroupImports,
      config.tests,
    );
    tsSplits.forEach((split, splitIdx) =>
      formatAndWriteFile(path.join(typescriptTestGroup.outDir, `fhevmOperations${splitIdx + 1}.ts`), split),
    );
  } else {
    debugLog(`No Typescript tests.`);
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
