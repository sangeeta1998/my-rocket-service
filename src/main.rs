#[macro_use]
extern crate rocket;

use reqwest::blocking::Client;
use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket::State;

// Rocket state to hold the Reqwest client
struct ReqwestClient(Client);

#[get("/fetch")]
fn fetch_data(reqwest_client: &State<ReqwestClient>) -> Result<String, Status> {
    let response = reqwest_client
        .0
        .get("http://0.0.0.0:8008/") // from hello-http-wasi
        .send()
        .map_err(|_| Status::InternalServerError)?;

    if response.status().is_success() {
        let data = response.text().map_err(|_| Status::InternalServerError)?;
        Ok(data)
    } else {
        Err(Status::InternalServerError)
    }
}

#[launch]
fn rocket() -> _ {
    let reqwest_client = Client::new();
    rocket::build()
        .manage(ReqwestClient(reqwest_client))
        .mount("/", routes![fetch_data])
}
