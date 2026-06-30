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
import type { WithEncrypt } from './coreFhevmRuntime.js';
import type { FhevmChain } from './fhevmChain.js';
import type { TfheVersion } from '../../wasm/tfhe/TfheApi.js';

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
    context: { readonly chain: FhevmChain; readonly runtime: WithEncrypt; readonly tfheVersion: TfheVersion },
    parameters: {
      readonly contractAddress: string;
      readonly userAddress: string;
      readonly extraData: string;
    },
  ): Promise<ZkProof>;
  buildSolana(
    context: { readonly chain: FhevmChain; readonly runtime: WithEncrypt; readonly tfheVersion: TfheVersion },
    parameters: {
      readonly contractAddress: string;
      readonly userAddress: string;
    },
  ): Promise<SolanaZkProof>;
}
