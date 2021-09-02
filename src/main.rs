#[macro_use]
extern crate rocket;

use regex::Regex;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use std::fs;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Sticker {
    desc: String,
    url: String
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct MPSticker {
    author: String,
    desc: String,
    url: String,
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Student {
    name: String,
    avatar: String,
    stickers: Vec<Sticker>
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Grade {
    name: String,
    students: Vec<Student>
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Department {
    name: String,
    grades: Vec<Grade>
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Response<T> {
    data: T
}

#[get("/departments")]
fn departments() -> Json<Response<Vec<Department>>> {
    let depts = vec![
        "PM", "Design", "Frontend", "Backend", "Android", "iOS", "SRE", "0xfa",
    ]
    .iter()
    .map(|d| d.to_string())
    .collect();
    Json(Response {
        data: get_department(depts)
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
}

fn get_department(department_names: Vec<String>) -> Vec<Department> {
    let mut total_data: Vec<Department> = Vec::new();

    for department_name in department_names {
        let department_data: Department = Department {
            name: department_name.clone(),
            grades: get_grades(department_name)
        };
        total_data.push(department_data);
    }

    total_data
}

fn get_grades(department_name: String) -> Vec<Grade> {
    let grade_dirs = fs::read_dir(format!("./src/images/{}", department_name))
        .unwrap()
        .map(|res| res.unwrap());
    let mut grades_data = Vec::<Grade>::new();
    for grade_dir in grade_dirs {
        let grade_name = grade_dir.file_name().into_string().unwrap();
        let grade_data: Grade = Grade {
            name: grade_name.clone(),
            students: get_students(grade_dir, department_name.clone())
        };
        grades_data.push(grade_data);
    }
    grades_data
}

fn get_students(grade_dir: fs::DirEntry, department_name: String) -> Vec<Student> {
    let grade_name = grade_dir.file_name().into_string().unwrap();
    let student_dirs = fs::read_dir(grade_dir.path())
        .unwrap()
        .map(|res| res.unwrap());
    let mut students_data =  Vec::<Student>::new();
    for student_dir in student_dirs {
        let student_name = student_dir.file_name().into_string().unwrap();
        let relative_path = format!("/static/{}/{}/{}", department_name, grade_name, student_name);
        let stickers_data = get_stickers(student_dir, relative_path);
        let student_data = Student {
            name: student_name,
            avatar: String::from("114514"),
            stickers: stickers_data
        };
        students_data.push(student_data) ;
    }
    students_data
}

fn get_stickers(dir: fs::DirEntry, relative_path: String) -> Vec<Sticker> {
    let items = fs::read_dir(dir.path()).unwrap().map(|res| res.unwrap());
    let mut stickers = Vec::<Sticker>::new();
    for item in items {
        // the item is perhaps a directory or an image file.
        let itemname = item.file_name().into_string().unwrap();
        if item.file_type().unwrap().is_dir() {
            // if the item is a directory:
            let files = fs::read_dir(item.path()).unwrap().map(|res| res.unwrap());
            let dirname = itemname.replace(" ", "%20");
            for file in files {
                let filename = file.file_name().into_string().unwrap();
                stickers.push(Sticker {
                    desc: trim_extention_name(filename.clone()),
                    url: format!("{}/{}/{}", relative_path, dirname, filename),
                });
            }
        } else {
            // if the item is just an image file:
            let filename = itemname.replace(" ", "%20");
            stickers.push(Sticker {
                desc: trim_extention_name(itemname),
                url: format!("{}/{}", relative_path, filename),
            });
        }
    }
    stickers
}

fn trim_extention_name(filename: String) -> String {
    let reg = Regex::new(r"(.*)\..*$").unwrap();
    let caps = reg.captures(filename.as_str()).unwrap();
    caps.get(1).unwrap().as_str().to_string()
}

fn get_multiple() -> Vec<MPSticker> {
    let images = fs::read_dir("./src/images/Multiple").unwrap().map(|res| res.unwrap());
    let mut stickers = Vec::<MPSticker>::new();
    for image in images {
        let image_name = image.file_name().into_string().unwrap();
        stickers.push(resolve_mpsticker_filename(image_name));
    }
    stickers
}

fn resolve_mpsticker_filename(filename: String) -> MPSticker {
    let reg = Regex::new(r"(.*?)-(.*)\..*$").unwrap();
    let caps = reg.captures(filename.as_str()).unwrap();
    let author = caps.get(1).unwrap().as_str().to_string();
    let desc = caps.get(2).unwrap().as_str().to_string();
    let url = format!("./src/images/multiple/{}", filename.replace(" ", "%20"));
    MPSticker {
        author: author,
        desc: desc,
        url: url,
    }
}
