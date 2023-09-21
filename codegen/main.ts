import { mkdirSync, writeFileSync } from 'fs';

import { ALL_OPERATORS, ALL_PRECOMPILES, SUPPORTED_BITS, checks } from './common';
import * as t from './templates';
import * as testgen from './testgen';

function generateAllFiles() {
  const operators = checks(ALL_OPERATORS);

  const [tfheSolSource, overloads] = t.tfheSol(operators, SUPPORTED_BITS);
  const ovShards = testgen.splitOverloadsToShards(overloads);
  writeFileSync('lib/Impl.sol', t.implSol(operators));
  writeFileSync('lib/TFHE.sol', tfheSolSource);
  mkdirSync('examples/tests', { recursive: true });
  ovShards.forEach((os) => {
    writeFileSync(`examples/tests/TFHETestSuite${os.shardNumber}.sol`, testgen.generateSmartContract(os));
  });
  writeFileSync('test/tfheOperations/tfheOperations.ts', testgen.generateTestCode(ovShards));
}

generateAllFiles();
