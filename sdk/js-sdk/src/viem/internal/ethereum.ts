import type { PublicClient, WalletClient } from 'viem';
import type {
  DecodeParameters,
  DecodeReturnType,
  EncodePackedParameters,
  EncodePackedReturnType,
  EncodeParameters,
  EncodeReturnType,
  EthCallParameters,
  EthCallResult,
  EthereumModuleFactory,
  GetChainIdReturnType,
  HashTypedDataParameters,
  HashTypedDataReturnType,
  TrustedClient,
  ReadContractParameters,
  RecoverTypedDataAddressParameters,
  RecoverTypedDataAddressReturnType,
  SignTypedDataParameters,
  SignTypedDataReturnType,
  NativeSigner,
} from '../../core/modules/ethereum/types.js';
import type { Bytes32Hex, BytesHex } from '../../core/types/primitives.js';
import { BaseError, ExecutionRevertedError, hashTypedData as viemHashTypedData } from 'viem';
import { recoverTypedDataAddress as viemRecoverTypedDataAddress } from 'viem';
import { encodePacked as viemEncodePacked, encodeAbiParameters, decodeAbiParameters } from 'viem';
import { asChecksummedAddress } from '../../core/base/address.js';
import { trustedClientToViemPublicClient } from './viem-p.js';

////////////////////////////////////////////////////////////////////////////////
// recoverTypedDataAddress
////////////////////////////////////////////////////////////////////////////////

export async function recoverTypedDataAddress(
  parameters: RecoverTypedDataAddressParameters,
): Promise<RecoverTypedDataAddressReturnType> {
  const recoveredAddress = await viemRecoverTypedDataAddress(parameters);
  return asChecksummedAddress(recoveredAddress);
}

////////////////////////////////////////////////////////////////////////////////
// encodePacked
////////////////////////////////////////////////////////////////////////////////

export function encodePacked(parameters: EncodePackedParameters): EncodePackedReturnType {
  return viemEncodePacked(parameters.types, parameters.values) as BytesHex;
}

////////////////////////////////////////////////////////////////////////////////
// encode
////////////////////////////////////////////////////////////////////////////////

export function encode(parameters: EncodeParameters): EncodeReturnType {
  const abiParameters = parameters.types.map((type) => ({ type }));
  return encodeAbiParameters(abiParameters, parameters.values) as BytesHex;
}

////////////////////////////////////////////////////////////////////////////////
// decode
////////////////////////////////////////////////////////////////////////////////

export function decode(parameters: DecodeParameters): DecodeReturnType {
  const abiParameters = parameters.types.map((type) => ({ type }));
  return [...decodeAbiParameters(abiParameters, parameters.encodedData as `0x${string}`)];
}

////////////////////////////////////////////////////////////////////////////////
// hashTypedData
////////////////////////////////////////////////////////////////////////////////

export function hashTypedData(parameters: HashTypedDataParameters): HashTypedDataReturnType {
  return viemHashTypedData({
    domain: parameters.domain,
    types: parameters.types,
    primaryType: parameters.primaryType,
    message: parameters.message,
  }) as Bytes32Hex;
}

////////////////////////////////////////////////////////////////////////////////
// readContract
////////////////////////////////////////////////////////////////////////////////

export async function readContract(
  hostPublicClient: TrustedClient<PublicClient>,
  parameters: ReadContractParameters,
): Promise<unknown> {
  const publicClient = trustedClientToViemPublicClient(hostPublicClient);
  return publicClient.readContract(parameters);
}

////////////////////////////////////////////////////////////////////////////////
// call (raw eth_call / STATICCALL)
////////////////////////////////////////////////////////////////////////////////

export async function call(
  hostPublicClient: TrustedClient<PublicClient>,
  parameters: EthCallParameters,
): Promise<EthCallResult> {
  const publicClient = trustedClientToViemPublicClient(hostPublicClient);
  try {
    const { data } = await publicClient.call({
      to: parameters.to,
      data: parameters.data,
      ...(parameters.gas !== undefined ? { gas: parameters.gas } : {}),
    });
    return { success: true, data: (data ?? '0x') as BytesHex };
  } catch (err) {
    // A clean EVM revert is a definitive on-chain rejection; anything else
    // (network, timeout, rate limit, …) is transport-level and rethrown so the
    // precautionary caller can degrade gracefully.
    if (err instanceof BaseError && err.walk((e) => e instanceof ExecutionRevertedError) !== null) {
      return { success: false, reason: err.shortMessage };
    }
    throw err;
  }
}

////////////////////////////////////////////////////////////////////////////////
// getChainId
////////////////////////////////////////////////////////////////////////////////

export async function getChainId(hostPublicClient: TrustedClient<PublicClient>): Promise<GetChainIdReturnType> {
  const publicClient = trustedClientToViemPublicClient(hostPublicClient);
  const chainId = await publicClient.getChainId();
  return BigInt(chainId);
}

////////////////////////////////////////////////////////////////////////////////
// signTypedData
////////////////////////////////////////////////////////////////////////////////

export async function signTypedData(
  signer: NativeSigner,
  parameters: SignTypedDataParameters,
): Promise<SignTypedDataReturnType> {
  const { account, primaryType, types, domain, message } = parameters;

  const walletClient = signer as WalletClient;
  if (typeof walletClient.signTypedData !== 'function') {
    throw new Error('signer does not support signTypedData');
  }

  // Skipped: viem delegates account selection to the wallet/RPC node.
  // Adding a getAddresses() guard here would add an extra RPC call
  // on every sign for little benefit.
  // Keep code if needed
  /*

      const walletAddresses = await walletClient.getAddresses();
      if (walletAddresses.length === 0) {
        throw new Error("No account found in wallet client");
      }
      const accountLower = account.toLowerCase();
      if (!walletAddresses.some((addr) => addr.toLowerCase() === accountLower)) {
        throw new Error(
          `Signer address mismatch: expected ${account} but wallet contains [${walletAddresses.join(", ")}]`,
        );
      }
        
  */

  const signature = await walletClient.signTypedData({
    account,
    domain,
    types,
    primaryType,
    message,
  });

  return signature as SignTypedDataReturnType;
}

////////////////////////////////////////////////////////////////////////////////
// ethereumModule
////////////////////////////////////////////////////////////////////////////////

export const ethereumModule: EthereumModuleFactory = () => {
  return Object.freeze({
    ethereum: Object.freeze({
      decode,
      encode,
      encodePacked,
      recoverTypedDataAddress,
      hashTypedData,
      signTypedData,
      getChainId,
      readContract,
      call,
    }),
  });
};
