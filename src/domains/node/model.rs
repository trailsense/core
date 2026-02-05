use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Node {
    pub id: Uuid,
    pub status: NodeStatus,
    pub latitude: f64,
    pub longitude: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Copy, ToSchema, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum NodeStatus {
    Pending,
    Online,
    Unstable,
    Offline,
}

impl TryFrom<&str> for NodeStatus {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "pending" => Ok(Self::Pending),
            "online" => Ok(Self::Online),
            "unstable" => Ok(Self::Unstable),
            "offline" => Ok(Self::Offline),
            _ => Err(format!("Unknown node status: {value}")),
        }
    }
}

impl TryFrom<String> for NodeStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        NodeStatus::try_from(value.as_str())
    }
}
