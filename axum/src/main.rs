use std::str::FromStr;
use std::time::Duration;
use axum::Json;
use axum::response::IntoResponse;
use tokio::time::sleep;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Data {
	payload: String
}

async fn simple_endpoint(Json(param): Json<Data>) -> impl IntoResponse {
	Json(Data {
		payload: format!("Hello, {}", param.payload)
	})
}

async fn timed_endpoint(Json(param): Json<Data>) -> impl IntoResponse {
	sleep(Duration::from_millis(20)).await;
	Json(Data {
		payload: format!("Hello, {}", param.payload)
	})
}

async fn bcrypt_endpoint(Json(param): Json<Data>) -> impl IntoResponse {
	Json(Data {
		payload: bcrypt::hash(&param.payload, 10).unwrap()
	})
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
	let router = axum::Router::new()
		.route("/test/simple", axum::routing::post(simple_endpoint))
		.route("/test/timed", axum::routing::post(timed_endpoint))
		.route("/test/bcrypt", axum::routing::post(bcrypt_endpoint));
	let address = "0.0.0.0";
	let port = 3000;
	log::info!("Listening on http://{}:{}/", address, port);
	axum::Server::bind(
		&std::net::SocketAddr::new(
			std::net::IpAddr::from_str(&address).unwrap(),
			port
		)
	).serve(router.into_make_service()).await?;
	Ok(())
}
