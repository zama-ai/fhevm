//! Admin configuration management module.
//!
//! This module provides a centralized registry pattern for admin-configurable parameters.
//! It eliminates match statement duplication and provides consistent validation and logging.

mod config_param;
pub mod handlers;
mod registry;

pub use config_param::{ConfigError, ConfigParam, ConfigValue, ParamConstraints};
pub use handlers::{get_config, update_config, UpdateConfigRequest};
pub use registry::AdminConfigRegistry;
