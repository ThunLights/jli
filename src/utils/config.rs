use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::database::DBClient;

pub struct AppState {
    pub db: Arc<DBClient>,
}

#[derive(Serialize, Deserialize)]
pub struct CompressApiResponse {
    pub link: String,
    pub id: String,
}
#[derive(Serialize, Deserialize)]
pub struct DeCompressApiRequest {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct CompressApiRequest {
    pub link: String,
}

#[derive(Serialize, Deserialize)]
pub struct CompressApiBadResponse {
    pub content: String,
}
