import { address } from '@solana/kit';

import { SOLANA_HOST_CHAIN_ID } from './constants';

/** Preview key material is shared with the canonical EVM host, as in local bring-up. */
export const registerSolanaCoprocessorSql = (programId: string, sourceChainId: string): string => {
  const program = address(programId);
  const source = BigInt(sourceChainId).toString();
  return `BEGIN;
    LOCK TABLE host_chains IN SHARE ROW EXCLUSIVE MODE;
    DO $$ BEGIN
      IF EXISTS (SELECT 1 FROM host_chains WHERE chain_id = ${SOLANA_HOST_CHAIN_ID}
        AND acl_contract_address <> '${program}') THEN
        RAISE EXCEPTION 'Solana chain is already registered with a different program';
      END IF;
      IF NOT EXISTS (SELECT 1 FROM keys WHERE chain_id = ${source}) THEN
        RAISE EXCEPTION 'Canonical host key material is not ready';
      END IF;
    END $$;
    INSERT INTO host_chains (chain_id,name,acl_contract_address)
      VALUES (${SOLANA_HOST_CHAIN_ID},'solana','${program}') ON CONFLICT DO NOTHING;
    INSERT INTO keys (key_id_gw,key_id,pks_key,sks_key,cks_key,sns_pk,compressed_xof_keyset,chain_id,block_hash)
      SELECT k.key_id_gw,k.key_id,k.pks_key,k.sks_key,k.cks_key,k.sns_pk,k.compressed_xof_keyset,${SOLANA_HOST_CHAIN_ID},k.block_hash
      FROM keys k WHERE k.chain_id=${source}
        -- uniq_keys_chain_block_id_gw treats NULL block_hash values as distinct, so a retry must check itself.
        AND NOT EXISTS (SELECT 1 FROM keys s WHERE s.chain_id=${SOLANA_HOST_CHAIN_ID}
          AND s.key_id_gw=k.key_id_gw AND s.block_hash IS NOT DISTINCT FROM k.block_hash)
      ORDER BY k.sequence_number ASC ON CONFLICT DO NOTHING;
    COMMIT;`;
};
