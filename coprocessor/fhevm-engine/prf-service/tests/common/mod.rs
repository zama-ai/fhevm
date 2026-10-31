use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};

pub const GRANTS: &str = include_str!("../../../db-migration/db-scripts/grant_pad_roles.sql");

/// A pool that logs in as `prf_service`, with the rights that the migration job gives it.
pub async fn prf_service_pool(admin: &PgPool, db_url: &str, max_connections: u32) -> PgPool {
    sqlx::raw_sql(
        "DO $$ BEGIN CREATE ROLE prf_service LOGIN PASSWORD 'prf_service';
         EXCEPTION WHEN duplicate_object THEN NULL; END $$",
    )
    .execute(admin)
    .await
    .unwrap();
    sqlx::raw_sql(GRANTS).execute(admin).await.unwrap();
    let options: PgConnectOptions = db_url.parse().unwrap();
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect_with(options.username("prf_service").password("prf_service"))
        .await
        .unwrap()
}
