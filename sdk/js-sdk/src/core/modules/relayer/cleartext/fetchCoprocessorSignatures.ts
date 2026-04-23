import { asUint32BigInt, randomUniqueUints } from '../../../base/uint.js';
import { getTrustedClient } from '../../../runtime/CoreFhevm-p.js';
import type { Bytes65Hex, BytesHex, ChecksummedAddress } from '../../../types/primitives.js';
import type { CleartextEthereumModule } from '../../ethereum/types-ct.js';
import type {
  FetchCoprocessorSignaturesParameters,
  FetchCoprocessorSignaturesReturnType,
  RelayerClientWithRuntime,
} from '../types.js';
import { getCoprocessorSignersPrivateKeyMap } from './signers.js';

/*
export type FetchCoprocessorSignaturesReturnType = {
  readonly handles: readonly InputHandle[];
  readonly coprocessorEip712Signatures: readonly Bytes65Hex[];
  readonly extraData: BytesHex;
};

*/
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

export async function fetchCoprocessorSignatures(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchCoprocessorSignaturesParameters,
): Promise<FetchCoprocessorSignaturesReturnType> {
  const cleartextEthereumModule = relayerClient.runtime.ethereum as CleartextEthereumModule;
  const { payload } = parameters;

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
