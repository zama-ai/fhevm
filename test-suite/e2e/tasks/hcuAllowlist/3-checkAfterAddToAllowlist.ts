// Step 2 of zama-ai/planning-blockchain#1839. Run it after the execution of the proposal that adds the operator and dapp A to the allowlist.
import { task } from 'hardhat/config';

import { check, run } from './checks';

task(
  'task:allowlist-check-after-add-to-allowlist',
  'Checks the state after the proposal that adds the operator and dapp A',
).setAction(async (_, hre) =>
  run(hre, async ({ hcuLimit, addressA, addressB, operator }) => {
    check('isAllowlisted(A)', true, await hcuLimit.isAllowlisted(addressA));
    check('isAllowlisted(B)', false, await hcuLimit.isAllowlisted(addressB));
    check('isAllowlister(operator)', true, await hcuLimit.isAllowlister(operator.address));
    const deadline = Number(await hcuLimit.allowlisterDeadline(operator.address));
    check('the operator deadline is more than one hour away', true, deadline > Date.now() / 1000 + 3600);
    check('isAllowlistActivated()', false, await hcuLimit.isAllowlistActivated());
  }),
);
