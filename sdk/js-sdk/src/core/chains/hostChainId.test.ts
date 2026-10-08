// `solana/test-fixtures/host-chain/host_chain_v1.json` pins the DD-052 chain-type layout that
// `zama_solana_acl::host_chain` generates; these helpers must agree with it.
import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import {
  EVM_CHAIN_TYPE,
  SOLANA_CHAIN_TYPE,
  chainTypeByte,
  isEvmHostChainId,
  isSolanaHostChainId,
  solanaHostChainId,
} from './hostChainId.js';

type HostChainVectorFile = {
  schema: string;
  constants: { evm_chain_type: string; solana_chain_type: string };
  chain_ids: { chain_id: string; chain_type_byte: number; is_evm: boolean; is_solana: boolean }[];
  solana_host_chain_ids: { chain_id: string; cluster_tag: string }[];
};

const VECTOR_FILE = new URL('../../../../../solana/test-fixtures/host-chain/host_chain_v1.json', import.meta.url);
const file = JSON.parse(readFileSync(VECTOR_FILE, 'utf8')) as HostChainVectorFile;

describe('host chain vectors (solana/test-fixtures/host-chain)', () => {
  it('reads the v1 schema', () => {
    expect(file.schema).toBe('zama-host-chain-vectors/v1');
  });

  it('agrees on the chain-type bytes', () => {
    expect(EVM_CHAIN_TYPE).toBe(BigInt(file.constants.evm_chain_type));
    expect(SOLANA_CHAIN_TYPE).toBe(BigInt(file.constants.solana_chain_type));
  });

  it.each(file.chain_ids)('classifies chain id $chain_id', ({ chain_id, chain_type_byte, is_evm, is_solana }) => {
    const chainId = BigInt(chain_id);
    expect(chainTypeByte(chainId)).toBe(BigInt(chain_type_byte));
    expect(isEvmHostChainId(chainId)).toBe(is_evm);
    expect(isSolanaHostChainId(chainId)).toBe(is_solana);
  });

  it.each(file.solana_host_chain_ids)(
    'builds the Solana chain id for cluster tag $cluster_tag',
    ({ chain_id, cluster_tag }) => {
      expect(solanaHostChainId(BigInt(cluster_tag))).toBe(BigInt(chain_id));
    },
  );
});
