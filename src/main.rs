use std::collections::HashMap;
use std::fs;

type Department = HashMap<String, Grade>;
type Grade = HashMap<String, PersonData>;
type PersonData = Vec<String>;

fn main() {
    let depts = vec![
        "PM", "Design", "Frontend", "Backend", "Android", "iOS", "SRE", "0xfa",
    ].iter().map(|p| p.to_string()).collect();
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
                let person_data = get_person_images(person_dir);
                grade_data.insert(person_name, person_data);
            }
            dept_data.insert(grade, grade_data);
        }
        total_data.insert(dept.to_string(), dept_data);
    }

    total_data
}

fn get_person_images(dir: fs::DirEntry) -> PersonData {
    let items = fs::read_dir(dir.path()).unwrap().map(|res| res.unwrap());
    let mut images: PersonData = Vec::new();
    for item in items {
        if item.file_type().unwrap().is_dir() {
            let files = fs::read_dir(item.path()).unwrap().map(|res| res.unwrap());
            for file in files {
                images.push(file.file_name().into_string().unwrap());
            }
        } else {
            images.push(item.file_name().into_string().unwrap());
        }
    }
    images
}
