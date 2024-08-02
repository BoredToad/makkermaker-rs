struct Student {
    id: usize,
    name: String,
    previous_partners: Vec<(usize, u32)>, // id, count
}

impl Student {
    pub fn new(id: usize, name: String) -> Student {
        Student {
            id,
            name,
            previous_partners: Vec::new(),
        }
    }
}

struct Class {
    students: Vec<Student>,
}

impl Class {
    pub fn new<T>(student_names: T) -> Class
    where
        T: Iterator<Item = String>,
    {
        Class {
            students: student_names
                .enumerate()
                .map(|(i, name)| Student::new(i, name))
                .collect(),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
