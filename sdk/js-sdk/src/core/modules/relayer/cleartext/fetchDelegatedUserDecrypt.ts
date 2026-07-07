import type { FetchUserDecryptResultItem } from '../../../types/relayer.js';
import type { CleartextEthereumModule } from '../../ethereum/types-ct.js';
import type {
  FetchDelegatedUserDecryptParameters,
  FetchUserDecryptReturnType,
  RelayerClientWithRuntime,
} from '../types.js';
import { remove0x } from '../../../base/string.js';
import { randomUniqueUints } from '../../../base/uint.js';
import { getTrustedClient } from '../../../runtime/CoreFhevm-p.js';
import { userDecryptResultToKmsSigncryptedShares } from '../utils.js';
import { getKmsSignersPrivateKeyMap } from './signers.js';
import {
  readCleartextExecutorAddress,
  readKmsSignersAndThreshold,
  readPlaintexts,
  xorMaskWithPublicKey,
} from './plaintextSource.js';

////////////////////////////////////////////////////////////////////////////////

export async function fetchDelegatedUserDecrypt(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchDelegatedUserDecryptParameters,
): Promise<FetchUserDecryptReturnType> {
  const cleartextEthereumModule = relayerClient.runtime.ethereum as CleartextEthereumModule;
  const { payload } = parameters;
  const pairs = payload.handleContractPairs;
  const contractAddresses = payload.kmsDecryptEip712Message.contractAddresses;

  // Replicates CleartextKMSVerifier._requireAllPairsAuthorized: every requested
  // pair's contract must appear in the EIP-712 authorized contractAddresses list.
  const authCount = contractAddresses.length;
  for (const { contractAddress: c } of pairs) {
    let authorized: boolean = false;
    for (let i = 0; i < authCount; ++i) {
      if (contractAddresses[i] == c) {
        authorized = true;
        break;
      }
    }
    if (!authorized) {
      throw new Error(`ContractAddressNotAuthorized: ${c} is not in the EIP-712 authorized contractAddresses list.`);
    }
  }

  const { kmsDecryptEip712Message } = payload;

  // Option B: read cleartexts from CleartextFHEVMExecutor.plaintexts and rebuild
  // the KMS payload off-chain. The delegation (delegator/delegate) authorization
  // is enforced by the caller (fetchKmsSigncryptedSharesV1 step 7, checkDelegation).
  const trustedClient = getTrustedClient(relayerClient);
  const executorAddress = await readCleartextExecutorAddress(relayerClient, trustedClient);
  const rawCleartexts = await readPlaintexts(
    relayerClient,
    trustedClient,
    executorAddress,
    pairs.map((p) => p.handle),
  );

  const maskedCleartexts = xorMaskWithPublicKey(kmsDecryptEip712Message.publicKey, rawCleartexts);
  const extraData = kmsDecryptEip712Message.extraData;

  const commonKmsSigncryptedSharePayload = relayerClient.runtime.ethereum.encode({
    types: ['uint256[]', 'bytes'],
    values: [maskedCleartexts, extraData],
  });
  const commonKmsSigncryptedSharePayloadNo0x = remove0x(commonKmsSigncryptedSharePayload);

  const { signers: signersAddress, threshold } = await readKmsSignersAndThreshold(relayerClient, trustedClient);
  const signers = getKmsSignersPrivateKeyMap(relayerClient);

  const randomSignersAddress = randomUniqueUints(signersAddress.length, Number(threshold)).map(
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    (i) => signersAddress[i]!,
  );

  const result: FetchUserDecryptResultItem[] = [];

  for (const signerAddress of randomSignersAddress) {
    const privateKey = signers.get(signerAddress);
    if (privateKey === undefined) {
      throw new Error(`Unable to find KMS signer for address ${signerAddress}`);
    }

    const signature = await cleartextEthereumModule.sign({ hash: commonKmsSigncryptedSharePayload, privateKey });
    result.push({ signature: remove0x(signature), payload: commonKmsSigncryptedSharePayloadNo0x, extraData });
  }

  return userDecryptResultToKmsSigncryptedShares(result);
}
