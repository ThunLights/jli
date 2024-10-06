use actix_web::middleware::Logger;
use jli::utils::config::{AppState, EnvConfig};
use jli::utils::database::DBClient;
use jli::utils::api::{compress_api, decompress_api};

use actix_files::Files;
use actix_web::web::Redirect;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder, Result};
use dotenv::dotenv;

use std::fs;
use std::sync::Arc;

#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    match fs::read("public/favicon.ico") {
        Ok(contents) => HttpResponse::Ok()
            .content_type("image/x-icon")
            .body(contents),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/sitemap.xml")]
async fn sitemap() -> impl Responder {
	HttpResponse::Ok().content_type("application/xml").body(include_str!("../public/sitemap.xml"))
}

#[get("/robots.txt")]
async fn robots() -> impl Responder {
	HttpResponse::Ok().body(include_str!("../public/robots.txt"))
}

#[get("/")]
async fn main_page() -> impl Responder {
    Redirect::to("/site/index.html").permanent()
}

#[get("/{compress_id}")]
async fn compression_url(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let compress_id = path.into_inner();
    let db = data.get_ref().db.clone();

    if let Ok(link) = db.id2link(&compress_id).await {
        return Redirect::to(link).permanent();
    }

    Redirect::to("/site/index.html").permanent()
}

async fn not_found() -> Result<HttpResponse> {
    let contents = fs::read_to_string("public/404/index.html").unwrap_or_else(|_| {
        "<title>404 Page Not Found</title>\n<h1>404 Page Not Found</h1>".to_string()
    });

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(contents))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let server_config = match envy::from_env::<EnvConfig>() {
        Ok(val) => val,
        Err(_) => EnvConfig {
            port: 3000,
            id_size: 6,
            database_url: String::from("./db/sites.db"),
        },
    };
    let db = Arc::new(DBClient::new(&server_config.database_url, server_config.id_size).await);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::DefaultHeaders::new().add(("cache-control", "no-cache, no-store, must-revalidate")))
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .service(main_page)
            .service(favicon)
			.service(sitemap)
			.service(robots)
            .service(compression_url)
            .service(compress_api)
			.service(decompress_api)
            .service(Files::new("/site/", "public/").show_files_listing())
            .default_service(web::route().to(not_found))
    })
    .bind(("0.0.0.0", server_config.port))
    .expect("Failed to bind server")
    .run()
    .await
}
