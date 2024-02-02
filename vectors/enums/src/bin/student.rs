enum Degree {
    BTech,
    MTech,
    PhD,
}

struct Student {
    name: String,
    program: Degree,
}

fn main() {
    let s = Student {
        name: String::from("John Doe"),
        program: Degree::BTech,
    };

    match s.program {
        Degree::BTech => println!("{} is a UG student", s.name),
        Degree::MTech | Degree::PhD => println!("{} is a PG student", s.name),
    }
}
