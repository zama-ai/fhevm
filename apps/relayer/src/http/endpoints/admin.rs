//! Admin endpoint handlers.
//!
//! This module re-exports from the new `src/http/admin/` module for backward compatibility.
//! New code should import directly from `crate::http::admin`.

// Re-export the new admin module types and handlers
pub use crate::http::admin::{
    get_config, update_config, AdminConfigRegistry, ConfigParam, ConfigValue, ParamConstraints,
    UpdateConfigRequest,
};

// Legacy type aliases for backward compatibility
pub use crate::http::admin::handlers::AdminErrorResponse as ErrorResponse;
pub use crate::http::admin::handlers::UpdateConfigResponse;
