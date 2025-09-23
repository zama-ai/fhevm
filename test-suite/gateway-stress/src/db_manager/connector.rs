use super::config::DbConnectorConfig;
use super::request_builder::DecryptionRequest;
use anyhow::Result;
use fhevm_gateway_bindings::decryption::Decryption::{PublicDecryptionRequest, UserDecryptionRequest};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use std::time::Duration;
use tracing::{debug, info};

/// Database connector for handling decryption requests
pub struct DbConnector {
    pool: Pool<Postgres>,
    name: String,
}

impl DbConnector {
    /// Create a new database connector
    pub async fn new(config: DbConnectorConfig, index: usize) -> Result<Self> {
        let name = config
            .name
            .clone()
            .unwrap_or_else(|| format!("connector_{}", index));
        
        info!("Connecting to database {} ({})", name, index);
        
        let pool = PgPoolOptions::new()
            .max_connections(config.pool_size.unwrap_or(10))
            .acquire_timeout(Duration::from_secs(config.connection_timeout.unwrap_or(30)))
            .connect(&config.database_url)
            .await?;
        
        info!("Successfully connected to database {}", name);
        
        Ok(Self { pool, name })
    }
    
    /// Check database connectivity
    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await?;
        Ok(())
    }
    
    /// Clear decryption request and response tables
    pub async fn clear_tables(&self) -> Result<()> {
        info!("Clearing database tables for connector {}", self.name);
        
        // Clear request tables
        sqlx::query("DELETE FROM public_decryption_requests")
            .execute(&self.pool)
            .await?;
            
        sqlx::query("DELETE FROM user_decryption_requests")
            .execute(&self.pool)
            .await?;
            
        // Clear response tables (if they exist)
        let _ = sqlx::query("DELETE FROM public_decryption_responses")
            .execute(&self.pool)
            .await;
            
        let _ = sqlx::query("DELETE FROM user_decryption_responses")
            .execute(&self.pool)
            .await;
            
        info!("Database tables cleared for connector {}", self.name);
        Ok(())
    }
    
    /// Insert a decryption request into the database
    pub async fn insert_request(&self, request: DecryptionRequest) -> Result<()> {
        match request {
            DecryptionRequest::Public(req) => {
                self.insert_public_request(req).await?;
            }
            DecryptionRequest::User(req) => {
                self.insert_user_request(req).await?;
            }
        }
        Ok(())
    }
    
    /// Insert a public decryption request
    async fn insert_public_request(&self, request: PublicDecryptionRequest) -> Result<()> {
        // Convert to the same format used by KMS connector
        // Using LITTLE ENDIAN for consistency with actual implementation
        let sns_ciphertexts_db: Vec<serde_json::Value> = request.snsCtMaterials.iter().map(|m| {
            serde_json::json!({
                "ct_handle": hex::encode(m.ctHandle.as_slice()),
                "key_id": hex::encode(m.keyId.to_le_bytes::<32>()), // LITTLE ENDIAN!
                "sns_ciphertext_digest": hex::encode(m.snsCiphertextDigest.as_slice()),
                "coprocessor_tx_sender_addresses": m.coprocessorTxSenderAddresses.iter()
                    .map(|addr| hex::encode(addr.as_slice()))
                    .collect::<Vec<_>>()
            })
        }).collect();
        
        // Note: The actual KMS connector inserts directly with the composite type
        // We need to match the exact format. The tables don't have under_process or created_at in VALUES
        // as these have defaults in the schema
        let query = r#"
            INSERT INTO public_decryption_requests 
            VALUES ($1, $2::sns_ciphertext_material[], $3)
            ON CONFLICT DO NOTHING
        "#;
        
        // Use LITTLE ENDIAN for decryptionId to match KMS connector
        let decryption_id_bytes = request.decryptionId.to_le_bytes::<32>().to_vec();
        sqlx::query(query)
            .bind(&decryption_id_bytes)
            .bind(serde_json::to_string(&sns_ciphertexts_db)?)
            .bind(request.extraData.as_ref())
            .execute(&self.pool)
            .await?;
        
        debug!("Inserted public decryption request {} to {}", 
            hex::encode(&decryption_id_bytes), self.name);
        
        Ok(())
    }
    
    /// Insert a user decryption request
    async fn insert_user_request(&self, request: UserDecryptionRequest) -> Result<()> {
        // Convert to the same format used by KMS connector
        // Using LITTLE ENDIAN for consistency with actual implementation
        let sns_ciphertexts_db: Vec<serde_json::Value> = request.snsCtMaterials.iter().map(|m| {
            serde_json::json!({
                "ct_handle": hex::encode(m.ctHandle.as_slice()),
                "key_id": hex::encode(m.keyId.to_le_bytes::<32>()), // LITTLE ENDIAN!
                "sns_ciphertext_digest": hex::encode(m.snsCiphertextDigest.as_slice()),
                "coprocessor_tx_sender_addresses": m.coprocessorTxSenderAddresses.iter()
                    .map(|addr| hex::encode(addr.as_slice()))
                    .collect::<Vec<_>>()
            })
        }).collect();
        
        // Match the exact format used by KMS connector
        let query = r#"
            INSERT INTO user_decryption_requests 
            VALUES ($1, $2::sns_ciphertext_material[], $3, $4, $5)
            ON CONFLICT DO NOTHING
        "#;
        
        // Use LITTLE ENDIAN for decryptionId to match KMS connector
        let decryption_id_bytes = request.decryptionId.to_le_bytes::<32>().to_vec();
        sqlx::query(query)
            .bind(&decryption_id_bytes)
            .bind(serde_json::to_string(&sns_ciphertexts_db)?)
            .bind(request.userAddress.as_slice())
            .bind(request.publicKey.as_ref())
            .bind(request.extraData.as_ref())
            .execute(&self.pool)
            .await?;
        
        debug!("Inserted user decryption request {} to {}", 
            hex::encode(&decryption_id_bytes), self.name);
        
        Ok(())
    }
    
    /// Fetch decryption responses for specific request IDs
    pub async fn fetch_responses_for_requests(&self, request_ids: &[String]) -> Result<Vec<String>> {
        let mut response_ids = Vec::new();
        
        if request_ids.is_empty() {
            return Ok(response_ids);
        }
        
        // Convert request IDs to bytes for SQL query
        let id_bytes: Vec<Vec<u8>> = request_ids.iter()
            .filter_map(|id| hex::decode(id).ok())
            .collect();
        
        if id_bytes.is_empty() {
            return Ok(response_ids);
        }
        
        // Fetch public decryption responses for our specific IDs
        for id_batch in id_bytes.chunks(100) { // Process in batches to avoid too large queries
            let placeholders: Vec<String> = (1..=id_batch.len())
                .map(|i| format!("${}", i))
                .collect();
            let public_query = format!(
                "SELECT decryption_id FROM public_decryption_responses WHERE decryption_id IN ({})",
                placeholders.join(",")
            );
            
            let mut query = sqlx::query(&public_query);
            for id in id_batch {
                query = query.bind(id);
            }
            
            let public_rows = query.fetch_all(&self.pool).await?;
            
            for row in public_rows {
                let id: Vec<u8> = row.get("decryption_id");
                response_ids.push(hex::encode(id));
            }
            
            // Also check user decryption responses
            let user_query = format!(
                "SELECT decryption_id FROM user_decryption_responses WHERE decryption_id IN ({})",
                placeholders.join(",")
            );
            
            let mut query = sqlx::query(&user_query);
            for id in id_batch {
                query = query.bind(id);
            }
            
            let user_rows = query.fetch_all(&self.pool).await?;
            
            for row in user_rows {
                let id: Vec<u8> = row.get("decryption_id");
                response_ids.push(hex::encode(id));
            }
        }
        
        debug!("Fetched {} responses from {} for {} requests", 
               response_ids.len(), self.name, request_ids.len());
        
        Ok(response_ids)
    }
    
}

