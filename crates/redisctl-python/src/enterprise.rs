//! Python bindings for Redis Enterprise API client
//!
//! Provides both async and sync APIs for managing Redis Enterprise clusters.

use crate::cloud::{json_to_py, py_to_json};
use crate::error::IntoPyResult;
use crate::runtime::{block_on, future_into_py};
use pyo3::prelude::*;
use redis_enterprise::{BdbHandler, ClusterHandler, EnterpriseClient, NodeHandler, UserHandler};
use std::sync::Arc;
use std::time::Duration;

/// Redis Enterprise API client
///
/// Provides access to Redis Enterprise cluster management APIs for databases,
/// nodes, users, and cluster operations.
///
/// # Examples
///
/// ```python
/// from redisctl import EnterpriseClient
///
/// # Create client
/// client = EnterpriseClient(
///     base_url="https://cluster:9443",
///     username="admin@redis.local",
///     password="secret",
///     insecure=True  # For self-signed certs in dev
/// )
///
/// # Async usage
/// async def main():
///     dbs = await client.databases()
///     for db in dbs:
///         print(db["name"], db["uid"])
///
/// # Sync usage
/// dbs = client.databases_sync()
/// ```
#[pyclass(name = "EnterpriseClient")]
pub struct PyEnterpriseClient {
    client: Arc<EnterpriseClient>,
}

#[pymethods]
impl PyEnterpriseClient {
    /// Create a new Redis Enterprise client
    ///
    /// Args:
    ///     base_url: Cluster URL (e.g., "https://cluster:9443")
    ///     username: Username for authentication
    ///     password: Password for authentication
    ///     insecure: Allow insecure TLS (self-signed certs), default False
    ///     timeout_secs: Optional timeout in seconds (default: 30)
    ///
    /// Returns:
    ///     EnterpriseClient instance
    ///
    /// Raises:
    ///     RedisCtlError: If client creation fails
    #[new]
    #[pyo3(signature = (base_url, username, password, insecure=false, timeout_secs=None))]
    fn new(
        base_url: String,
        username: String,
        password: String,
        insecure: bool,
        timeout_secs: Option<u64>,
    ) -> PyResult<Self> {
        let mut builder = EnterpriseClient::builder()
            .base_url(base_url)
            .username(username)
            .password(password)
            .insecure(insecure);

        if let Some(secs) = timeout_secs {
            builder = builder.timeout(Duration::from_secs(secs));
        }

        let client = builder.build().into_py_result()?;
        Ok(Self {
            client: Arc::new(client),
        })
    }

    /// Create client from environment variables (sync)
    ///
    /// Reads configuration from:
    /// - REDIS_ENTERPRISE_URL: Base URL (default: https://localhost:9443)
    /// - REDIS_ENTERPRISE_USER: Username (default: admin@redis.local)
    /// - REDIS_ENTERPRISE_PASSWORD: Password (required)
    /// - REDIS_ENTERPRISE_INSECURE: Set to "true" to skip SSL verification
    ///
    /// Returns:
    ///     EnterpriseClient instance
    #[staticmethod]
    fn from_env() -> PyResult<Self> {
        let client = EnterpriseClient::from_env().into_py_result()?;
        Ok(Self {
            client: Arc::new(client),
        })
    }

    // -------------------------------------------------------------------------
    // Cluster API
    // -------------------------------------------------------------------------

    /// Get cluster information (async)
    ///
    /// Returns:
    ///     Cluster info dictionary
    fn cluster_info<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let handler = ClusterHandler::new((*client).clone());
            let info = handler.info().await.into_py_result()?;
            let json = serde_json::to_value(&info)
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            Python::attach(|py| Ok(json_to_py(py, json)))
        })
    }

    /// Get cluster information (sync/blocking)
    fn cluster_info_sync(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let client = self.client.clone();
        let result = block_on(py, async move {
            let handler = ClusterHandler::new((*client).clone());
            handler.info().await.into_py_result()
        })?;
        let json = serde_json::to_value(&result)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(json_to_py(py, json))
    }

    /// Get cluster statistics (async)
    fn cluster_stats<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let handler = ClusterHandler::new((*client).clone());
            let stats = handler.stats().await.into_py_result()?;
            Python::attach(|py| Ok(json_to_py(py, stats)))
        })
    }

    /// Get cluster statistics (sync/blocking)
    fn cluster_stats_sync(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let client = self.client.clone();
        let result = block_on(py, async move {
            let handler = ClusterHandler::new((*client).clone());
            handler.stats().await.into_py_result()
        })?;
        Ok(json_to_py(py, result))
    }

    /// Get license information (async)
    fn license<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let handler = ClusterHandler::new((*client).clone());
            let license = handler.license().await.into_py_result()?;
            let json = serde_json::to_value(&license)
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            Python::attach(|py| Ok(json_to_py(py, json)))
        })
    }

    /// Get license information (sync/blocking)
    fn license_sync(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let client = self.client.clone();
        let result = block_on(py, async move {
            let handler = ClusterHandler::new((*client).clone());
            handler.license().await.into_py_result()
        })?;
        let json = serde_json::to_value(&result)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(json_to_py(py, json))
    }

    // -------------------------------------------------------------------------
    // Databases API
    // -------------------------------------------------------------------------

    /// List all databases (async)
    ///
    /// Returns:
    ///     List of database dictionaries
    fn databases<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let handler = BdbHandler::new((*client).clone());
            let dbs = handler.list().await.into_py_result()?;
            let json = serde_json::to_value(&dbs)
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            Python::attach(|py| Ok(json_to_py(py, json)))
        })
    }

    /// List all databases (sync/blocking)
    fn databases_sync(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let client = self.client.clone();
        let result = block_on(py, async move {
            let handler = BdbHandler::new((*client).clone());
            handler.list().await.into_py_result()
        })?;
        let json = serde_json::to_value(&result)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(json_to_py(py, json))
    }

    /// Get a specific database by ID (async)
    ///
    /// Args:
    ///     uid: Database UID
    ///
    /// Returns:
    ///     Database dictionary
    fn database<'py>(&self, py: Python<'py>, uid: u32) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let handler = BdbHandler::new((*client).clone());
            let db = handler.get(uid).await.into_py_result()?;
            let json = serde_json::to_value(&db)
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            Python::attach(|py| Ok(json_to_py(py, json)))
        })
    }

    /// Get a specific database by ID (sync/blocking)
    fn database_sync(&self, py: Python<'_>, uid: u32) -> PyResult<Py<PyAny>> {
        let client = self.client.clone();
        let result = block_on(py, async move {
            let handler = BdbHandler::new((*client).clone());
            handler.get(uid).await.into_py_result()
        })?;
        let json = serde_json::to_value(&result)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(json_to_py(py, json))
    }

    /// Get database statistics (async)
    fn database_stats<'py>(&self, py: Python<'py>, uid: u32) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let handler = BdbHandler::new((*client).clone());
            let stats = handler.stats(uid).await.into_py_result()?;
            // stats is already a serde_json::Value
            Python::attach(|py| Ok(json_to_py(py, stats)))
        })
    }

    /// Get database statistics (sync/blocking)
    fn database_stats_sync(&self, py: Python<'_>, uid: u32) -> PyResult<Py<PyAny>> {
        let client = self.client.clone();
        let result = block_on(py, async move {
            let handler = BdbHandler::new((*client).clone());
            handler.stats(uid).await.into_py_result()
        })?;
        Ok(json_to_py(py, result))
    }

    // -------------------------------------------------------------------------
    // Nodes API
    // -------------------------------------------------------------------------

    /// List all nodes (async)
    ///
    /// Returns:
    ///     List of node dictionaries
    fn nodes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let handler = NodeHandler::new((*client).clone());
            let nodes = handler.list().await.into_py_result()?;
            let json = serde_json::to_value(&nodes)
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            Python::attach(|py| Ok(json_to_py(py, json)))
        })
    }

    /// List all nodes (sync/blocking)
    fn nodes_sync(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let client = self.client.clone();
        let result = block_on(py, async move {
            let handler = NodeHandler::new((*client).clone());
            handler.list().await.into_py_result()
        })?;
        let json = serde_json::to_value(&result)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(json_to_py(py, json))
    }

    /// Get a specific node by ID (async)
    fn node<'py>(&self, py: Python<'py>, uid: u32) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let handler = NodeHandler::new((*client).clone());
            let node = handler.get(uid).await.into_py_result()?;
            let json = serde_json::to_value(&node)
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            Python::attach(|py| Ok(json_to_py(py, json)))
        })
    }

    /// Get a specific node by ID (sync/blocking)
    fn node_sync(&self, py: Python<'_>, uid: u32) -> PyResult<Py<PyAny>> {
        let client = self.client.clone();
        let result = block_on(py, async move {
            let handler = NodeHandler::new((*client).clone());
            handler.get(uid).await.into_py_result()
        })?;
        let json = serde_json::to_value(&result)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(json_to_py(py, json))
    }

    /// Get node statistics (async)
    fn node_stats<'py>(&self, py: Python<'py>, uid: u32) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let handler = NodeHandler::new((*client).clone());
            let stats = handler.stats(uid).await.into_py_result()?;
            let json = serde_json::to_value(&stats)
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            Python::attach(|py| Ok(json_to_py(py, json)))
        })
    }

    /// Get node statistics (sync/blocking)
    fn node_stats_sync(&self, py: Python<'_>, uid: u32) -> PyResult<Py<PyAny>> {
        let client = self.client.clone();
        let result = block_on(py, async move {
            let handler = NodeHandler::new((*client).clone());
            handler.stats(uid).await.into_py_result()
        })?;
        let json = serde_json::to_value(&result)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(json_to_py(py, json))
    }

    // -------------------------------------------------------------------------
    // Users API
    // -------------------------------------------------------------------------

    /// List all users (async)
    ///
    /// Returns:
    ///     List of user dictionaries
    fn users<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let handler = UserHandler::new((*client).clone());
            let users = handler.list().await.into_py_result()?;
            let json = serde_json::to_value(&users)
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            Python::attach(|py| Ok(json_to_py(py, json)))
        })
    }

    /// List all users (sync/blocking)
    fn users_sync(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let client = self.client.clone();
        let result = block_on(py, async move {
            let handler = UserHandler::new((*client).clone());
            handler.list().await.into_py_result()
        })?;
        let json = serde_json::to_value(&result)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(json_to_py(py, json))
    }

    /// Get a specific user by ID (async)
    fn user<'py>(&self, py: Python<'py>, uid: u32) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let handler = UserHandler::new((*client).clone());
            let user = handler.get(uid).await.into_py_result()?;
            let json = serde_json::to_value(&user)
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            Python::attach(|py| Ok(json_to_py(py, json)))
        })
    }

    /// Get a specific user by ID (sync/blocking)
    fn user_sync(&self, py: Python<'_>, uid: u32) -> PyResult<Py<PyAny>> {
        let client = self.client.clone();
        let result = block_on(py, async move {
            let handler = UserHandler::new((*client).clone());
            handler.get(uid).await.into_py_result()
        })?;
        let json = serde_json::to_value(&result)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(json_to_py(py, json))
    }

    // -------------------------------------------------------------------------
    // Raw API access
    // -------------------------------------------------------------------------

    /// Execute a raw GET request (async)
    ///
    /// Args:
    ///     path: API path (e.g., "/v1/cluster")
    ///
    /// Returns:
    ///     Response as dictionary
    fn get<'py>(&self, py: Python<'py>, path: String) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let result = client.get_raw(&path).await.into_py_result()?;
            Python::attach(|py| Ok(json_to_py(py, result)))
        })
    }

    /// Execute a raw GET request (sync/blocking)
    fn get_sync(&self, py: Python<'_>, path: String) -> PyResult<Py<PyAny>> {
        let client = self.client.clone();
        let result = block_on(
            py,
            async move { client.get_raw(&path).await.into_py_result() },
        )?;
        Ok(json_to_py(py, result))
    }

    /// Execute a raw POST request (async)
    fn post<'py>(
        &self,
        py: Python<'py>,
        path: String,
        body: Py<PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let body_json = py_to_json(py, body)?;
        let client = self.client.clone();
        future_into_py(py, async move {
            let result = client.post_raw(&path, body_json).await.into_py_result()?;
            Python::attach(|py| Ok(json_to_py(py, result)))
        })
    }

    /// Execute a raw POST request (sync/blocking)
    fn post_sync(&self, py: Python<'_>, path: String, body: Py<PyAny>) -> PyResult<Py<PyAny>> {
        let client = self.client.clone();
        let body_json = py_to_json(py, body)?;
        let result = block_on(py, async move {
            client.post_raw(&path, body_json).await.into_py_result()
        })?;
        Ok(json_to_py(py, result))
    }

    /// Execute a raw PUT request (async)
    fn put<'py>(
        &self,
        py: Python<'py>,
        path: String,
        body: Py<PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let body_json = py_to_json(py, body)?;
        let client = self.client.clone();
        future_into_py(py, async move {
            let result = client.put_raw(&path, body_json).await.into_py_result()?;
            Python::attach(|py| Ok(json_to_py(py, result)))
        })
    }

    /// Execute a raw PUT request (sync/blocking)
    fn put_sync(&self, py: Python<'_>, path: String, body: Py<PyAny>) -> PyResult<Py<PyAny>> {
        let client = self.client.clone();
        let body_json = py_to_json(py, body)?;
        let result = block_on(py, async move {
            client.put_raw(&path, body_json).await.into_py_result()
        })?;
        Ok(json_to_py(py, result))
    }

    /// Execute a raw DELETE request (async)
    fn delete<'py>(&self, py: Python<'py>, path: String) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let result = client.delete_raw(&path).await.into_py_result()?;
            Python::attach(|py| Ok(json_to_py(py, result)))
        })
    }

    /// Execute a raw DELETE request (sync/blocking)
    fn delete_sync(&self, py: Python<'_>, path: String) -> PyResult<Py<PyAny>> {
        let client = self.client.clone();
        let result = block_on(py, async move {
            client.delete_raw(&path).await.into_py_result()
        })?;
        Ok(json_to_py(py, result))
    }
}
