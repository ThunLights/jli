
use jli::utils::{ip::{header2ip, tor_check}, json::struct2json};

use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TestStruct {
	ip: String,
	tor: bool,
	content: String,
}

#[get("/")]
async fn main_page(req: HttpRequest) -> impl Responder {
	let ip = header2ip(&req).await;
	let tor = tor_check(&req).await;
	let contents = TestStruct {
		ip,
		tor,
		content: String::from("これはテストです。"),
	};

    HttpResponse::Ok().content_type("application/json").body(struct2json(&contents).await.unwrap())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(main_page)
    })
    .bind(("0.0.0.0", 3090))
    .expect("fn main crashed")
    .run().await
}
