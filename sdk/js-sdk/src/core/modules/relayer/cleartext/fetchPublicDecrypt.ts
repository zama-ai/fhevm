import type { Bytes32Hex, Bytes65Hex, BytesHex, ChecksummedAddress } from '../../../types/primitives.js';
import type { CleartextEthereumModule } from '../../ethereum/types-ct.js';
import type { FetchPublicDecryptParameters, FetchPublicDecryptReturnType, RelayerClientWithRuntime } from '../types.js';
import type { KmsSignersContext } from '../../../types/kmsSignersContext.js';
import type { FhevmClientFrozenContext } from '../../../types/fhevmClientFrozenContext-p.js';
import { getTrustedClient } from '../../../runtime/CoreFhevm-p.js';
import { getKmsSignersPrivateKeyMap } from './signers.js';
import { encodeTypedCleartexts, isForgeFhevmV1, readPlaintexts } from './forgeFhevmV1.js';
import { createKmsEip712Domain } from '../../../kms/createKmsEip712Domain.js';
import { readCurrentKmsSignersContext } from '../../../host-contracts/readKmsSignersContext-p.js';
import { kmsPublicDecryptEip712Types } from '../../../kms/kmsPublicDecryptEip712Types.js';

////////////////////////////////////////////////////////////////////////////////

const publicDecryptAbi = [
  {
    type: 'function',
    name: 'publicDecrypt',
    inputs: [{ name: 'handles', type: 'bytes32[]', internalType: 'bytes32[]' }],
    outputs: [
      {
        name: 'abiEncodedCleartexts',
        type: 'bytes',
        internalType: 'bytes',
      },
      { name: 'digest', type: 'bytes32', internalType: 'bytes32' },
      { name: 'signers', type: 'address[]', internalType: 'address[]' },
      { name: 'threshold', type: 'uint256', internalType: 'uint256' },
      { name: 'extraData', type: 'bytes', internalType: 'bytes' },
    ],
    stateMutability: 'view',
  },
] as const;

////////////////////////////////////////////////////////////////////////////////

async function runPublicDecryptOffChain(
  relayerClient: RelayerClientWithRuntime,
  payload: FetchPublicDecryptParameters['payload'],
  fhevmContext: FhevmClientFrozenContext,
): Promise<{
  readonly orderedAbiEncodedClearValues: BytesHex;
  readonly digest: Bytes32Hex;
  readonly signersAddress: readonly ChecksummedAddress[];
  readonly extraData: BytesHex;
}> {
  const cleartextEthereumModule = relayerClient.runtime.ethereum as CleartextEthereumModule;
  const { orderedHandles } = payload;

  // Option B: read cleartexts from CleartextFHEVMExecutor.plaintexts and rebuild
  // the public-decrypt result off-chain instead of reading the CleartextKMSVerifier
  // view. ACL `isAllowedForDecryption` gating is enforced by the caller
  // (publicDecrypt step 4, checkAllowedForDecryption).
  const trustedClient = getTrustedClient(relayerClient);
  const cleartexts = await readPlaintexts(relayerClient, trustedClient, orderedHandles);

  const orderedAbiEncodedClearValues = encodeTypedCleartexts(cleartextEthereumModule, orderedHandles, cleartexts);

  // The caller passes the requested extraData (= current KmsSignersContext
  // extraData). Reuse it verbatim so the returned extraData reconciles exactly
  // (publicDecrypt step 6) and the signed digest matches the verifier's.
  const extraData = payload.extraData;

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

  const currentKmsSignersContext: KmsSignersContext = await readCurrentKmsSignersContext(relayerClient, {
    kmsVerifierAddress: relayerClient.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
    protocolConfigAddress: relayerClient.chain.fhevm.contracts.protocolConfig?.address as
      | ChecksummedAddress
      | undefined,
    fhevmContext,
  });

  const digest: Bytes32Hex = cleartextEthereumModule.hashTypedData({
    domain,
    types: kmsPublicDecryptEip712Types,
    primaryType: 'PublicDecryptVerification',
    message: {
      ctHandles: orderedHandles.map((h) => h.bytes32Hex),
      decryptedResult: orderedAbiEncodedClearValues,
      extraData: signedExtraData,
    },
  });

  return {
    orderedAbiEncodedClearValues,
    digest,
    signersAddress: currentKmsSignersContext.signers,
    extraData,
  };
}

////////////////////////////////////////////////////////////////////////////////
// runPublicDecryptOnChain
////////////////////////////////////////////////////////////////////////////////

async function runPublicDecryptOnChain(
  relayerClient: RelayerClientWithRuntime,
  payload: FetchPublicDecryptParameters['payload'],
): Promise<{
  readonly orderedAbiEncodedClearValues: BytesHex;
  readonly digest: Bytes32Hex;
  readonly signersAddress: readonly ChecksummedAddress[];
  readonly extraData: BytesHex;
}> {
  const trustedClient = getTrustedClient(relayerClient);
  const res = (await relayerClient.runtime.ethereum.readContract(trustedClient, {
    abi: publicDecryptAbi,
    address: relayerClient.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
    args: [payload.orderedHandles.map((h) => h.bytes32Hex)],
    functionName: publicDecryptAbi[0].name,
  })) as unknown[];

  const orderedAbiEncodedClearValues = res[0] as BytesHex;
  const digest = res[1] as Bytes32Hex;
  const signersAddress = res[2] as readonly ChecksummedAddress[];
  const extraData = res[4] as BytesHex;

  return {
    orderedAbiEncodedClearValues,
    digest,
    signersAddress,
    extraData,
  };
}

////////////////////////////////////////////////////////////////////////////////

export async function fetchPublicDecrypt(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchPublicDecryptParameters,
): Promise<FetchPublicDecryptReturnType> {
  const cleartextEthereumModule = relayerClient.runtime.ethereum as CleartextEthereumModule;

  const offChain = await isForgeFhevmV1(relayerClient, parameters.fhevmContext);

  let res;
  if (offChain) {
    res = await runPublicDecryptOffChain(relayerClient, parameters.payload, parameters.fhevmContext);
  } else {
    res = await runPublicDecryptOnChain(relayerClient, parameters.payload);
  }

  const digest = res.digest;
  const signersAddress = res.signersAddress;
  const signers = getKmsSignersPrivateKeyMap(relayerClient);

  const kmsPublicDecryptEip712Signatures: Bytes65Hex[] = [];

  for (let i = 0; i < signersAddress.length; ++i) {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const privateKey = signers.get(signersAddress[i]!);
    if (privateKey === undefined) {
      throw new Error('Unable to find kms signer');
    }

    const signature = await cleartextEthereumModule.sign({ hash: digest, privateKey });
    kmsPublicDecryptEip712Signatures.push(signature);
  }

  return {
    orderedAbiEncodedClearValues: res.orderedAbiEncodedClearValues,
    kmsPublicDecryptEip712Signatures,
    extraData: res.extraData,
  };
}
