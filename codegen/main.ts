import { mkdirSync, writeFileSync } from 'fs';

import { ALL_OPERATORS, Network, SUPPORTED_BITS, checks, networkCodegenContext } from './common';
import * as t from './templates';
import * as testgen from './testgen';

function generateAllFiles() {
  const numSplits = 12;
  const operators = checks(ALL_OPERATORS);

  const network = Network[(process.env.TARGET_NETWORK as keyof typeof Network) || 'Evmos'];
  const context = networkCodegenContext(network);
  const [tfheSolSource, overloads] = t.tfheSol(context, operators, SUPPORTED_BITS, false);
  const ovShards = testgen.splitOverloadsToShards(overloads);
  writeFileSync('lib/Impl.sol', t.implSol(context, operators));
  writeFileSync('lib/TFHE.sol', tfheSolSource);
  writeFileSync('lib/FhevmLib.sol', t.fhevmLibSol(operators));
  mkdirSync('examples/tests', { recursive: true });
  ovShards.forEach((os) => {
    writeFileSync(`examples/tests/TFHETestSuite${os.shardNumber}.sol`, testgen.generateSmartContract(os));
  });
  const tsSplits: string[] = testgen.generateTestCode(ovShards, numSplits);
  tsSplits.forEach((split, splitIdx) => writeFileSync(`test/tfheOperations/tfheOperations${splitIdx + 1}.ts`, split));
}

generateAllFiles();
