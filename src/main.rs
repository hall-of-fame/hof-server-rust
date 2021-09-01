use std::collections::HashMap;
use std::fs;

type Department = HashMap<String, Grade>;
type Grade = HashMap<String, PersonData>;
type PersonData = HashMap<String, String>;

fn main() {
    const DEPTS: [&str; 8] = ["PM", "Design", "Frontend", "Backend", "Android", "iOS", "SRE", "0xfa"];
    let mut total_data: HashMap<String, Department> = HashMap::new();

    for dept in DEPTS {
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
    println!("{:?}", total_data);
}

fn get_person_images(_dir: fs::DirEntry) -> PersonData {
    HashMap::<String, String>::new()
}
