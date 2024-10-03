use jli::utils::database::DBClient;

use actix_files::Files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use actix_web::http::StatusCode;
use actix_web::web::Redirect;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;

use std::fs;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
struct CompressApiResponse {
	pub link: String,
	pub id: String,
}

#[derive(Serialize, Deserialize)]
struct CompressApiRequest {
	pub link: String,
}

#[derive(Serialize, Deserialize)]
struct CompressApiBadResponse {
	pub content: String,
}

#[derive(Serialize, Deserialize)]
struct EnvConfig {
	port: u16,
	id_size: u16,
	database_url: String,
}

#[post("/api/compress")]
async fn compress_api(data: web::Data<Arc<DBClient>>, body: web::Json<CompressApiRequest>) -> impl Responder {
	let db = data.get_ref();
	match db.get_link(&body.link).await {
		Ok(id) => {
			let response = CompressApiResponse {
				link: body.link.to_string(),
				id,
			};
			return HttpResponse::Ok().content_type("application/json").status(StatusCode::OK).json(response);
		},
		Err(message) => {
			return HttpResponse::Ok().content_type("application/json").status(StatusCode::BAD_REQUEST).json(CompressApiBadResponse { content: message.to_string() });
		},
	}
}

#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    match fs::read("public/favicon.ico") {
        Ok(contents) => HttpResponse::Ok()
            .content_type("image/x-icon")
            .body(contents),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/")]
async fn main_page() -> impl Responder {
	Redirect::to("/index.html").permanent()
}

async fn not_found() -> Result<HttpResponse> {
    let contents = fs::read_to_string("public/404/index.html")
        .unwrap_or_else(|_| "<title>404 Page Not Found</title>\n<h1>404 Page Not Found</h1>".to_string());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(contents))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();
	let server_config = match envy::from_env::<EnvConfig>() {
		Ok(val) => val,
		Err(_) => EnvConfig { port: 3000, id_size: 6, database_url: String::from("./db/sites.db") }
	};
	let db = Arc::new(DBClient::new(&server_config.database_url, server_config.id_size).await);

    HttpServer::new(move || {
        App::new()
			.app_data(db.clone())
			.service(main_page)
			.service(favicon)
            .service(compress_api)
			.service(Files::new("/", "public/").show_files_listing())
			.default_service(web::route().to(not_found))
    })
    .bind(("0.0.0.0", server_config.port))
    .expect("Failed to bind server")
    .run().await
}
