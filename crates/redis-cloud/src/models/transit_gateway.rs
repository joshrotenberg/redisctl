//! Transit Gateway models

use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitGatewayAttachment {
    pub id: Option<String>,
    pub tgw_id: Option<String>,
    pub region: Option<String>,
    pub status: Option<String>,
    pub cidrs: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitGatewayInvitation {
    pub id: String,
    pub status: Option<String>,
    pub region: Option<String>,
    pub created_at: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateTransitGatewayAttachmentRequest {
    #[builder(setter(into))]
    pub tgw_id: String,
    #[builder(default, setter(into, strip_option))]
    pub aws_account_id: Option<String>,
    pub cidrs: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_attachment() {
        let raw = serde_json::json!({
            "id": "att-1",
            "tgw_id": "tgw-123",
            "cidrs": ["10.0.0.0/16"],
            "status": "available"
        });
        let a: TransitGatewayAttachment = serde_json::from_value(raw).unwrap();
        assert_eq!(a.id.as_deref(), Some("att-1"));
        assert_eq!(a.status.as_deref(), Some("available"));
    }
}

