//! Python bindings for Redis Cloud and Enterprise management APIs
//!
//! This crate provides Python bindings via PyO3 for the `redis-cloud` and `redis-enterprise`
//! Rust crates, enabling Python developers to manage Redis infrastructure programmatically.
//!
//! ## Features
//!
//! - **Async Support**: Native async/await support via `pyo3-async-runtimes`
//! - **Sync API**: Blocking variants for simple scripts
//! - **Type Safety**: Python type hints via stub files
//! - **Error Handling**: Rust errors mapped to Python exceptions
//!
//! ## Quick Start
//!
//! ```python
//! from redisctl import CloudClient, EnterpriseClient
//!
//! # Async usage
//! async def main():
//!     cloud = CloudClient(api_key="...", api_secret="...")
//!     subs = await cloud.subscriptions()
//!     print(subs)
//!
//! # Sync usage
//! cloud = CloudClient(api_key="...", api_secret="...")
//! subs = cloud.subscriptions_sync()
//! ```

use pyo3::prelude::*;

mod cloud;
mod enterprise;
mod error;
mod runtime;

use cloud::PyCloudClient;
use enterprise::PyEnterpriseClient;
use error::RedisCtlError;

/// Python module for Redis Cloud and Enterprise management
#[pymodule]
fn redisctl(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register exception type
    m.add("RedisCtlError", m.py().get_type::<RedisCtlError>())?;

    // Register Cloud client
    m.add_class::<PyCloudClient>()?;

    // Register Enterprise client
    m.add_class::<PyEnterpriseClient>()?;

    // Add version info
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
