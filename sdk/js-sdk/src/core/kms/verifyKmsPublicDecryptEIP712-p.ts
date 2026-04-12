import { recoverSigners } from '../utils-p/runtime/recoverSigners.js';
import {
  assertKmsSignerThreshold,
  kmsSignersContextToExtraData,
} from '../host-contracts/KmsSignersContext-p.js';
import { createKmsEIP712Domain } from './createKmsEIP712Domain.js';
import { kmsPublicDecryptEIP712Types } from './kmsPublicDecryptEIP712Types.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { KmsPublicDecryptEIP712Message } from '../types/kms.js';
import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import type { Bytes32Hex, Bytes65Hex, BytesHex } from '../types/primitives.js';
import type { EncryptedValue } from '../types/encryptedTypes.js';

type Context = {
  readonly chain: FhevmChain;
  readonly runtime: FhevmRuntime;
};

type Parameters = {
  readonly orderedEncryptedValues: readonly EncryptedValue[];
  readonly orderedAbiEncodedClearValues: BytesHex;
  readonly kmsPublicDecryptEIP712Signatures: readonly Bytes65Hex[];
  readonly kmsSignersContext: KmsSignersContext;
};

export async function verifyKmsPublicDecryptEIP712(
  context: Context,
  parameters: Parameters,
): Promise<void> {
  const {
    kmsSignersContext,
    orderedEncryptedValues,
    orderedAbiEncodedClearValues,
  } = parameters;

  // TODO:  use createKmsPublicDecryptEIP712 instead!

  const extraData: BytesHex = kmsSignersContextToExtraData(kmsSignersContext);

  ////////////////////////////////////////////////////////////////////////////
  //
  // Warning!!!! Do not use '0x00' here!! Only '0x' is permitted!
  //
  ////////////////////////////////////////////////////////////////////////////
  let signedExtraData: BytesHex = extraData;
  if (extraData === ('0x00' as BytesHex)) {
    signedExtraData = '0x' as BytesHex;
  }

  const handlesBytes32Hex: readonly Bytes32Hex[] = orderedEncryptedValues.map(
    (h) => h.bytes32Hex,
  );

  const message: KmsPublicDecryptEIP712Message = {
    ctHandles: handlesBytes32Hex,
    decryptedResult: orderedAbiEncodedClearValues,
    extraData: signedExtraData,
  };

  //////////////////////////////////////////////////////////////////////////////
  //
  // Warning!
  // A 'PublicDecryptVerification' KmsEIP712Domain uses the gateway chainId!
  //
  //////////////////////////////////////////////////////////////////////////////
  const domain = createKmsEIP712Domain({
    chainId: context.chain.fhevm.gateway.id,
    verifyingContractAddressDecryption:
      context.chain.fhevm.gateway.contracts.decryption.address,
  });

  // 1. Verify signatures
  const recoveredAddresses = await recoverSigners(context, {
    domain,
    types: kmsPublicDecryptEIP712Types,
    primaryType: 'PublicDecryptVerification',
    signatures: parameters.kmsPublicDecryptEIP712Signatures,
    message,
  });

  // 2. Verify signature threshold is reached
  assertKmsSignerThreshold(kmsSignersContext, recoveredAddresses);
}
