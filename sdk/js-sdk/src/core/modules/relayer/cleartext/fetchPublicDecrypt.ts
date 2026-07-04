import type { Bytes65Hex, BytesHex } from '../../../types/primitives.js';
import type { CleartextEthereumModule } from '../../ethereum/types-ct.js';
import type { FetchPublicDecryptParameters, FetchPublicDecryptReturnType, RelayerClientWithRuntime } from '../types.js';
import { getTrustedClient } from '../../../runtime/CoreFhevm-p.js';
import { createKmsEip712Domain } from '../../../kms/createKmsEip712Domain.js';
import { kmsPublicDecryptEip712Types } from '../../../kms/kmsPublicDecryptEip712Types.js';
import { getKmsSignersPrivateKeyMap } from './signers.js';
import {
  encodeTypedCleartexts,
  readCleartextExecutorAddress,
  readKmsSignersAndThreshold,
  readPlaintexts,
} from './plaintextSource.js';

////////////////////////////////////////////////////////////////////////////////

export async function fetchPublicDecrypt(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchPublicDecryptParameters,
): Promise<FetchPublicDecryptReturnType> {
  const cleartextEthereumModule = relayerClient.runtime.ethereum as CleartextEthereumModule;
  const orderedHandles = parameters.payload.orderedHandles;

  // Option B: read cleartexts from CleartextFHEVMExecutor.plaintexts and rebuild
  // the public-decrypt result off-chain instead of reading the CleartextKMSVerifier
  // view. ACL `isAllowedForDecryption` gating is enforced by the caller
  // (publicDecrypt step 4, checkAllowedForDecryption).
  const trustedClient = getTrustedClient(relayerClient);
  const executorAddress = await readCleartextExecutorAddress(relayerClient, trustedClient);
  const cleartexts = await readPlaintexts(relayerClient, trustedClient, executorAddress, orderedHandles);

  const orderedAbiEncodedClearValues = encodeTypedCleartexts(cleartextEthereumModule, orderedHandles, cleartexts);

  // The caller passes the requested extraData (= current KmsSignersContext
  // extraData). Reuse it verbatim so the returned extraData reconciles exactly
  // (publicDecrypt step 6) and the signed digest matches the verifier's.
  const extraData = parameters.payload.extraData;

  ////////////////////////////////////////////////////////////////////////////
  // Warning!!!! Do not sign over '0x00' — only '0x' is permitted (matches
  // verifyKmsPublicDecryptEip712 / createPublicDecryptionProof).
  ////////////////////////////////////////////////////////////////////////////
  const signedExtraData: BytesHex = extraData === ('0x00' as BytesHex) ? ('0x' as BytesHex) : extraData;

  // A 'PublicDecryptVerification' domain uses the gateway chainId.
  const domain = createKmsEip712Domain({
    chainId: relayerClient.chain.fhevm.gateway.id,
    verifyingContractAddressDecryption: relayerClient.chain.fhevm.gateway.contracts.decryption.address,
  });

  const digest = cleartextEthereumModule.hashTypedData({
    domain,
    types: kmsPublicDecryptEip712Types,
    primaryType: 'PublicDecryptVerification',
    message: {
      ctHandles: orderedHandles.map((h) => h.bytes32Hex),
      decryptedResult: orderedAbiEncodedClearValues,
      extraData: signedExtraData,
    },
  });

  const { signers: signersAddress } = await readKmsSignersAndThreshold(relayerClient, trustedClient);
  const signers = getKmsSignersPrivateKeyMap(relayerClient);

  const kmsPublicDecryptEip712Signatures: Bytes65Hex[] = [];

  for (const signerAddress of signersAddress) {
    const privateKey = signers.get(signerAddress);
    if (privateKey === undefined) {
      throw new Error(`Unable to find KMS signer for address ${signerAddress}`);
    }

    const signature = await cleartextEthereumModule.sign({ hash: digest, privateKey });
    kmsPublicDecryptEip712Signatures.push(signature);
  }

  return {
    orderedAbiEncodedClearValues,
    kmsPublicDecryptEip712Signatures,
    extraData,
  };
}
