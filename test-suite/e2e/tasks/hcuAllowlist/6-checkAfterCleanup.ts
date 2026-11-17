// Step 6 of zama-ai/planning-blockchain#1839. Run it after the execution of the cleanup proposal. It checks the final state of the test.
import { task } from 'hardhat/config';

import { check, run } from './checks';

task('task:allowlist-check-after-cleanup', 'Checks the final state, after the cleanup proposal').setAction(
  async (_, hre) =>
    run(hre, async ({ hcuLimit, addressA, addressB, operator }) => {
      check('isAllowlistActivated()', false, await hcuLimit.isAllowlistActivated());
      check('isAllowlisted(A)', false, await hcuLimit.isAllowlisted(addressA));
      check('isAllowlisted(B)', false, await hcuLimit.isAllowlisted(addressB));
      check('isAllowlister(operator)', false, await hcuLimit.isAllowlister(operator.address));
      check('getGlobalHCUCapPerBlock()', 281474976710655n, await hcuLimit.getGlobalHCUCapPerBlock());
      check('getMaxHCUDepthPerTx()', 5000000n, await hcuLimit.getMaxHCUDepthPerTx());
      check('getMaxHCUPerTx()', 20000000n, await hcuLimit.getMaxHCUPerTx());
    }),
);
