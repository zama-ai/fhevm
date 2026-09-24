// User decrypt happy path: deploy UserDecrypt (values allowed to alice), sign the unified EIP-712 request as
// alice, POST the relayer envelope, check the shares structurally (the SDK reconstructs plaintexts only inside its
// own relayer call). The SDK is used for the transport keypair only.
import { createInstances, verifyingContractAddressDecryption } from '../../test/instance';
import {
  UNIFIED_ATTESTATION_TYPE,
  type UnifiedDecryptRequest,
  backdatedStartTimestamp,
  signRequest,
} from '../../test/sdk/unified/unifiedUserDecrypt';
import { RELAYER, alice, deploy, fail, postUntilSettled, requireRelayer, run } from './common';

interface UserReply {
  status: string;
  requestId: string;
  result: { result: { payload: string; signature: string; extraData: string }[] };
}

const MIN_SHARES = Number(process.env.MIN_SHARES ?? '1');

run(async () => {
  await requireRelayer();
  const { signers, alice: signer } = await alice();
  const fixture = await deploy('UserDecrypt', signer);
  const contractAddress = await fixture.getAddress();
  const handle = (await fixture.getFunction('xUint64')()) as string;

  let publicKey: string;
  try {
    const instances = await createInstances(signers);
    ({ publicKey } = await instances.alice.generateKeypair());
  } catch (e) {
    console.error(`SDK keypair failed: ${(e as Error).message}`);
    console.error('the SDK fetches the FHE key from the old relayer at init: `docker start fhevm-relayer` and retry');
    process.exit(1);
  }

  const request: UnifiedDecryptRequest = {
    handles: [{ ctHandle: handle, contractAddress, ownerAddress: signer.address }],
    userAddress: signer.address,
    allowedContracts: [contractAddress],
    publicKey,
    startTimestamp: backdatedStartTimestamp(),
    durationSeconds: 3600,
    extraData: '0x00',
  };
  const signature = await signRequest(
    { relayerUrl: RELAYER, decryptionContractAddress: verifyingContractAddressDecryption },
    request,
    { kind: 'eoa', signer },
  );
  console.log(`handle: ${handle}`);

  const reply = await postUntilSettled<UserReply>('/v4/exp/user-decrypt', {
    attestationType: UNIFIED_ATTESTATION_TYPE,
    payload: {
      handles: request.handles,
      userAddress: request.userAddress,
      publicKey: request.publicKey,
      allowedContracts: request.allowedContracts,
      requestValidity: { startTimestamp: request.startTimestamp, durationSeconds: request.durationSeconds },
      extraData: request.extraData,
    },
    signature,
  });
  if (reply.status !== 200) fail('user decrypt', reply);
  const shares = reply.body.result.result;
  const malformed = shares.filter((s) => !/^[0-9a-f]{2,}$/i.test(s.payload) || !/^[0-9a-f]{130}$/i.test(s.signature));
  console.log(
    `user decrypt: ${reply.body.status}, ${shares.length} shares, extraData=${shares[0]?.extraData}, ` +
      `malformed=${malformed.length}`,
  );
  if (shares.length < MIN_SHARES || malformed.length > 0) {
    console.error(`expected at least ${MIN_SHARES} well-formed shares`);
    process.exit(1);
  }
  console.log('user decrypt OK');
});
