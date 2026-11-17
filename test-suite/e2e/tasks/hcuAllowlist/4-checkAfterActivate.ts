// Steps 3 and 4 of zama-ai/planning-blockchain#1839. Run it after the execution of the activate proposal.
// Run it one time only. The operator adds B to the allowlist during the run, so a second run cannot show that B is blocked.
// The rollback proposal can be executed as soon as the script prints the rollback note.
import { task } from 'hardhat/config';

import { attempt, check, createSdk, expectRevert, expectSuccess, note, publicDecrypt, run, storeInput } from './checks';

task('task:allowlist-check-after-activate', 'Runs the five rows of step 4 while the allowlist is activated').setAction(
  async (_, hre) =>
    run(hre, async (ctx) => {
      const { hcuLimit, dappA, dappB, addressA: a, addressB: b, user, operator } = ctx;
      const hcuLimitAddress = await hcuLimit.getAddress();

      // Stop here if the activate proposal is not executed yet.
      // The rows below make the operator add B to the allowlist, and only a DAO proposal can remove B.
      if (!check('isAllowlistActivated()', true, await hcuLimit.isAllowlistActivated())) {
        note('The script did not run rows 1 to 5 because the allowlist is not activated.');
        return;
      }
      const decryptions: [string, () => Promise<void>][] = [];

      // Row 1. Any wallet calls A.addTrivialValues(40, 2). The call succeeds, and A.result() decrypts to 42.
      await attempt('row 1', async () => {
        const receipt = await expectSuccess('row 1 A.addTrivialValues(40, 2)', dappA.addTrivialValues(40, 2));
        const handle = await dappA.result({ blockTag: receipt.blockNumber });
        decryptions.push([
          'row 1',
          async () => {
            check('row 1 public decryption of A.result()', 42n, await publicDecrypt(await createSdk(ctx), handle));
          },
        ]);
      });

      // Row 2. Any wallet calls B.addTrivialValues(40, 2). The call reverts with CallerNotAllowlisted(B).
      await attempt('row 2', () =>
        expectRevert(
          'row 2 B.addTrivialValues(40, 2)',
          user,
          b,
          dappB.interface.encodeFunctionData('addTrivialValues', [40, 2]),
          `CallerNotAllowlisted(${b})`,
        ),
      );

      // Row 3. Any wallet calls B.storeInput for the value 1234, then B.publishStored(). Both calls succeed, and B.stored() decrypts to 1234.
      await attempt('row 3', async () => {
        const handle = await storeInput(ctx, await createSdk(ctx), 1234n, 'row 3 ');
        decryptions.push([
          'row 3',
          async () => {
            check('row 3 public decryption of B.stored()', 1234n, await publicDecrypt(await createSdk(ctx), handle));
          },
        ]);
      });

      // Row 4. The operator calls HCULimit.addToAllowlist([B]), then B.addTrivialValues(40, 2). Both calls succeed with no proposal.
      await attempt('row 4', async () => {
        await expectSuccess(
          'row 4 HCULimit.addToAllowlist([B]) sent by the operator',
          hcuLimit.connect(operator).getFunction('addToAllowlist')([b]),
        );
        await expectSuccess(
          'row 4 B.addTrivialValues(40, 2) sent by the operator',
          dappB.connect(operator).addTrivialValues(40, 2),
        );
      });

      // Row 5. The operator calls setAllowlistActivated(false), then removeFromAllowlist([A]). Each call reverts with NotHostOwner.
      // The allowlist stays activated, and A stays allowlisted.
      await attempt('row 5', async () => {
        await expectRevert(
          'row 5 HCULimit.setAllowlistActivated(false) sent by the operator',
          operator,
          hcuLimitAddress,
          hcuLimit.interface.encodeFunctionData('setAllowlistActivated', [false]),
          `NotHostOwner(${operator.address})`,
        );
        await expectRevert(
          'row 5 HCULimit.removeFromAllowlist([A]) sent by the operator',
          operator,
          hcuLimitAddress,
          hcuLimit.interface.encodeFunctionData('removeFromAllowlist', [[a]]),
          `NotHostOwner(${operator.address})`,
        );
        check('row 5 isAllowlistActivated()', true, await hcuLimit.isAllowlistActivated());
        check('row 5 isAllowlisted(A)', true, await hcuLimit.isAllowlisted(a));
      });

      check('isAllowlisted(B), which must be true for the cleanup proposal', true, await hcuLimit.isAllowlisted(b));

      // The public decryptions run last. They can be slow, and the rollback proposal does not depend on them.
      note('The rollback proposal can be executed now. The script continues with the public decryptions.');

      for (const [row, decrypt] of decryptions) await attempt(row, decrypt);
    }),
);
