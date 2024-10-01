use jli::utils::{ip::{header2ip, tor_check}, json::struct2json};

use actix_files::Files;
use actix_web::{get, post, web::Redirect, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TestStruct {
	ip: String,
	tor: bool,
	content: String,
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

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
			.service(main_page)
            .service(test_api)
			.service(Files::new("/", "public/").prefer_utf8(true))
    })
    .bind(("0.0.0.0", 3090))
    .expect("fn main crashed")
    .run().await
}
