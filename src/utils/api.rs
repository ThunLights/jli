use crate::utils::config::{CompressApiRequest, CompressApiResponse, CompressApiBadResponse, DeCompressApiRequest};
use crate::utils::config::AppState;
use actix_web::{http::StatusCode, post, web, HttpResponse, Responder};

#[post("/api/decompress")]
pub async fn decompress_api(data: web::Data<AppState>, body: web::Json<DeCompressApiRequest>) -> impl Responder {
	let db = data.get_ref().db.clone();
	match db.id2link(&body.id).await {
		Ok(link) => HttpResponse::Ok()
			.content_type("application/json")
			.status(StatusCode::OK)
			.json(CompressApiResponse {
				id: body.id.to_string(),
				link,
			}),
		Err(message) => HttpResponse::Ok()
			.content_type("application/json")
			.status(StatusCode::BAD_REQUEST)
			.json(CompressApiBadResponse {
				content: message.to_string(),
			})
	}
}

#[post("/api/compress")]
pub async fn compress_api(data: web::Data<AppState>, body: web::Json<CompressApiRequest>) -> impl Responder {
    let db = data.get_ref().db.clone();
    match db.link2id(&body.link).await {
        Ok(id) => HttpResponse::Ok()
			.content_type("application/json")
			.status(StatusCode::OK)
			.json(CompressApiResponse {
				link: body.link.to_string(),
				id,
			}),
        Err(message) => HttpResponse::Ok()
			.content_type("application/json")
			.status(StatusCode::BAD_REQUEST)
			.json(CompressApiBadResponse {
				content: message.to_string(),
			})
    }
}
