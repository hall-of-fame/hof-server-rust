#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::fs::FileServer;

mod service;
mod utils;

use service::departments::{ get_departments, Department };
use service::multiple::{ get_multiple, MPSticker };
use service::popular::{ get_popular, PopSticker };

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Response<T> {
    data: T
}

#[get("/departments")]
fn departments() -> Json<Response<Vec<Department>>> {
    Json(Response {
        data: get_departments(vec![
            "产品", "视觉", "前端", "后端", "Android", "iOS", "运维", "安全",
        ])
    })
}

#[get("/multiple")]
fn multiple() -> Json<Response<Vec<MPSticker>>> {
    Json(Response {
        data: get_multiple()
    })
}

#[get("/popular")]
fn popular<'a>() -> Json<Response<Vec<PopSticker<'a>>>> {
    Json(Response {
        data: get_popular()
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![departments])
        .mount("/", routes![multiple])
        .mount("/", routes![popular])
        .mount("/static", FileServer::from("./static"))
}
