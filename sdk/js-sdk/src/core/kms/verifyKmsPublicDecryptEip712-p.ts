import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { KmsPublicDecryptEip712Message } from '../types/kms.js';
import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import type { Bytes32Hex, Bytes65Hex, BytesHex } from '../types/primitives.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import type { KmsExtraData } from '../types/kms-p.js';
import { recoverSigners } from '../utils-p/runtime/recoverSigners.js';
import { assertKmsSignerThreshold, kmsSignersContextToExtraData } from '../host-contracts/KmsSignersContext-p.js';
import { createKmsEip712Domain } from './createKmsEip712Domain.js';
import { kmsPublicDecryptEip712Types } from './kmsPublicDecryptEip712Types.js';
import { EXTRA_DATA_V0 } from './kmsExtraData-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly chain: FhevmChain;
  readonly runtime: FhevmRuntime;
};

type Parameters = {
  readonly orderedHandles: readonly Handle[];
  readonly orderedAbiEncodedClearValues: BytesHex;
  readonly kmsPublicDecryptEip712Signatures: readonly Bytes65Hex[];
  readonly kmsSignersContext: KmsSignersContext;
};

////////////////////////////////////////////////////////////////////////////////

export async function verifyKmsPublicDecryptEip712(context: Context, parameters: Parameters): Promise<void> {
  const { kmsSignersContext, orderedHandles: orderedEncryptedValues, orderedAbiEncodedClearValues } = parameters;

  // TODO:  use createKmsPublicDecryptEip712 instead!

  const extraData: KmsExtraData = kmsSignersContextToExtraData(kmsSignersContext);

  ////////////////////////////////////////////////////////////////////////////
  //
  // Warning!!!! Do not use '0x00' here!! Only '0x' is permitted!
  //
  ////////////////////////////////////////////////////////////////////////////

  let signedExtraDataBytesHex: BytesHex = extraData.bytesHex;
  if (extraData.version === EXTRA_DATA_V0) {
    signedExtraDataBytesHex = '0x' as BytesHex;
  }

  const handlesBytes32Hex: readonly Bytes32Hex[] = orderedEncryptedValues.map((h) => h.bytes32Hex);

  const message: KmsPublicDecryptEip712Message = {
    ctHandles: handlesBytes32Hex,
    decryptedResult: orderedAbiEncodedClearValues,
    extraData: signedExtraDataBytesHex,
  };

  //////////////////////////////////////////////////////////////////////////////
  //
  // Warning!
  // A 'PublicDecryptVerification' KmsEip712Domain uses the gateway chainId!
  //
  //////////////////////////////////////////////////////////////////////////////

  const domain = createKmsEip712Domain({
    chainId: context.chain.fhevm.gateway.id,
    verifyingContractAddressDecryption: context.chain.fhevm.gateway.contracts.decryption.address,
  });

  // 1. Verify signatures
  const recoveredAddresses = await recoverSigners(context, {
    domain,
    types: kmsPublicDecryptEip712Types,
    primaryType: 'PublicDecryptVerification',
    signatures: parameters.kmsPublicDecryptEip712Signatures,
    message,
  });

  // 2. Verify signature threshold is reached
  assertKmsSignerThreshold(kmsSignersContext, recoveredAddresses);
}
