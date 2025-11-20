use anyhow::Result;
use uuid::Uuid;

use crate::store::sql::client::PgClient;

pub struct PublicDecryptRepository {
    pool: PgClient,
}

impl PublicDecryptRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }


    // NOTE: We have a query which is performed at the database level in a pg_cron job instead of being called by the internals. and is trigged on this condition:
    // If status == 'receipt_recieved' and now - `updated_at` > 30 min roughly (TBD.)
    // Update status to timed_out with err_reason = 'response timed out' (ACL propagation error).
    // OR IN THE TIMEOUT REPO.

    // INITIAL POST REQUEST:
}
