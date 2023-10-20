use std::time::Duration;
use tokio::time::sleep;
use rocket::serde::json::Json;

#[macro_use] extern crate rocket;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Data {
	payload: String
}

#[post("/test/simple", data = "<param>")]
async fn simple_endpoint(param: Json<Data>) -> Json<Data> {
	Json(Data {
		payload: format!("Hello, {}", param.into_inner().payload)
	})
}

#[post("/test/timed", data = "<param>")]
async fn timed_endpoint(param: Json<Data>) -> Json<Data> {
	sleep(Duration::from_millis(20)).await;
	Json(Data {
		payload: format!("Hello, {}", param.into_inner().payload)
	})
}

#[post("/test/bcrypt", data = "<param>")]
async fn bcrypt_endpoint(param: Json<Data>) -> Json<Data> {
	Json(Data {
		payload: bcrypt::hash(&param.into_inner().payload, 10).unwrap()
	})
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.configure(rocket::Config::figment()
			.merge(("address", "0.0.0.0"))
			.merge(("port", 3000))
		)
		.mount("/", routes![
			simple_endpoint,
			timed_endpoint,
			bcrypt_endpoint
		])
}
