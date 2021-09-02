#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::fs::FileServer;

mod service;

use service::departments::{ get_departments, Department };
use service::multiple::{ get_multiple, MPSticker };

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Response<T> {
    data: T
}

#[get("/departments")]
fn departments() -> Json<Response<Vec<Department>>> {
    Json(Response {
        data: get_departments(vec![
            "PM", "Design", "Frontend", "Backend", "Android", "iOS", "SRE", "0xfa",
        ])
    })
}

#[get("/multiple")]
fn multiple() -> Json<Response<Vec<MPSticker>>> {
    Json(Response {
        data: get_multiple()
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![departments])
        .mount("/", routes![multiple])
        .mount("/static", FileServer::from("./static"))
}
