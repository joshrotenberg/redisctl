//! Private Service Connect models

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PscService {
    pub id: String,
    pub name: Option<String>,
    pub status: Option<String>,
    pub provider: Option<String>,
    pub region: Option<String>,
    pub endpoints: Option<Vec<PscEndpoint>>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PscEndpoint {
    pub id: String,
    pub status: Option<String>,
    pub connection: Option<Value>,
    #[serde(flatten)]
    pub extra: Value,
}

/// Creation/deletion scripts payload wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PscScripts {
    #[serde(flatten)]
    pub scripts: Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_psc_service() {
        let raw = serde_json::json!({
            "id": "psc-1",
            "name": "svc",
            "status": "active",
            "region": "us-east-1",
            "endpoints": [
                {"id": "ep-1", "status": "ready"}
            ]
        });
        let s: PscService = serde_json::from_value(raw).unwrap();
        assert_eq!(s.id, "psc-1");
        assert_eq!(s.endpoints.as_ref().unwrap()[0].id, "ep-1");
    }
}

