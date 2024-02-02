#[derive(PartialEq)]
enum CourseType {
    Core,
    Elective,
}

struct Course {
    credits: u32,
    category: CourseType,
}

fn main() {
    let mut courses: Vec<Course> = vec![];
    for i in 0..10 {
        let c = if i % 3 == 0 {
            Course {
                credits: 8,
                category: CourseType::Core,
            }
        } else {
            Course {
                credits: 6,
                category: CourseType::Elective,
            }
        };
        courses.push(c);
    }

    let core_credits: u32 = courses
        .iter()
        .filter(|c| c.category == CourseType::Core)
        .map(|c| c.credits)
        .sum();
    println!("Core credits = {core_credits}");
}
