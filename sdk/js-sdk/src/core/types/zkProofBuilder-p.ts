import type { EncryptionBits } from './fheType.js';
import type {
  AddressValueLike,
  BoolValueLike,
  TypedValue,
  Uint128ValueLike,
  Uint16ValueLike,
  Uint256ValueLike,
  Uint32ValueLike,
  Uint64ValueLike,
  Uint8ValueLike,
} from './primitives.js';
import type { ZkProof, SolanaZkProof } from './zkProof-p.js';
import type { FhevmRuntime, WithEncrypt } from './coreFhevmRuntime.js';
import type { FhevmBase, OptionalNativeClient } from './coreFhevmClient.js';
import type { FhevmChain } from './fhevmChain.js';
import type { FhevmClientFrozenContext } from './fhevmClientFrozenContext-p.js';
import type { FhevmSolanaChain } from './fhevmSolanaChain.js';
import type { Bytes32Hex } from './primitives.js';

export type SolanaProofContext = {
  /** The Solana client's core: it holds the caller-provided FHE encryption key until the first fetch. */
  readonly fhevm: FhevmBase<undefined, FhevmRuntime, undefined>;
  readonly chain: FhevmSolanaChain;
  readonly aclProgramAddress: Bytes32Hex;
  readonly runtime: WithEncrypt;
};

export interface ZkProofBuilder {
  addBool(value: boolean | number | bigint | BoolValueLike): this;
  addUint8(value: number | bigint | Uint8ValueLike): this;
  addUint16(value: number | bigint | Uint16ValueLike): this;
  addUint32(value: number | bigint | Uint32ValueLike): this;
  addUint64(value: number | bigint | Uint64ValueLike): this;
  addUint128(value: number | bigint | Uint128ValueLike): this;
  addUint256(value: number | bigint | Uint256ValueLike): this;
  addAddress(value: string | AddressValueLike): this;
  addTypedValue(typedValue: TypedValue): this;
  getBits(): EncryptionBits[];
  build(
    context: FhevmBase<FhevmChain, WithEncrypt, OptionalNativeClient>,
    parameters: {
      readonly contractAddress: string;
      readonly userAddress: string;
      readonly extraData: string;
      readonly fhevmContext: FhevmClientFrozenContext;
    },
  ): Promise<ZkProof>;
  buildSolana(
    context: SolanaProofContext,
    parameters: {
      readonly contractAddress: string;
      readonly userAddress: string;
    },
  ): Promise<SolanaZkProof>;
}
