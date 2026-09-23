use actix_web::HttpResponse;
use kms_connector_api::VersionResponse;

/// `GET /version`
pub async fn version() -> HttpResponse {
    HttpResponse::Ok().json(VersionResponse::default())
}
