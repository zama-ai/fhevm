// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "gateway_operation_status_type"))]
    pub struct GatewayOperationStatusType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "gateway_operation_type"))]
    pub struct GatewayOperationType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::GatewayOperationType;
    use super::sql_types::GatewayOperationStatusType;

    gateway_requests (request_id) {
        request_id -> Uuid,
        on_chain_request_id -> Nullable<Bytea>,
        op -> GatewayOperationType,
        status -> GatewayOperationStatusType,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::GatewayOperationType;

    gateway_responses (id) {
        id -> Int4,
        on_chain_request_id -> Bytea,
        op -> GatewayOperationType,
        event_log -> Jsonb,
    }
}

diesel::table! {
    httpz_host_events (request_id) {
        request_id -> Uuid,
        event_log -> Jsonb,
        chain_id -> Bytea,
        timestamp -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    gateway_requests,
    gateway_responses,
    httpz_host_events,
);
