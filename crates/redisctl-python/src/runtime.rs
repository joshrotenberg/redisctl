//! Tokio runtime management for Python bindings
//!
//! Provides a shared Tokio runtime for all async operations, ensuring proper
//! lifecycle management between Python's asyncio and Rust's tokio.

use pyo3::prelude::*;
use std::sync::OnceLock;
use tokio::runtime::Runtime;

/// Global Tokio runtime instance
static RUNTIME: OnceLock<Runtime> = OnceLock::new();

/// Get or initialize the global Tokio runtime
///
/// This creates a multi-threaded runtime suitable for I/O-bound operations.
/// The runtime is lazily initialized on first use and persists for the
/// lifetime of the Python process.
pub fn get_runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("redisctl-tokio")
            .build()
            .expect("Failed to create Tokio runtime")
    })
}

/// Run a future to completion on the global runtime (blocking)
///
/// This is used for sync API variants that need to block until completion.
/// It properly releases the GIL during the blocking call to allow other
/// Python threads to run.
pub fn block_on<F, T>(py: Python<'_>, future: F) -> T
where
    F: std::future::Future<Output = T> + Send,
    T: Send,
{
    // Release the GIL while we block on the future
    py.allow_threads(|| get_runtime().block_on(future))
}

/// Convert a Rust future into a Python awaitable
///
/// This bridges Rust async to Python async, allowing Python code to
/// `await` on Rust futures.
pub fn future_into_py<'py, F>(py: Python<'py>, future: F) -> PyResult<Bound<'py, PyAny>>
where
    F: std::future::Future<Output = PyResult<PyObject>> + Send + 'static,
{
    pyo3_async_runtimes::tokio::future_into_py(py, future)
}
