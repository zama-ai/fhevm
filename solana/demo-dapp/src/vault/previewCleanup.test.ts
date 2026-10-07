import { expect, test } from 'vitest';
import { AccountRole, address, createNoopSigner } from '@solana/kit';
import { TOKEN_PROGRAM_ADDRESS } from '@solana-program/token';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, findVaultAuthorityPda, getVaultAuthorityPdaSeeds } from '@fhevm/confidential-token';
import {
  getPreviewCloseTokenInstruction,
  PREVIEW_CLOSE_TOKEN_DISCRIMINATOR,
} from './internal/generated/demoVault/instructions/previewCloseToken.js';
import {
  getPreviewDrainInstruction,
  PREVIEW_DRAIN_DISCRIMINATOR,
} from './internal/generated/demoVault/instructions/previewDrain.js';
import fixture from '../../../test-fixtures/pda/pda_v1.json';

test('preview recovery supplies every account when reusing demo builders for confidential-token', async () => {
  const admin = createNoopSigner(address(fixture.inputs.payer));
  const programData = address(fixture.inputs.program);
  const mint = address(fixture.inputs.mint);
  const account = address(fixture.inputs.owner);
  const systemProgram = address('11111111111111111111111111111111');
  const [authority, bump] = await findVaultAuthorityPda({ mint }, { programAddress: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS });
  const seeds = [...getVaultAuthorityPdaSeeds({ mint }), new Uint8Array([bump])];
  const config = { programAddress: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS };
  const close = getPreviewCloseTokenInstruction({
    admin, programData, authority, account, mint, tokenProgram: TOKEN_PROGRAM_ADDRESS, seeds,
  }, config);
  const drain = getPreviewDrainInstruction({ admin, programData, authority, systemProgram, seeds }, config);

  expect(close.programAddress).toBe(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS);
  expect(drain.programAddress).toBe(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS);
  expect(close.data.subarray(0, 8)).toEqual(PREVIEW_CLOSE_TOKEN_DISCRIMINATOR);
  expect(drain.data.subarray(0, 8)).toEqual(PREVIEW_DRAIN_DISCRIMINATOR);
  const metas = (instruction: typeof close | typeof drain) => instruction.accounts.map(({ address, role }) => ({ address, role }));
  expect(metas(close)).toEqual([
    { address: admin.address, role: AccountRole.WRITABLE_SIGNER },
    { address: programData, role: AccountRole.READONLY },
    { address: authority, role: AccountRole.READONLY },
    { address: account, role: AccountRole.WRITABLE },
    { address: mint, role: AccountRole.WRITABLE },
    { address: TOKEN_PROGRAM_ADDRESS, role: AccountRole.READONLY },
  ]);
  expect(metas(drain)).toEqual([
    { address: admin.address, role: AccountRole.WRITABLE_SIGNER },
    { address: programData, role: AccountRole.READONLY },
    { address: authority, role: AccountRole.WRITABLE },
    { address: systemProgram, role: AccountRole.READONLY },
  ]);
});
