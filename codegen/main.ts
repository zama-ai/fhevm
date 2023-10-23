import { mkdirSync, writeFileSync } from 'fs';

import { ALL_OPERATORS, Network, SUPPORTED_BITS, checks, networkCodegenContext } from './common';
import * as t from './templates';
import * as testgen from './testgen';

function generateAllFiles() {
  const operators = checks(ALL_OPERATORS);

  const network = Network[(process.env.TARGET_NETWORK as keyof typeof Network) || 'Evmos'];
  const context = networkCodegenContext(network);
  const [tfheSolSource, overloads] = t.tfheSol(context, operators, SUPPORTED_BITS);
  const ovShards = testgen.splitOverloadsToShards(overloads);
  writeFileSync('lib/Impl.sol', t.implSol(context, operators));
  writeFileSync('lib/TFHE.sol', tfheSolSource);
  mkdirSync('examples/tests', { recursive: true });
  ovShards.forEach((os) => {
    writeFileSync(`examples/tests/TFHETestSuite${os.shardNumber}.sol`, testgen.generateSmartContract(os));
  });
  writeFileSync('test/tfheOperations/tfheOperations.ts', testgen.generateTestCode(ovShards));
  writeFileSync(
    'test/generated.ts',
    `
    export const FHE_LIB_ADDRESS = '${context.libFheAddress}';
  `,
  );
}

generateAllFiles();
