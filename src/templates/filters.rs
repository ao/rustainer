//! Custom filters for Askama templates.

use crate::models::service::ServiceType;

/// Convert a ServiceType enum to a string.
pub fn as_str(s: &ServiceType) -> ::askama::Result<String> {
    Ok(s.to_string())
}

/// Convert an Option<String> to a string.
pub fn option_to_string(s: &Option<String>) -> ::askama::Result<String> {
    match s {
        Some(val) => Ok(val.clone()),
        None => Ok(String::new()),
    }
}