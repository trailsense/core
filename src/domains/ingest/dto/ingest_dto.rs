use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct IngestDto {
    pub node_id: Uuid,
    pub count: u32,
    #[schema(maximum = 31_536_000)]
    #[validate(range(max = 31_536_000))] // 1 year
    pub age_in_seconds: u32,
}
