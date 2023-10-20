use std::time::Duration;
use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Json;
use tokio::time::sleep;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Data {
	payload: String
}

#[post("/test/simple")]
async fn simple_endpoint(Json(param): Json<Data>) -> impl Responder {
	HttpResponse::Ok().json(Json(Data {
		payload: format!("Hello, {}", param.payload)
	}))
}

#[post("/test/timed")]
async fn timed_endpoint(Json(param): Json<Data>) -> impl Responder {
	sleep(Duration::from_millis(20)).await;
    HttpResponse::Ok().json(Json(Data {
		payload: format!("Hello, {}", param.payload)
	}))
}

#[post("/test/bcrypt")]
async fn bcrypt_endpoint(Json(param): Json<Data>) -> impl Responder {
	HttpResponse::Ok().json(Json(Data {
		payload: bcrypt::hash(&param.payload, 10).unwrap()
	}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
	let address = "0.0.0.0";
	let port = 3000;
	log::info!("Listening on http://{}:{}/", address, port);
    HttpServer::new(|| {
        App::new()
            .service(simple_endpoint)
            .service(timed_endpoint)
			.service(bcrypt_endpoint)
    })
        .bind((address, port))?
        .run()
        .await
}
