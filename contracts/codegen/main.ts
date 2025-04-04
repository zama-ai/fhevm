import { mkdirSync, writeFileSync } from 'fs';

import { ALL_OPERATORS, SUPPORTED_BITS, checks } from './common';
import operatorsPrices from './operatorsPrices.json';
import { generateFHEGasLimit } from './payments';
import * as t from './templates';
import * as testgen from './testgen';

function generateAllFiles() {
  const numberOfTestSplits = 12;
  if (!ALL_OPERATORS || !Array.isArray(ALL_OPERATORS) || ALL_OPERATORS.length === 0) {
    throw new Error('ALL_OPERATORS is not defined or invalid');
  }
  const operators = checks(ALL_OPERATORS);
  const [tfheSolSource, overloads] = t.tfheSol(operators, SUPPORTED_BITS, false);
  const overloadShards = testgen.splitOverloadsToShards(overloads);
  writeFileSync('lib/Impl.sol', t.implSol(operators));
  writeFileSync('lib/TFHE.sol', tfheSolSource);
  writeFileSync('contracts/FHEGasLimit.sol', generateFHEGasLimit(operatorsPrices));
  mkdirSync('contracts/tests', { recursive: true });
  overloadShards.forEach((os) => {
    writeFileSync(`examples/tests/TFHETestSuite${os.shardNumber}.sol`, testgen.generateSmartContract(os));
  });
  const tsSplits: string[] = testgen.generateTestCode(overloadShards, numberOfTestSplits);
  tsSplits.forEach((split, splitIdx) => writeFileSync(`test/tfheOperations/tfheOperations${splitIdx + 1}.ts`, split));
}

generateAllFiles();
