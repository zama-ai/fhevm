use std::io::Write;

use alloy::primitives::{Address, Bytes, FixedBytes};
use alloy::rpc::types::{Log, TransactionReceipt};
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::query_builder::QueryId;
use diesel::serialize::ToSql;
use diesel::sql_types::SqlType;
use diesel::{Insertable, Queryable};
use fhevm_relayer::orchestrator::traits::Event;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// TODO: maybe rename some types defined here

// - Public decryption
//   - Authorization Request
//   - Authorization Response
// - Private decryption
//   - Private decryption Request
//   - Private decryption Response
// - Input mechanism
//   - TODO

// NOTE: Box<dyn Event> vs Enum implementing Event trait

// TODO: prefix events with relayer
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ZwsRelayerEvent {
    /// Host is usually a public decryption request made on the Oracle contract
    #[serde(rename = "relayer:blockchain:host-event")]
    BlockchainEvent(BlockchainEvent),
    /// Gateway Event is usually a response from one of the HTTPZ contracts
    /// i.e. Decryption Manager
    #[serde(rename = "relayer:blockchain:gateway-event")]
    HTTPZGatewayEvent(HTTPZGatewayEvent),
    /// Just a debug message to check communication issues
    #[serde(rename = "relayer:sqs:debug")]
    SQSRelayerDebugMessage(SQSRelayerDebugMessage),
    /// Input registration request, should come from the console back
    /// but for debugging purposes the HTTP listener can also handle them
    #[serde(rename = "relayer:input-registration:input-registration-request")]
    SQSRelayerInputRegistrationRequest(SQSRelayerInputRegistrationRequest),
    /// Input registration response to answer a user
    #[serde(rename = "relayer:input-registration:input-registration-response")]
    SQSRelayerInputRegistrationResponse(SQSRelayerInputRegistrationResponse),
    /// Authorization request made from the relayer to the console to check
    /// that a given contract is whitelisted
    #[serde(rename = "relayer:public-decryption:authorization-request")]
    SQSRelayerAuthorizationRequest(SQSRelayerAuthorizationRequest),
    /// Authorization response from the console w.r.t. a given contract
    #[serde(rename = "relayer:public-decryption:authorization-response")]
    SQSRelayerAuthorizationResponse(SQSRelayerAuthorizationResponse),
    /// Private decryption request from the HTTP endpoint or the Console
    #[serde(rename = "relayer:private-decryption:operation-request")]
    SQSRelayerPrivateDecryptionRequest(SQSRelayerPrivateDecryptionRequest),
    /// Private decryption response
    #[serde(rename = "relayer:private-decryption:operation-response")]
    SQSRelayerPrivateDecryptionResponse(SQSRelayerPrivateDecryptionResponse),
    /// Transaction request
    #[serde(rename = "relayer:transaction:tx-request")]
    SQSRelayerTransactionRequest(SQSRelayerTransactionRequest),
    /// Transaction response, i.e. receipt and response
    #[serde(rename = "relayer:transaction:tx-response")]
    SQSRelayerTransactionResponse(Box<SQSRelayerTransactionResponse>),
}

// TODO: clean this with a macro
impl Event for ZwsRelayerEvent {
    fn event_name(&self) -> &str {
        match self {
            Self::BlockchainEvent(value) => value.event_name(),
            Self::HTTPZGatewayEvent(value) => value.event_name(),
            Self::SQSRelayerDebugMessage(value) => value.event_name(),
            Self::SQSRelayerAuthorizationRequest(value) => value.event_name(),
            Self::SQSRelayerAuthorizationResponse(value) => value.event_name(),
            Self::SQSRelayerTransactionRequest(value) => value.event_name(),
            Self::SQSRelayerTransactionResponse(value) => value.event_name(),
            Self::SQSRelayerPrivateDecryptionRequest(value) => value.event_name(),
            Self::SQSRelayerPrivateDecryptionResponse(value) => value.event_name(),
            Self::SQSRelayerInputRegistrationRequest(value) => value.event_name(),
            Self::SQSRelayerInputRegistrationResponse(value) => value.event_name(),
        }
    }

    fn event_id(&self) -> u8 {
        match self {
            Self::BlockchainEvent(value) => value.event_id(),
            Self::HTTPZGatewayEvent(value) => value.event_id(),
            Self::SQSRelayerDebugMessage(value) => value.event_id(),
            Self::SQSRelayerAuthorizationRequest(value) => value.event_id(),
            Self::SQSRelayerAuthorizationResponse(value) => value.event_id(),
            Self::SQSRelayerTransactionRequest(value) => value.event_id(),
            Self::SQSRelayerTransactionResponse(value) => value.event_id(),
            Self::SQSRelayerPrivateDecryptionRequest(value) => value.event_id(),
            Self::SQSRelayerPrivateDecryptionResponse(value) => value.event_id(),
            Self::SQSRelayerInputRegistrationRequest(value) => value.event_id(),
            Self::SQSRelayerInputRegistrationResponse(value) => value.event_id(),
        }
    }

    fn request_id(&self) -> uuid::Uuid {
        match self {
            Self::BlockchainEvent(value) => value.request_id(),
            Self::HTTPZGatewayEvent(value) => value.request_id(),
            Self::SQSRelayerDebugMessage(value) => value.request_id(),
            Self::SQSRelayerAuthorizationRequest(value) => value.request_id(),
            Self::SQSRelayerAuthorizationResponse(value) => value.request_id(),
            Self::SQSRelayerTransactionRequest(value) => value.request_id(),
            Self::SQSRelayerTransactionResponse(value) => value.request_id(),
            Self::SQSRelayerPrivateDecryptionRequest(value) => value.request_id(),
            Self::SQSRelayerPrivateDecryptionResponse(value) => value.request_id(),
            Self::SQSRelayerInputRegistrationRequest(value) => value.request_id(),
            Self::SQSRelayerInputRegistrationResponse(value) => value.request_id(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockchainEvent {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    #[serde(rename = "eventLog")]
    pub event_log: alloy::rpc::types::Log,
}

#[derive(Clone, Debug, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name=httpz_host_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BlockchainEventRow {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    #[serde(rename = "eventLog")]
    pub event_log: diesel_json::Json<Log>,
}

impl From<BlockchainEvent> for BlockchainEventRow {
    fn from(event: BlockchainEvent) -> Self {
        Self {
            request_id: event.request_id,
            event_log: diesel_json::Json(event.event_log),
        }
    }
}

impl From<BlockchainEventRow> for BlockchainEvent {
    fn from(row: BlockchainEventRow) -> Self {
        Self {
            request_id: row.request_id,
            event_log: row.event_log.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name=gateway_responses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GatewayResponseRow {
    #[serde(rename = "dbId")]
    pub id: i32,
    #[serde(rename = "onChainRequestId")]
    pub on_chain_request_id: Vec<u8>,
    #[serde(rename = "eventLog")]
    pub event_log: diesel_json::Json<alloy::rpc::types::Log>,
    #[serde(rename = "op")]
    pub op: GatewayOperation,
}

#[derive(Clone, Debug, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name=gateway_responses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewGatewayResponseRow {
    #[serde(rename = "onChainRequestId")]
    pub on_chain_request_id: Vec<u8>,
    #[serde(rename = "eventLog")]
    pub event_log: diesel_json::Json<alloy::rpc::types::Log>,
    #[serde(rename = "op")]
    pub op: GatewayOperation,
}

#[derive(SqlType, QueryId)]
#[diesel(postgres_type(name = "gateway_operation_type"))]
pub struct GatewayOperationType;

#[derive(
    Clone, Serialize, Deserialize, Debug, PartialEq, FromSqlRow, AsExpression, Eq, QueryId,
)]
#[diesel(sql_type = GatewayOperationType)]
pub enum GatewayOperation {
    PublicDecryption,
    PrivateDecryption,
    InputRegistration,
}

impl ToSql<GatewayOperationType, Pg> for GatewayOperation {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match *self {
            GatewayOperation::PublicDecryption => out.write_all(b"PublicDecryption")?,
            GatewayOperation::PrivateDecryption => out.write_all(b"PrivateDecryption")?,
            GatewayOperation::InputRegistration => out.write_all(b"InputRegistration")?,
        }
        Ok(diesel::serialize::IsNull::No)
    }
}

impl FromSql<GatewayOperationType, Pg> for GatewayOperation {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"PublicDecryption" => Ok(GatewayOperation::PublicDecryption),
            b"PrivateDecryption" => Ok(GatewayOperation::PrivateDecryption),
            b"InputRegistration" => Ok(GatewayOperation::InputRegistration),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(SqlType)]
#[diesel(postgres_type(name = "gateway_operation_status_type"))]
pub struct GatewayOperationStatusType;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = GatewayOperationStatusType)]
pub enum GatewayOperationStatus {
    /// Transaction has been requested to the transaction manager
    TXRequested,
    /// Transaction request has been fulfilled by the transaction manager
    TXFulfilled,
    /// Response for request was emitted on-chain
    ResponseFulfilled,
}

impl ToSql<GatewayOperationStatusType, Pg> for GatewayOperationStatus {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match *self {
            GatewayOperationStatus::TXRequested => out.write_all(b"TXRequested")?,
            GatewayOperationStatus::TXFulfilled => out.write_all(b"TXFulfilled")?,
            GatewayOperationStatus::ResponseFulfilled => out.write_all(b"ResponseFulfilled")?,
        }
        Ok(diesel::serialize::IsNull::No)
    }
}

impl FromSql<GatewayOperationStatusType, Pg> for GatewayOperationStatus {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"TXRequested" => Ok(GatewayOperationStatus::TXRequested),
            b"TXFulfilled" => Ok(GatewayOperationStatus::TXFulfilled),
            b"ResponseFulfilled" => Ok(GatewayOperationStatus::ResponseFulfilled),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name=gateway_requests)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GatewayRequestRow {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    #[serde(rename = "op")]
    pub op: GatewayOperation,
    #[serde(rename = "status")]
    pub status: GatewayOperationStatus,
    #[serde(rename = "onChainRequestId")]
    pub on_chain_request_id: Option<Vec<u8>>,
}

table! {
    use diesel::sql_types::*;
    use super::GatewayOperationStatusType;
    use super::GatewayOperationType;

    gateway_requests (request_id) {
        request_id -> Uuid,
        op -> GatewayOperationType,
        status -> GatewayOperationStatusType,
        on_chain_request_id -> Nullable<Bytea>,
    }
}

table! {
    use diesel::sql_types::*;

    httpz_host_events (request_id) {
        request_id -> Uuid,
        event_log -> Jsonb,
    }
}

table! {
    use diesel::sql_types::*;
    use super::GatewayOperationType;

    gateway_responses (id) {
        id -> Int4,
        on_chain_request_id -> Bytea,
        event_log -> Jsonb,
        op -> GatewayOperationType,
    }
}

// TODO: rename all `create_*` into `insert_*`
pub fn create_host_event(
    conn: &mut PgConnection,
    event: BlockchainEvent,
) -> Result<BlockchainEventRow, diesel::result::Error> {
    diesel::insert_into(httpz_host_events::table)
        .values(&(BlockchainEventRow::from(event)))
        .get_result(conn)
}

pub fn create_gateway_response(
    conn: &mut PgConnection,
    event: NewGatewayResponseRow,
) -> Result<GatewayResponseRow, diesel::result::Error> {
    diesel::insert_into(gateway_responses::table)
        .values(event)
        .get_result(conn)
}

pub fn create_gateway_request(
    conn: &mut PgConnection,
    row: GatewayRequestRow,
) -> Result<GatewayRequestRow, diesel::result::Error> {
    diesel::insert_into(gateway_requests::table)
        .values(row)
        .get_result(conn)
}

pub fn fetch_host_event(
    conn: &mut PgConnection,
    req_id: Uuid,
) -> Result<BlockchainEvent, diesel::result::Error> {
    httpz_host_events::dsl::httpz_host_events
        .find(req_id)
        .first::<BlockchainEventRow>(conn)
        .map(Into::into)
}

pub fn fetch_gateway_request(
    conn: &mut PgConnection,
    req_id: Uuid,
) -> Result<GatewayRequestRow, diesel::result::Error> {
    gateway_requests::dsl::gateway_requests
        .find(req_id)
        .first::<GatewayRequestRow>(conn)
}

pub fn fetch_gateway_response(
    conn: &mut PgConnection,
    req_id: Vec<u8>,
    op: GatewayOperation,
) -> Result<Vec<GatewayResponseRow>, diesel::result::Error> {
    gateway_responses::dsl::gateway_responses
        .filter(gateway_responses::dsl::on_chain_request_id.eq(req_id))
        .filter(gateway_responses::dsl::op.eq(op))
        .limit(1)
        .select(GatewayResponseRow::as_select())
        .load(conn)
}

// TODO: actually we also need to store the event type since we could have decryption-id ==
// zkpok-id
pub fn fetch_gateway_request_chain_id(
    conn: &mut PgConnection,
    on_chain_req_id: Vec<u8>,
) -> Result<Vec<GatewayRequestRow>, diesel::result::Error> {
    gateway_requests::dsl::gateway_requests
        .filter(gateway_requests::dsl::on_chain_request_id.eq(on_chain_req_id))
        .limit(1)
        .select(GatewayRequestRow::as_select())
        .load(conn)
}

pub fn update_gateway_request_status(
    conn: &mut PgConnection,
    req_id: Uuid,
    status: GatewayOperationStatus,
) -> Result<GatewayRequestRow, diesel::result::Error> {
    diesel::update(gateway_requests::dsl::gateway_requests.find(req_id))
        .set(gateway_requests::dsl::status.eq(status))
        .returning(GatewayRequestRow::as_returning())
        .get_result(conn)
}

pub fn update_gateway_request_onchain_id(
    conn: &mut PgConnection,
    req_id: Uuid,
    status: GatewayOperationStatus,
    on_chain_id: Option<Vec<u8>>,
) -> Result<GatewayRequestRow, diesel::result::Error> {
    diesel::update(gateway_requests::dsl::gateway_requests.find(req_id))
        .set((
            gateway_requests::dsl::status.eq(status),
            gateway_requests::dsl::on_chain_request_id.eq(on_chain_id),
        ))
        .returning(GatewayRequestRow::as_returning())
        .get_result(conn)
}

// TODO: remove
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HTTPZGatewayEvent {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SQSRelayerDebugMessage {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub message: String,
}

// TODO:
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SQSRelayerInputRegistrationRequest {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    #[serde(rename = "contractChainId")]
    pub contract_chain_id: u64,
    #[serde(rename = "contractAddress")]
    pub contract_address: Address,
    #[serde(rename = "userAddress")]
    pub user_address: Address,
    #[serde(rename = "ciphertextWithZkpok")]
    pub ciphetext_with_zk_proof: Bytes,
}

// TODO:
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SQSRelayerInputRegistrationResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    #[serde(rename = "handles")]
    pub handles: Vec<FixedBytes<32>>,
    #[serde(rename = "signatures")]
    pub signatures: Vec<Bytes>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SQSRelayerAuthorizationRequest {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    #[serde(rename = "callerAddress")]
    pub caller_address: Address,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SQSRelayerAuthorizationResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub authorized: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SQSRelayerTransactionRequest {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub address: Address,
    pub calldata: Bytes,
    #[serde(rename = "chainId")]
    pub chain_id: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SQSRelayerTransactionResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    /// Transaction receipt, holds the logs, the gas used, ...
    pub receipt: TransactionReceipt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SQSRelayerPrivateDecryptionRequest {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    #[serde(rename = "ctHandles")]
    pub ct_handles: Vec<Bytes>,
    #[serde(rename = "publicKey")]
    pub pub_key: Bytes,
    #[serde(rename = "chainId")]
    pub chain_id: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SQSRelayerPrivateDecryptionResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    #[serde(rename = "ctValues")]
    pub ct_values: Vec<Bytes>,
    pub signatures: Vec<Bytes>,
}

macro_rules! impl_event {
    ($struct_name:ident, $event_id:expr) => {
        impl Event for $struct_name {
            fn event_name(&self) -> &str {
                stringify!($struct_name)
            }

            fn event_id(&self) -> u8 {
                $event_id
            }

            fn request_id(&self) -> uuid::Uuid {
                self.request_id
            }
        }

        impl $struct_name {
            pub fn event_id() -> u8 {
                $event_id
            }
        }
    };
}

// TODO: document these values
// TODO: figure out if there is a way to register an enum directly instead of these values
// TODO: Maybe add a check to make sure that there is no value conflict
impl_event!(BlockchainEvent, 1);
impl_event!(HTTPZGatewayEvent, 2);
impl_event!(SQSRelayerDebugMessage, 3);
impl_event!(SQSRelayerAuthorizationRequest, 4);
impl_event!(SQSRelayerAuthorizationResponse, 5);
impl_event!(SQSRelayerTransactionRequest, 6);
impl_event!(SQSRelayerTransactionResponse, 7);
impl_event!(SQSRelayerPrivateDecryptionRequest, 8);
impl_event!(SQSRelayerPrivateDecryptionResponse, 9);
impl_event!(SQSRelayerInputRegistrationRequest, 10);
impl_event!(SQSRelayerInputRegistrationResponse, 11);
