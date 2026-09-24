// Public decrypt happy path: deploy HTTPPublicDecrypt (three publicly decryptable values), POST the three
// handles to the relayer, check the clear values against the fixture.
import { decodeDecryptedResult } from '../../test/sdk/connector/verify';
import { alice, deploy, fail, postUntilSettled, requireRelayer, run, show } from './common';

interface PublicReply {
  status: string;
  requestId: string;
  result: { decryptedValue: string; signatures: string[]; extraData: string };
}

const EXPECTED = [true, 242n, '0xfC4382C084fCA3f4fB07c3BCDA906C01797595a8'];

run(async () => {
  await requireRelayer();
  const { alice: signer } = await alice();
  const fixture = await deploy('HTTPPublicDecrypt', signer);
  const handles = [
    (await fixture.getFunction('xBool')()) as string,
    (await fixture.getFunction('xUint32')()) as string,
    (await fixture.getFunction('xAddress')()) as string,
  ];
  console.log(`handles: ${show(handles)}`);

  const reply = await postUntilSettled<PublicReply>('/v4/exp/public-decrypt', { ciphertextHandles: handles, extraData: '0x00' });
  if (reply.status !== 200) fail('public decrypt', reply);
  const clear = decodeDecryptedResult(handles, `0x${reply.body.result.decryptedValue}`);
  console.log(
    `public decrypt: ${reply.body.status}, ${reply.body.result.signatures.length} signatures, ` +
      `extraData=${reply.body.result.extraData}, clear=${show(clear)}`,
  );
  if (show(clear) !== show(EXPECTED)) {
    console.error(`unexpected clear values, expected ${show(EXPECTED)}`);
    process.exit(1);
  }
  console.log('public decrypt OK');
});
