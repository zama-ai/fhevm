use std::fmt::Display;
use std::io::Write;

use alloy::primitives::{Address, Bytes, FixedBytes};
use alloy::rpc::types::{AnyReceiptEnvelope, Log, TransactionReceipt};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::query_builder::QueryId;
use diesel::serialize::ToSql;
use diesel::sql_types::SqlType;
use diesel::{Insertable, Queryable};
use fhevm_relayer::core::event::{HandleContractPair, RequestValidity};
use fhevm_relayer::http::userdecrypt_http_listener::UserDecryptResponsePayloadJson;
use fhevm_relayer::orchestrator::traits::Event;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use tracing::{debug, error};
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
//

// Convert u64 to Vec<u8>
fn u64_to_bytes(value: u64) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(8);
    bytes.write_u64::<BigEndian>(value).unwrap();
    bytes
}

pub fn bytes_to_u64(bytes: &[u8]) -> Result<u64, std::io::Error> {
    if bytes.len() != 8 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid byte length for u64",
        ));
    }
    let mut cursor = Cursor::new(bytes);
    cursor.read_u64::<BigEndian>()
}

// TODO: prefix events with relayer
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ZwsRelayerEvent {
    // Blockchain
    /// Host is usually a public decryption request made on the Oracle contract
    #[serde(rename = "relayer:blockchain:host-event")]
    BlockchainEvent(BlockchainEvent),
    /// Gateway Event is usually a response from one of the HTTPZ contracts
    /// i.e. Decryption Manager
    #[serde(rename = "relayer:blockchain:gateway-event")]
    HTTPZGatewayEvent(HTTPZGatewayEvent),
    /// Just a debug message to check communication issues
    #[serde(rename = "relayer:sqs:debug")]
    DebugMessage(DebugMessage),

    // Console
    // Input Registration
    /// Input registration request, should come from the console back
    /// but for debugging purposes the builtin HTTP listener can also handle them
    #[serde(rename = "relayer:input-registration:input-registration-request")]
    HTTPInputRegistrationRequest(HTTPInputRegistrationRequest),
    /// Input registration response to answer a user
    #[serde(rename = "relayer:input-registration:input-registration-response")]
    HTTPInputRegistrationResponse(HTTPInputRegistrationResponse),
    // Private Decryption
    /// Private decryption request from the HTTP endpoint or the Console
    #[serde(rename = "relayer:http-private-decryption:operation-request")]
    HTTPPrivateDecryptionRequest(PrivateDecryptionRequest),
    /// Private decryption response
    #[serde(rename = "relayer:http-private-decryption:operation-response")]
    HTTPPrivateDecryptionResponse(PrivateDecryptionResponse),
    // Public Decryption
    // TODO: implement
    #[serde(rename = "relayer:http-public-decryption:operation-request")]
    HTTPPublicDecryptionRequest(PrivateDecryptionRequest),
    /// Private decryption response
    #[serde(rename = "relayer:http-public-decryption:operation-response")]
    HTTPPublicDecryptionResponse(PrivateDecryptionResponse),
    // TODO: implement
    #[serde(rename = "relayer:oracle-public-decryption:operation-request")]
    OraclePublicDecryptionRequest(PrivateDecryptionRequest),
    /// Private decryption response
    #[serde(rename = "relayer:oracle-public-decryption:operation-response")]
    OraclePublicDecryptionResponse(PrivateDecryptionResponse),
    /// Authorization request made from the relayer to the console to check
    /// that a given contract is whitelisted
    #[serde(rename = "relayer:public-decryption:authorization-request")]
    OracleAuthorizationRequest(OracleAuthorizationRequest),
    /// Authorization response from the console w.r.t. a given contract
    #[serde(rename = "relayer:public-decryption:authorization-response")]
    OracleAuthorizationResponse(OracleAuthorizationResponse),

    // TX-Manager
    /// Transaction request
    #[serde(rename = "relayer:transaction:tx-request")]
    TransactionRequest(TransactionRequest),
    /// Transaction response, i.e. receipt and response
    #[serde(rename = "relayer:transaction:tx-response")]
    TransactionResponse(Box<TransactionResponse>),

    // Errors
    #[serde(rename = "relayer:error:unrecoverable")]
    UnrecoverableError(UnrecoverableError),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum RelayerEventNoError {
    // Blockchain
    /// Host is usually a public decryption request made on the Oracle contract
    #[serde(rename = "relayer:blockchain:host-event")]
    BlockchainEvent(BlockchainEvent),
    /// Gateway Event is usually a response from one of the HTTPZ contracts
    /// i.e. Decryption Manager
    #[serde(rename = "relayer:blockchain:gateway-event")]
    HTTPZGatewayEvent(HTTPZGatewayEvent),
    /// Just a debug message to check communication issues
    #[serde(rename = "relayer:sqs:debug")]
    DebugMessage(DebugMessage),

    // Console
    // Input Registration
    /// Input registration request, should come from the console back
    /// but for debugging purposes the builtin HTTP listener can also handle them
    #[serde(rename = "relayer:input-registration:input-registration-request")]
    HTTPInputRegistrationRequest(HTTPInputRegistrationRequest),
    /// Input registration response to answer a user
    #[serde(rename = "relayer:input-registration:input-registration-response")]
    HTTPInputRegistrationResponse(HTTPInputRegistrationResponse),
    // Private Decryption
    /// Private decryption request from the HTTP endpoint or the Console
    #[serde(rename = "relayer:http-private-decryption:operation-request")]
    HTTPPrivateDecryptionRequest(PrivateDecryptionRequest),
    /// Private decryption response
    #[serde(rename = "relayer:http-private-decryption:operation-response")]
    HTTPPrivateDecryptionResponse(PrivateDecryptionResponse),
    // Public Decryption
    // TODO: implement
    #[serde(rename = "relayer:http-public-decryption:operation-request")]
    HTTPPublicDecryptionRequest(PrivateDecryptionRequest),
    /// Private decryption response
    #[serde(rename = "relayer:http-public-decryption:operation-response")]
    HTTPPublicDecryptionResponse(PrivateDecryptionResponse),
    // TODO: implement
    #[serde(rename = "relayer:oracle-public-decryption:operation-request")]
    OraclePublicDecryptionRequest(PrivateDecryptionRequest),
    /// Private decryption response
    #[serde(rename = "relayer:oracle-public-decryption:operation-response")]
    OraclePublicDecryptionResponse(PrivateDecryptionResponse),
    /// Authorization request made from the relayer to the console to check
    /// that a given contract is whitelisted
    #[serde(rename = "relayer:public-decryption:authorization-request")]
    OracleAuthorizationRequest(OracleAuthorizationRequest),
    /// Authorization response from the console w.r.t. a given contract
    #[serde(rename = "relayer:public-decryption:authorization-response")]
    OracleAuthorizationResponse(OracleAuthorizationResponse),

    // TX-Manager
    /// Transaction request
    #[serde(rename = "relayer:transaction:tx-request")]
    TransactionRequest(TransactionRequest),
    /// Transaction response, i.e. receipt and response
    #[serde(rename = "relayer:transaction:tx-response")]
    TransactionResponse(Box<TransactionResponse>),
}

// Implementation to convert from SubEnum to OriginalEnum
impl From<RelayerEventNoError> for ZwsRelayerEvent {
    fn from(sub: RelayerEventNoError) -> Self {
        match sub {
            RelayerEventNoError::BlockchainEvent(n) => ZwsRelayerEvent::BlockchainEvent(n),
            RelayerEventNoError::TransactionResponse(n) => ZwsRelayerEvent::TransactionResponse(n),
            RelayerEventNoError::DebugMessage(n) => ZwsRelayerEvent::DebugMessage(n),
            RelayerEventNoError::HTTPZGatewayEvent(n) => ZwsRelayerEvent::HTTPZGatewayEvent(n),
            RelayerEventNoError::TransactionRequest(n) => ZwsRelayerEvent::TransactionRequest(n),
            RelayerEventNoError::OracleAuthorizationRequest(n) => {
                ZwsRelayerEvent::OracleAuthorizationRequest(n)
            }
            RelayerEventNoError::OracleAuthorizationResponse(n) => {
                ZwsRelayerEvent::OracleAuthorizationResponse(n)
            }
            RelayerEventNoError::HTTPInputRegistrationRequest(n) => {
                ZwsRelayerEvent::HTTPInputRegistrationRequest(n)
            }
            RelayerEventNoError::HTTPPublicDecryptionRequest(n) => {
                ZwsRelayerEvent::HTTPPublicDecryptionRequest(n)
            }
            RelayerEventNoError::HTTPPrivateDecryptionRequest(n) => {
                ZwsRelayerEvent::HTTPPrivateDecryptionRequest(n)
            }
            RelayerEventNoError::HTTPInputRegistrationResponse(n) => {
                ZwsRelayerEvent::HTTPInputRegistrationResponse(n)
            }
            RelayerEventNoError::HTTPPublicDecryptionResponse(n) => {
                ZwsRelayerEvent::HTTPPublicDecryptionResponse(n)
            }
            RelayerEventNoError::HTTPPrivateDecryptionResponse(n) => {
                ZwsRelayerEvent::HTTPPrivateDecryptionResponse(n)
            }
            RelayerEventNoError::OraclePublicDecryptionRequest(n) => {
                ZwsRelayerEvent::OraclePublicDecryptionRequest(n)
            }
            RelayerEventNoError::OraclePublicDecryptionResponse(n) => {
                ZwsRelayerEvent::OraclePublicDecryptionResponse(n)
            }
        }
    }
}

impl Display for RelayerEventNoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.event_name(), self.request_id())
    }
}

impl Event for RelayerEventNoError {
    fn event_name(&self) -> &str {
        match self {
            Self::BlockchainEvent(value) => value.event_name(),
            Self::HTTPZGatewayEvent(value) => value.event_name(),
            Self::DebugMessage(value) => value.event_name(),
            Self::OracleAuthorizationRequest(value) => value.event_name(),
            Self::OracleAuthorizationResponse(value) => value.event_name(),
            Self::TransactionRequest(value) => value.event_name(),
            Self::TransactionResponse(value) => value.event_name(),
            Self::HTTPPrivateDecryptionRequest(value) => value.event_name(),
            Self::HTTPPrivateDecryptionResponse(value) => value.event_name(),
            Self::HTTPPublicDecryptionRequest(value) => value.event_name(),
            Self::HTTPPublicDecryptionResponse(value) => value.event_name(),
            Self::OraclePublicDecryptionRequest(value) => value.event_name(),
            Self::OraclePublicDecryptionResponse(value) => value.event_name(),
            Self::HTTPInputRegistrationRequest(value) => value.event_name(),
            Self::HTTPInputRegistrationResponse(value) => value.event_name(),
        }
    }

    fn event_id(&self) -> u8 {
        match self {
            Self::BlockchainEvent(value) => value.event_id(),
            Self::HTTPZGatewayEvent(value) => value.event_id(),
            Self::DebugMessage(value) => value.event_id(),
            Self::OracleAuthorizationRequest(value) => value.event_id(),
            Self::OracleAuthorizationResponse(value) => value.event_id(),
            Self::TransactionRequest(value) => value.event_id(),
            Self::TransactionResponse(value) => value.event_id(),
            Self::HTTPPrivateDecryptionRequest(value) => value.event_id(),
            Self::HTTPPrivateDecryptionResponse(value) => value.event_id(),
            Self::HTTPPublicDecryptionRequest(value) => value.event_id(),
            Self::HTTPPublicDecryptionResponse(value) => value.event_id(),
            Self::OraclePublicDecryptionRequest(value) => value.event_id(),
            Self::OraclePublicDecryptionResponse(value) => value.event_id(),
            Self::HTTPInputRegistrationRequest(value) => value.event_id(),
            Self::HTTPInputRegistrationResponse(value) => value.event_id(),
        }
    }

    fn request_id(&self) -> uuid::Uuid {
        match self {
            Self::BlockchainEvent(value) => value.request_id(),
            Self::HTTPZGatewayEvent(value) => value.request_id(),
            Self::DebugMessage(value) => value.request_id(),
            Self::OracleAuthorizationRequest(value) => value.request_id(),
            Self::OracleAuthorizationResponse(value) => value.request_id(),
            Self::TransactionRequest(value) => value.request_id(),
            Self::TransactionResponse(value) => value.request_id(),
            Self::HTTPPrivateDecryptionRequest(value) => value.request_id(),
            Self::HTTPPrivateDecryptionResponse(value) => value.request_id(),
            Self::HTTPPublicDecryptionRequest(value) => value.request_id(),
            Self::HTTPPublicDecryptionResponse(value) => value.request_id(),
            Self::OraclePublicDecryptionRequest(value) => value.request_id(),
            Self::OraclePublicDecryptionResponse(value) => value.request_id(),
            Self::HTTPInputRegistrationRequest(value) => value.request_id(),
            Self::HTTPInputRegistrationResponse(value) => value.request_id(),
        }
    }
}

impl Display for ZwsRelayerEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.event_name(), self.request_id())
    }
}

// TODO: clean this with a macro
impl Event for ZwsRelayerEvent {
    fn event_name(&self) -> &str {
        match self {
            Self::BlockchainEvent(value) => value.event_name(),
            Self::HTTPZGatewayEvent(value) => value.event_name(),
            Self::DebugMessage(value) => value.event_name(),
            Self::OracleAuthorizationRequest(value) => value.event_name(),
            Self::OracleAuthorizationResponse(value) => value.event_name(),
            Self::TransactionRequest(value) => value.event_name(),
            Self::TransactionResponse(value) => value.event_name(),
            Self::HTTPPrivateDecryptionRequest(value) => value.event_name(),
            Self::HTTPPrivateDecryptionResponse(value) => value.event_name(),
            Self::HTTPPublicDecryptionRequest(value) => value.event_name(),
            Self::HTTPPublicDecryptionResponse(value) => value.event_name(),
            Self::OraclePublicDecryptionRequest(value) => value.event_name(),
            Self::OraclePublicDecryptionResponse(value) => value.event_name(),
            Self::HTTPInputRegistrationRequest(value) => value.event_name(),
            Self::HTTPInputRegistrationResponse(value) => value.event_name(),
            Self::UnrecoverableError(value) => value.event_name(),
        }
    }

    fn event_id(&self) -> u8 {
        match self {
            Self::BlockchainEvent(value) => value.event_id(),
            Self::HTTPZGatewayEvent(value) => value.event_id(),
            Self::DebugMessage(value) => value.event_id(),
            Self::OracleAuthorizationRequest(value) => value.event_id(),
            Self::OracleAuthorizationResponse(value) => value.event_id(),
            Self::TransactionRequest(value) => value.event_id(),
            Self::TransactionResponse(value) => value.event_id(),
            Self::HTTPPrivateDecryptionRequest(value) => value.event_id(),
            Self::HTTPPrivateDecryptionResponse(value) => value.event_id(),
            Self::HTTPPublicDecryptionRequest(value) => value.event_id(),
            Self::HTTPPublicDecryptionResponse(value) => value.event_id(),
            Self::OraclePublicDecryptionRequest(value) => value.event_id(),
            Self::OraclePublicDecryptionResponse(value) => value.event_id(),
            Self::HTTPInputRegistrationRequest(value) => value.event_id(),
            Self::HTTPInputRegistrationResponse(value) => value.event_id(),
            Self::UnrecoverableError(value) => value.event_id(),
        }
    }

    fn request_id(&self) -> uuid::Uuid {
        match self {
            Self::BlockchainEvent(value) => value.request_id(),
            Self::HTTPZGatewayEvent(value) => value.request_id(),
            Self::DebugMessage(value) => value.request_id(),
            Self::OracleAuthorizationRequest(value) => value.request_id(),
            Self::OracleAuthorizationResponse(value) => value.request_id(),
            Self::TransactionRequest(value) => value.request_id(),
            Self::TransactionResponse(value) => value.request_id(),
            Self::HTTPPrivateDecryptionRequest(value) => value.request_id(),
            Self::HTTPPrivateDecryptionResponse(value) => value.request_id(),
            Self::HTTPPublicDecryptionRequest(value) => value.request_id(),
            Self::HTTPPublicDecryptionResponse(value) => value.request_id(),
            Self::OraclePublicDecryptionRequest(value) => value.request_id(),
            Self::OraclePublicDecryptionResponse(value) => value.request_id(),
            Self::HTTPInputRegistrationRequest(value) => value.request_id(),
            Self::HTTPInputRegistrationResponse(value) => value.request_id(),
            Self::UnrecoverableError(value) => value.request_id(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockchainEvent {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    #[serde(rename = "eventLog")]
    pub event_log: alloy::rpc::types::Log,
    #[serde(rename = "chainId")]
    pub chain_id: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name=httpz_host_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BlockchainEventRow {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    #[serde(rename = "eventLog")]
    pub event_log: diesel_json::Json<Log>,
    #[serde(rename = "chainId")]
    #[diesel(sql_type = Binary)]
    pub chain_id: Vec<u8>,
}

impl BlockchainEventRow {
    fn from(event: BlockchainEvent) -> Self {
        Self {
            request_id: event.request_id,
            event_log: diesel_json::Json(event.event_log),
            chain_id: u64_to_bytes(event.chain_id),
        }
    }
}

impl TryFrom<BlockchainEventRow> for BlockchainEvent {
    type Error = std::io::Error;

    fn try_from(row: BlockchainEventRow) -> Result<Self, Self::Error> {
        let chain_id = bytes_to_u64(&row.chain_id)?;

        Ok(Self {
            request_id: row.request_id,
            event_log: row.event_log.0,
            chain_id,
        })
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
    Clone, Serialize, Deserialize, Debug, PartialEq, FromSqlRow, AsExpression, Eq, QueryId, Copy,
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

// TODO: Add date/block-number
table! {
    use diesel::sql_types::*;

    httpz_host_events (request_id) {
        request_id -> Uuid,
        event_log -> Jsonb,
        chain_id -> Bytea,
    }
}

// TODO: Add date/block-number
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
) -> Result<BlockchainEvent, Box<dyn std::error::Error>> {
    httpz_host_events::dsl::httpz_host_events
        .find(req_id)
        .first::<BlockchainEventRow>(conn)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        .and_then(|row| {
            row.try_into()
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        })
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
    op_type: GatewayOperation,
) -> Result<Vec<GatewayRequestRow>, diesel::result::Error> {
    gateway_requests::dsl::gateway_requests
        .filter(gateway_requests::dsl::on_chain_request_id.eq(on_chain_req_id))
        .filter(gateway_requests::dsl::op.eq(op_type))
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
pub struct DebugMessage {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnrecoverableError {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub event: RelayerEventNoError,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HTTPInputRegistrationRequest {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    #[serde(rename = "contractChainId")]
    pub contract_chain_id: u64,
    #[serde(rename = "contractAddress")]
    pub contract_address: Address,
    #[serde(rename = "userAddress")]
    pub user_address: Address,
    #[serde(rename = "ciphertextWithInputVerification")]
    pub ciphetext_with_zk_proof: Bytes,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HTTPInputRegistrationResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    #[serde(rename = "handles")]
    pub handles: Vec<FixedBytes<32>>,
    #[serde(rename = "signatures")]
    pub signatures: Vec<Bytes>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HTTPPrivateDecryptionRequest {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    #[serde(rename = "contractsChainId")]
    pub contracts_chain_id: u64,
    #[serde(rename = "ctHandleContractPairs")]
    pub ct_handle_contract_pairs: Vec<HandleContractPair>,
    #[serde(rename = "requestValidity")]
    pub request_validity: RequestValidity,
    #[serde(rename = "contractsAddresses")]
    pub contract_addresses: Vec<Address>,
    #[serde(rename = "userAddress")]
    pub user_address: Address,
    #[serde(rename = "signature")]
    pub signature: Bytes,
    #[serde(rename = "publicKey")]
    pub public_key: Bytes,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HTTPPrivateDecryptionResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    #[serde(rename = "gatewayRequestId")]
    pub gateway_request_id: u64,
    #[serde(rename = "decryptedValue")]
    pub decrypted_value: Bytes,
    #[serde(rename = "signatures")]
    pub signatures: Vec<Bytes>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OracleAuthorizationRequest {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    #[serde(rename = "callerAddress")]
    pub caller_address: Address,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OracleAuthorizationResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub authorized: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionRequest {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub address: Address,
    pub calldata: Bytes,
    #[serde(rename = "chainId")]
    pub chain_id: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    /// Transaction receipt, holds the logs, the gas used, ...
    pub receipt: TransactionReceipt<AnyReceiptEnvelope<Log>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrivateDecryptionRequest {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    #[serde(rename = "ctHandleContractPairs")]
    pub ct_handle_contract_pairs: Vec<HandleContractPair>,
    #[serde(rename = "requestValidity")]
    pub request_validity: RequestValidity,
    #[serde(rename = "contractChainId")]
    pub contracts_chain_id: u64,
    #[serde(rename = "contractAddresses")]
    pub contract_addresses: Vec<Address>,
    #[serde(rename = "userAddress")]
    pub user_address: Address,
    #[serde(rename = "signature")]
    pub signature: Bytes,
    #[serde(rename = "publicKey")]
    pub public_key: Bytes,
}

#[allow(clippy::from_over_into)]
impl Into<fhevm_relayer::core::event::UserDecryptRequest> for PrivateDecryptionRequest {
    fn into(self) -> fhevm_relayer::core::event::UserDecryptRequest {
        fhevm_relayer::core::event::UserDecryptRequest {
            ct_handle_contract_pairs: self.ct_handle_contract_pairs,
            request_validity: self.request_validity,
            contracts_chain_id: self.contracts_chain_id,
            contract_addresses: self.contract_addresses,
            user_address: self.user_address,
            signature: self.signature,
            public_key: self.public_key,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrivateDecryptionResponse {
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub responses: Vec<UserDecryptResponsePayloadJson>,
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

        impl Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({})", self.event_name(), self.request_id())
            }
        }
    };
}

// TODO: document these values
// TODO: figure out if there is a way to register an enum directly instead of these values
// TODO: Maybe add a check to make sure that there is no value conflict
impl_event!(BlockchainEvent, 1);
impl_event!(HTTPZGatewayEvent, 2);
impl_event!(DebugMessage, 3);

impl_event!(TransactionRequest, 4);
impl_event!(TransactionResponse, 5);

impl_event!(OracleAuthorizationRequest, 6);
impl_event!(OracleAuthorizationResponse, 7);

impl_event!(HTTPInputRegistrationRequest, 8);
impl_event!(HTTPInputRegistrationResponse, 9);

impl_event!(PrivateDecryptionRequest, 10);
impl_event!(PrivateDecryptionResponse, 11);

impl_event!(HTTPPrivateDecryptionRequest, 12);
impl_event!(HTTPPrivateDecryptionResponse, 13);

impl_event!(UnrecoverableError, 14);

// TODO: define an error mitigation policy in case of SQS publishing failure
pub async fn send_message_to_sqs_queue<T>(
    check_queue_exists: bool,
    sqs_client: &aws_sdk_sqs::Client,
    queue_url: &String,
    message: &T,
) -> std::result::Result<aws_sdk_sqs::operation::send_message::SendMessageOutput, std::string::String>
where
    T: serde::Serialize,
{
    if check_queue_exists {
        // TODO: implement
    }

    let serialized_message = match serde_json::to_string(&message) {
        Ok(value) => value,
        Err(err) => {
            let err_msg = format!("Error serializing message to JSON: {:?}", err);
            return Err(err_msg);
        }
    };
    let publishing_response = match sqs_client
        .send_message()
        .queue_url(queue_url)
        .message_body(serialized_message)
        // If the queue is FIFO, you need to set .message_deduplication_id
        // and message_group_id or configure the queue for ContentBasedDeduplication.
        .send()
        .await
    {
        Err(error) => {
            let err_msg = format!("Error publishing: {:?}", error);
            return Err(err_msg);
        }
        Ok(response) => response,
    };
    Ok(publishing_response)
}

pub async fn send_message_to_sqs_queue_empty<T>(
    check_queue_exists: bool,
    sqs_client: &aws_sdk_sqs::Client,
    queue_url: &String,
    message: &T,
) where
    T: serde::Serialize + std::fmt::Display,
{
    match send_message_to_sqs_queue(check_queue_exists, sqs_client, queue_url, message).await {
        Ok(_) => {
            debug!("Successfuly sent {} to {:?}", &message, queue_url,)
        }
        Err(error) => {
            error!("Error sending {} to {:?}: {:?}", &message, queue_url, error)
        }
    }
}
