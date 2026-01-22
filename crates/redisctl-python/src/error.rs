//! Error handling for Python bindings
//!
//! Maps Rust error types from redis-cloud and redis-enterprise to Python exceptions.

use pyo3::exceptions::{PyConnectionError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::{PyErr, create_exception};

// Create a custom exception type for redisctl errors
create_exception!(redisctl, RedisCtlError, pyo3::exceptions::PyException);

/// Convert a CloudError to a PyErr
pub fn cloud_error_to_py(err: redis_cloud::CloudError) -> PyErr {
    match &err {
        redis_cloud::CloudError::ConnectionError(_) => PyConnectionError::new_err(err.to_string()),
        redis_cloud::CloudError::AuthenticationFailed { .. } => {
            RedisCtlError::new_err(format!("Authentication failed: {}", err))
        }
        redis_cloud::CloudError::NotFound { .. } => {
            PyValueError::new_err(format!("Resource not found: {}", err))
        }
        redis_cloud::CloudError::BadRequest { .. } => PyValueError::new_err(err.to_string()),
        redis_cloud::CloudError::Forbidden { .. } => {
            RedisCtlError::new_err(format!("Access forbidden: {}", err))
        }
        _ => PyRuntimeError::new_err(err.to_string()),
    }
}

/// Convert a RestError to a PyErr
pub fn enterprise_error_to_py(err: redis_enterprise::RestError) -> PyErr {
    match &err {
        redis_enterprise::RestError::ConnectionError(_) => {
            PyConnectionError::new_err(err.to_string())
        }
        redis_enterprise::RestError::AuthenticationFailed => {
            RedisCtlError::new_err("Authentication failed")
        }
        redis_enterprise::RestError::Unauthorized => RedisCtlError::new_err("Unauthorized access"),
        redis_enterprise::RestError::NotFound => PyValueError::new_err("Resource not found"),
        redis_enterprise::RestError::ValidationError(_) => PyValueError::new_err(err.to_string()),
        _ => PyRuntimeError::new_err(err.to_string()),
    }
}

/// Helper trait for converting Results to PyResult
pub trait IntoPyResult<T> {
    fn into_py_result(self) -> PyResult<T>;
}

impl<T> IntoPyResult<T> for Result<T, redis_cloud::CloudError> {
    fn into_py_result(self) -> PyResult<T> {
        self.map_err(cloud_error_to_py)
    }
}

impl<T> IntoPyResult<T> for Result<T, redis_enterprise::RestError> {
    fn into_py_result(self) -> PyResult<T> {
        self.map_err(enterprise_error_to_py)
    }
}
