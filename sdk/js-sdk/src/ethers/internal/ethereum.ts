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
import type { ethers as EthersT } from "ethers";
import type { TypedDataField } from "ethers";
import { asChecksummedAddress } from "../../core/base/address.js";
import { AbiCoder, solidityPacked, verifyTypedData } from "ethers";
import type { BytesHex } from "../../core/types/primitives.js";
import { getEthersContract, getNetwork } from "./utils.js";

////////////////////////////////////////////////////////////////////////////////
// encodePacked
////////////////////////////////////////////////////////////////////////////////

// eslint-disable-next-line @typescript-eslint/require-await
export async function recoverTypedDataAddress(
  parameters: RecoverTypedDataAddressParameters,
): Promise<RecoverTypedDataAddressReturnType> {
  const { primaryType, types, domain, message, signature } = parameters;

  // If primaryType is specified, filter types to only include the primary type
  // This ensures ethers uses the correct primary type for signing
  let typesToSign: Record<string, TypedDataField[]>;

  if ((primaryType as unknown) !== undefined) {
    const primaryTypeFields = types[primaryType];
    if (primaryTypeFields === undefined) {
      throw new Error(`Primary type "${primaryType}" not found in types`);
    }
    typesToSign = { [primaryType]: primaryTypeFields };
  } else {
    typesToSign = types;
  }

  const recoveredAddress = verifyTypedData(
    domain,
    typesToSign,
    message,
    signature,
  );

  return asChecksummedAddress(recoveredAddress);
}

////////////////////////////////////////////////////////////////////////////////
// encodePacked
////////////////////////////////////////////////////////////////////////////////

export function encodePacked(
  parameters: EncodePackedParameters,
): EncodePackedReturnType {
  return solidityPacked(parameters.types, parameters.values) as BytesHex;
}

////////////////////////////////////////////////////////////////////////////////
// encode
////////////////////////////////////////////////////////////////////////////////

export function encode(parameters: EncodeParameters): EncodeReturnType {
  const abiCoder = AbiCoder.defaultAbiCoder();
  return abiCoder.encode(parameters.types, parameters.values) as BytesHex;
}

////////////////////////////////////////////////////////////////////////////////
// decode
////////////////////////////////////////////////////////////////////////////////

export function decode(parameters: DecodeParameters): DecodeReturnType {
  const abiCoder = AbiCoder.defaultAbiCoder();
  return abiCoder.decode(parameters.types, parameters.encodedData);
}

////////////////////////////////////////////////////////////////////////////////
// readContract
////////////////////////////////////////////////////////////////////////////////

export async function readContract(
  hostPublicClient: TrustedClient<EthersT.ContractRunner>,
  parameters: ReadContractParameters,
): Promise<unknown> {
  const contract = getEthersContract<EthersT.Contract>(
    hostPublicClient,
    parameters.address,
    parameters.abi,
  );
  const result = (await contract
    .getFunction(parameters.functionName)
    .staticCall(...parameters.args)) as unknown;
  return result;
}

////////////////////////////////////////////////////////////////////////////////
// getChainId
////////////////////////////////////////////////////////////////////////////////

export async function getChainId<T extends EthersT.ContractRunner>(
  hostPublicClient: TrustedClient<T>,
): Promise<GetChainIdReturnType> {
  const n = await getNetwork(hostPublicClient);
  // eslint-disable-next-line @typescript-eslint/no-unnecessary-type-conversion
  return BigInt(n.chainId);
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
