// Step 1 of zama-ai/planning-blockchain#1839. Run it before the execution of any proposal. It checks the initial state of the test.
import { task } from 'hardhat/config';

import { check, createSdk, expectSuccess, publicDecrypt, run, storeInput } from './checks';

task('task:allowlist-check-before-proposals', 'Checks the initial state, before any proposal').setAction(
  async (_, hre) =>
    run(hre, async (ctx) => {
      const { hcuLimit, dappA, dappB, operator } = ctx;
      check('getVersion()', 'HCULimit v0.5.0', await hcuLimit.getVersion());
      check('isAllowlistActivated()', false, await hcuLimit.isAllowlistActivated());
      check('getGlobalHCUCapPerBlock()', 281474976710655n, await hcuLimit.getGlobalHCUCapPerBlock());
      check('getMaxHCUDepthPerTx()', 5000000n, await hcuLimit.getMaxHCUDepthPerTx());
      check('getMaxHCUPerTx()', 20000000n, await hcuLimit.getMaxHCUPerTx());
      const receipt = await expectSuccess('A.addTrivialValues(40, 2)', dappA.addTrivialValues(40, 2));
      await expectSuccess('B.addTrivialValues(40, 2)', dappB.addTrivialValues(40, 2));
      check('the operator wallet has funds', true, (await hre.ethers.provider.getBalance(operator.address)) > 0n);
      const sdk = await createSdk(ctx);
      const result = await dappA.result({ blockTag: receipt.blockNumber });
      check('public decryption of A.result()', 42n, await publicDecrypt(sdk, result));
      check('public decryption of B.stored()', 4321n, await publicDecrypt(sdk, await storeInput(ctx, sdk, 4321n, '')));
    }),
);
