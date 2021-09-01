use std::collections::HashMap;
use std::fs;
use regex::Regex;

type Department = HashMap<String, Grade>;
type Grade = HashMap<String, PersonData>;
type PersonData = HashMap<String, String>;

fn main() {
    let depts = vec![
        "PM", "Design", "Frontend", "Backend", "Android", "iOS", "SRE", "0xfa",
    ]
    .iter()
    .map(|d| d.to_string())
    .collect();
    let total_data: HashMap<String, Department> = get_department_images(depts);
    println!("{:?}", total_data);
}

fn get_department_images(departments: Vec<String>) -> HashMap<String, Department> {
    let mut total_data: HashMap<String, Department> = HashMap::new();

    for dept in departments {
        let grade_dirs = fs::read_dir(format!("./src/images/{}", dept))
            .unwrap()
            .map(|res| res.unwrap());
        let mut dept_data: Department = HashMap::new();
        for grade_dir in grade_dirs {
            let grade = grade_dir.file_name().into_string().unwrap();
            let person_dirs = fs::read_dir(grade_dir.path())
                .unwrap()
                .map(|res| res.unwrap());
            let mut grade_data: Grade = HashMap::new();
            for person_dir in person_dirs {
                let person_name = person_dir.file_name().into_string().unwrap();
                let relative_path = format!("/static/{}/{}/{}", dept, grade, person_name);
                let person_data = get_person_images(person_dir, relative_path);
                grade_data.insert(person_name, person_data);
            }
            dept_data.insert(grade, grade_data);
        }
        total_data.insert(dept.to_string(), dept_data);
    }

    total_data
}

fn get_person_images(dir: fs::DirEntry, relative_path: String) -> PersonData {
    let items = fs::read_dir(dir.path()).unwrap().map(|res| res.unwrap());
    let mut images: PersonData = HashMap::new();
    for item in items {
        // the item is perhaps a directory or an image file.
        let itemname = item.file_name().into_string().unwrap();
        if item.file_type().unwrap().is_dir() {
            // if the item is a directory:
            let files = fs::read_dir(item.path()).unwrap().map(|res| res.unwrap());
            for file in files {
                let filename = file.file_name().into_string().unwrap();
                images.insert(
                    trim_extention_name(filename.clone()),
                    format!("{}/{}/{}", relative_path, itemname, filename),
                );
            }
        } else {
            // if the item is just an image file:
            images.insert(
                trim_extention_name(itemname.clone()),
                format!("{}/{}", relative_path, itemname),
            );
        }
    }
    images
}

fn trim_extention_name(filename: String) -> String {
    let reg = Regex::new(r"(.*)\..*$").unwrap();
    let caps = reg.captures(filename.as_str()).unwrap();
    caps.get(1).unwrap().as_str().to_string()
}
