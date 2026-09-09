import { address } from '@solana/kit';

import { SOLANA_HOST_CHAIN_ID_I64 } from './constants';

/** Preview key material is shared with the canonical EVM host, as in local bring-up. */
export const registerSolanaCoprocessorSql = (programId: string, sourceChainId: string): string => {
  const program = address(programId);
  const source = BigInt(sourceChainId).toString();
  return `BEGIN;
    LOCK TABLE host_chains IN SHARE ROW EXCLUSIVE MODE;
    DO $$ BEGIN
      IF EXISTS (SELECT 1 FROM host_chains WHERE chain_id = ${SOLANA_HOST_CHAIN_ID_I64}
        AND acl_contract_address <> '${program}') THEN
        RAISE EXCEPTION 'Solana chain is already registered with a different program';
      END IF;
      IF NOT EXISTS (SELECT 1 FROM keys WHERE chain_id = ${source}) THEN
        RAISE EXCEPTION 'Canonical host key material is not ready';
      END IF;
    END $$;
    INSERT INTO host_chains (chain_id,name,acl_contract_address)
      VALUES (${SOLANA_HOST_CHAIN_ID_I64},'solana','${program}') ON CONFLICT DO NOTHING;
    INSERT INTO keys (key_id_gw,key_id,pks_key,sks_key,cks_key,sns_pk,compressed_xof_keyset,chain_id,block_hash)
      SELECT key_id_gw,key_id,pks_key,sks_key,cks_key,sns_pk,compressed_xof_keyset,${SOLANA_HOST_CHAIN_ID_I64},block_hash
      FROM keys WHERE chain_id=${source} ORDER BY sequence_number ASC ON CONFLICT DO NOTHING;
    COMMIT;`;
};
