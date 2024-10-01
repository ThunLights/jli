use jli::utils::ip::{header2ip, tor_check};
use jli::utils::json::struct2json;

use actix_files::Files;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use actix_web::http::StatusCode;
use actix_web::web::Redirect;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;

use std::path::Path;

#[derive(Serialize, Deserialize)]
struct TestStruct {
	ip: String,
	tor: bool,
	content: String,
}

#[derive(Serialize, Deserialize)]
struct EnvConfig {
	port: u16,
}

#[post("/api/test")]
async fn test_api(req: HttpRequest) -> impl Responder {
	let ip = header2ip(&req).await;
	let tor = tor_check(&req).await;
	let contents = TestStruct {
		ip,
		tor,
		content: String::from("これはテストです。"),
	};

    HttpResponse::Ok().content_type("application/json").body(struct2json(&contents).await.unwrap())
}

#[get("/")]
async fn main_page() -> impl Responder {
	Redirect::to("/index.html").permanent()
}

async fn not_found() -> Result<HttpResponse> {
	if Path::new("../public/404/index.html").is_file() {
		return Ok(HttpResponse::build(StatusCode::OK)
			.content_type("text/html; charset=utf-8")
			.body(include_str!("../public/404/index.html")));
	}

	Ok(HttpResponse::build(StatusCode::OK)
		.content_type("text/html; charset=utf-8")
		.body("<h1>Error 404</h1>"))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();
	let server_config = match envy::from_env::<EnvConfig>() {
		Ok(val) => val,
		Err(_) => EnvConfig { port: 3000 }
	};

	println!("Server is running on port: {}", server_config.port);

    HttpServer::new(|| {
        App::new()
			.service(main_page)
            .service(test_api)
			.service(Files::new("/", "public/").prefer_utf8(true))
			.default_service(web::route().to(not_found))
    })
    .bind(("0.0.0.0", server_config.port))
    .expect("Failed to bind server")
    .run().await
}
