use serde::Serialize;
use serde_json::Result;

pub async fn struct2json<T: Serialize>(obj: &T) -> Result<String> {
	serde_json::to_string(obj)
}
