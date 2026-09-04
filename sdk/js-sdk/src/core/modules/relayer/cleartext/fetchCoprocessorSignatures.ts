import type { Bytes65Hex, BytesHex, ChecksummedAddress, Uint32BigInt } from '../../../types/primitives.js';
import type { CleartextEthereumModule } from '../../ethereum/types-ct.js';
import type {
  FetchCoprocessorSignaturesParameters,
  FetchCoprocessorSignaturesReturnType,
  RelayerClientWithRuntime,
} from '../types.js';
import type { CoprocessorSignersContext } from '../../../types/coprocessorSignersContext.js';
import type { FhevmClientFrozenContext } from '../../../types/fhevmClientFrozenContext-p.js';
import { asUint32BigInt, asUint64BigInt, randomUniqueUints } from '../../../base/uint.js';
import { getTrustedClient } from '../../../runtime/CoreFhevm-p.js';
import { getCoprocessorSignersPrivateKeyMap } from './signers.js';
import { isForgeFhevmV1 } from './forgeFhevmV1.js';
import { createCoprocessorEip712Domain } from '../../../coprocessor/createCoprocessorEip712Domain.js';
import { coprocessorEip712PrimaryType, coprocessorEip712Types } from '../../../coprocessor/coprocessorEip712Types.js';
import { readCoprocessorSignersContext } from '../../../host-contracts/readCoprocessorSignersContext-p.js';

////////////////////////////////////////////////////////////////////////////////
// runInputProofOnChain
////////////////////////////////////////////////////////////////////////////////

const inputProofAbi = [
  {
    type: 'function',
    name: 'inputProof',
    inputs: [
      {
        name: 'ctHandles',
        type: 'bytes32[]',
        internalType: 'bytes32[]',
      },
      { name: 'userAddress', type: 'address', internalType: 'address' },
      {
        name: 'contractAddress',
        type: 'address',
        internalType: 'address',
      },
      { name: 'extraData', type: 'bytes', internalType: 'bytes' },
    ],
    outputs: [
      { name: 'digest', type: 'bytes32', internalType: 'bytes32' },
      { name: 'signers', type: 'address[]', internalType: 'address[]' },
      { name: 'threshold', type: 'uint256', internalType: 'uint256' },
    ],
    stateMutability: 'view',
  },
] as const;

async function runInputProofOnChain(
  relayerClient: RelayerClientWithRuntime,
  payload: FetchCoprocessorSignaturesParameters['payload'],
): Promise<{
  readonly digest: BytesHex;
  readonly signersAddress: readonly ChecksummedAddress[];
  readonly threshold: Uint32BigInt;
}> {
  const trustedClient = getTrustedClient(relayerClient);
  const res = (await relayerClient.runtime.ethereum.readContract(trustedClient, {
    abi: inputProofAbi,
    address: relayerClient.chain.fhevm.contracts.inputVerifier.address as ChecksummedAddress,
    args: [
      payload.zkProof.getInputHandles().map((h) => h.bytes32Hex),
      payload.zkProof.userAddress,
      payload.zkProof.contractAddress,
      payload.zkProof.getExtraData(),
    ],
    functionName: inputProofAbi[0].name,
  })) as unknown[];

  const digest = res[0] as BytesHex;
  const signersAddress = res[1] as ChecksummedAddress[];
  const threshold = asUint32BigInt(res[2]);

  return {
    digest,
    signersAddress,
    threshold,
  };
}

////////////////////////////////////////////////////////////////////////////////
// runInputProofOffChain
////////////////////////////////////////////////////////////////////////////////

async function runInputProofOffChain(
  relayerClient: RelayerClientWithRuntime,
  payload: FetchCoprocessorSignaturesParameters['payload'],
  fhevmContext: FhevmClientFrozenContext,
): Promise<{
  readonly digest: BytesHex;
  readonly signersAddress: readonly ChecksummedAddress[];
  readonly threshold: Uint32BigInt;
}> {
  const cleartextEthereumModule = relayerClient.runtime.ethereum as CleartextEthereumModule;
  const { zkProof } = payload;

  // Read the coprocessor signer set + threshold from the InputVerifier's on-chain
  // storage. In the forge-fhevm-v1 setup only the `inputProof` view is missing;
  // the signers/threshold and the EIP-712 material are still readable, so we
  // recompute the digest off-chain below (mirroring CleartextInputVerifier.inputProof).
  const coprocessorSignersContext: CoprocessorSignersContext = await readCoprocessorSignersContext(relayerClient, {
    address: relayerClient.chain.fhevm.contracts.inputVerifier.address as ChecksummedAddress,
    fhevmContext,
  });

  // The 'InputVerification' domain uses the gateway chainId + gateway
  // inputVerification address (same as verifyHandlesCoprocessorSignatures).
  const domain = createCoprocessorEip712Domain({
    gatewayChainId: relayerClient.chain.fhevm.gateway.id,
    verifyingContractAddressInputVerification: relayerClient.chain.fhevm.gateway.contracts.inputVerification.address,
  });

  // Mirror CleartextInputVerifier.inputProof: hash the CiphertextVerification
  // struct with contractChainId = block.chainid (the host chain), and extraData
  // taken verbatim from the ZK proof (as the on-chain view would receive it).
  const digest: BytesHex = cleartextEthereumModule.hashTypedData({
    domain,
    types: coprocessorEip712Types,
    primaryType: coprocessorEip712PrimaryType,
    message: {
      ctHandles: zkProof.getInputHandles().map((h) => h.bytes32Hex),
      userAddress: zkProof.userAddress,
      contractAddress: zkProof.contractAddress,
      contractChainId: asUint64BigInt(BigInt(relayerClient.chain.id)),
      extraData: zkProof.getExtraData(),
    },
  });

  return {
    digest,
    signersAddress: coprocessorSignersContext.signers,
    threshold: asUint32BigInt(BigInt(coprocessorSignersContext.threshold)),
  };
}

export async function fetchCoprocessorSignatures(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchCoprocessorSignaturesParameters,
): Promise<FetchCoprocessorSignaturesReturnType> {
  const cleartextEthereumModule = relayerClient.runtime.ethereum as CleartextEthereumModule;
  const { payload, fhevmContext } = parameters;

  const offChain = await isForgeFhevmV1(relayerClient, fhevmContext);

  let res;
  if (offChain) {
    res = await runInputProofOffChain(relayerClient, payload, fhevmContext);
  } else {
    res = await runInputProofOnChain(relayerClient, payload);
  }

  const digest = res.digest;
  const signersAddress = res.signersAddress;
  const threshold = res.threshold;

  const signers = getCoprocessorSignersPrivateKeyMap(relayerClient);

  const randomSignersAddress = randomUniqueUints(signersAddress.length, Number(threshold)).map(
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    (i) => signersAddress[i]!,
  );

  const coprocessorEip712Signatures: Bytes65Hex[] = [];

  for (const signerAddress of randomSignersAddress) {
    const privateKey = signers.get(signerAddress);
    if (privateKey === undefined) {
      throw new Error(`Unable to find KMS signer for address ${signerAddress}`);
    }

    const signature = await cleartextEthereumModule.sign({ hash: digest, privateKey });
    coprocessorEip712Signatures.push(signature);
  }

  return Promise.resolve({
    extraData: payload.zkProof.getExtraData(),
    handles: payload.zkProof.getInputHandles(),
    coprocessorEip712Signatures,
  });
}
