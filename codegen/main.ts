import { writeFileSync } from 'fs';

import { ALL_OPERATORS, ALL_PRECOMPILES, checks } from './common';
import * as t from './templates';

function generateAllFiles() {
  const operators = checks(ALL_OPERATORS);

  writeFileSync(`lib/Common.sol`, t.commonSolHeader());
  writeFileSync(`lib/Precompiles.sol`, t.precompiles(ALL_PRECOMPILES));
  writeFileSync(`lib/Impl.sol`, t.implSol(operators));
}

generateAllFiles();
