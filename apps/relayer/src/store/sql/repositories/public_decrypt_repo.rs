// use anyhow::Result;
// use uuid::Uuid;

// use crate::store::sql::client::PgClient;

// pub struct UserDecryptReqRepository {
//     pool: PgClient,
// }

// impl UserDecryptReqRepository {
//     pub fn new(pool: PgClient) -> Self {
//         Self { pool }
//     }

    // INITIAL POST REQUEST:

    // Check if there is already existing internal_indexer_id and return ext_reference_id if there is one
    // Check if there is already an existing internal_indexer_id.
    // Returns the ext_reference_id if found.
    // pub async fn find_ext_ref_by_int_indexer_id(
    //     &self,
    //     int_indexer_id_bytes: &[u8],
    // ) -> Result<Option<Uuid>> {
    //     let result = sqlx::query_scalar!(
    //         r#"
    //         SELECT ext_reference_id
    //         FROM public_decrypt_req
    //         WHERE int_indexer_id = $1
    //         LIMIT 1
    //         "#,
    //         int_indexer_id_bytes
    //     )
    //     .fetch_optional(&self.pool.get_pool())
    //     .await?;

    //     Ok(result)
    // }

    // if success (not conflict)
    // / Insert req, ext_reference_id, int_indexer_id.
    // / If conflict on int_indexer_id, it returns the EXISTING ext_reference_id.
    // / If no conflict, it inserts and returns the NEW ext_reference_id.


// }
