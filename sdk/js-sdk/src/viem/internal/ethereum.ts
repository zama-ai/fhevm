import type {
  DecodeParameters,
  DecodeReturnType,
  EncodePackedParameters,
  EncodePackedReturnType,
  EncodeParameters,
  EncodeReturnType,
  EthereumModuleFactory,
  GetChainIdReturnType,
  TrustedClient,
  ReadContractParameters,
  RecoverTypedDataAddressParameters,
  RecoverTypedDataAddressReturnType,
} from "../../core/modules/ethereum/types.js";
import type { PublicClient, Chain, Transport } from "viem";
import { asChecksummedAddress } from "../../core/base/address.js";
import {
  encodeAbiParameters,
  decodeAbiParameters,
  encodePacked as viemEncodePacked,
  recoverTypedDataAddress as viemRecoverTypedDataAddress,
  parseAbiParameters,
} from "viem";
import type { BytesHex } from "../../core/types/primitives.js";
import { trustedClientToViemPublicClient } from "./viem-p.js";
import { getChainId as getChainId_ } from "./utils.js";

////////////////////////////////////////////////////////////////////////////////
// recoverTypedDataAddress
////////////////////////////////////////////////////////////////////////////////

export async function recoverTypedDataAddress(
  parameters: RecoverTypedDataAddressParameters,
): Promise<RecoverTypedDataAddressReturnType> {
  const { primaryType, types, domain, message, signature } = parameters;

  // If primaryType is specified, filter types to only include the primary type
  let typesToSign: Record<string, Array<{ name: string; type: string }>>;

  if ((primaryType as unknown) !== undefined) {
    const primaryTypeFields = types[primaryType];
    if (primaryTypeFields === undefined) {
      throw new Error(`Primary type "${primaryType}" not found in types`);
    }
    typesToSign = { [primaryType]: primaryTypeFields };
  } else {
    typesToSign = types;
  }

  const recoveredAddress = await viemRecoverTypedDataAddress({
    domain: {
      chainId: Number(domain.chainId),
      name: domain.name,
      verifyingContract: domain.verifyingContract as `0x${string}`,
      version: domain.version,
    },
    types: typesToSign,
    primaryType: primaryType,
    message: message as Record<string, unknown>,
    signature: signature as `0x${string}`,
  });

  return asChecksummedAddress(recoveredAddress);
}

////////////////////////////////////////////////////////////////////////////////
// encodePacked
////////////////////////////////////////////////////////////////////////////////

export function encodePacked(
  parameters: EncodePackedParameters,
): EncodePackedReturnType {
  const types = parameters.types as string[];
  const values = parameters.values as unknown[];
  return viemEncodePacked(types, values) as BytesHex;
}

////////////////////////////////////////////////////////////////////////////////
// encode
////////////////////////////////////////////////////////////////////////////////

export function encode(parameters: EncodeParameters): EncodeReturnType {
  const abiParams = parseAbiParameters(parameters.types.join(", "));
  return encodeAbiParameters(
    abiParams,
    parameters.values as readonly unknown[],
  ) as BytesHex;
}

////////////////////////////////////////////////////////////////////////////////
// decode
////////////////////////////////////////////////////////////////////////////////

export function decode(parameters: DecodeParameters): DecodeReturnType {
  const abiParams = parseAbiParameters(parameters.types.join(", "));
  const result = decodeAbiParameters(
    abiParams,
    parameters.encodedData as `0x${string}`,
  );
  return [...result];
}

////////////////////////////////////////////////////////////////////////////////
// readContract
////////////////////////////////////////////////////////////////////////////////

export async function readContract(
  hostPublicClient: TrustedClient<PublicClient<Transport, Chain>>,
  parameters: ReadContractParameters,
): Promise<unknown> {
  const client = trustedClientToViemPublicClient(hostPublicClient);
  const result = await client.readContract({
    address: parameters.address as `0x${string}`,
    abi: parameters.abi as readonly unknown[],
    functionName: parameters.functionName,
    args: parameters.args as readonly unknown[],
  });
  return result;
}

////////////////////////////////////////////////////////////////////////////////
// getChainId
////////////////////////////////////////////////////////////////////////////////

export async function getChainId<
  T extends PublicClient<Transport, Chain>,
>(hostPublicClient: TrustedClient<T>): Promise<GetChainIdReturnType> {
  return await getChainId_(
    hostPublicClient as TrustedClient<PublicClient<Transport, Chain>>,
  );
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
      getChainId,
      readContract,
    }),
  });
};
