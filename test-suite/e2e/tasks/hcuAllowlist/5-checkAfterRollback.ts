// Step 5 of zama-ai/planning-blockchain#1839. Run it after the execution of the rollback proposal.
import { task } from 'hardhat/config';

import { check, note, run } from './checks';

task('task:allowlist-check-after-rollback', 'Checks the state after the rollback proposal').setAction(async (_, hre) =>
  run(hre, async ({ hcuLimit, addressB }) => {
    // removeFromAllowlist([A, B]) of the cleanup proposal reverts if B is not allowlisted.
    check(
      'isAllowlisted(B), which must be true for the cleanup proposal',
      true,
      await hcuLimit.isAllowlisted(addressB),
    );
    if (check('isAllowlistActivated()', false, await hcuLimit.isAllowlistActivated())) {
      note(
        'ACTION. The infrastructure team must start the E2E workflows now. Attach the link to the workflow run to zama-ai/planning-blockchain#1839.',
      );
    }
  }),
);
