fn main() {
    let test = Course {
        hours: 3.0,
        grade: String::from("A")
    };
    println!("{:?}", test);
    println!("{}", test.points());
    println!("Hello, world!");
}

#[derive(Debug)]
struct Course {
    hours: f64,
    grade: String
}

impl Course {
    fn points(&self) -> f64 {
        self.grade_to_points() * self.hours
    }

    fn grade_to_points(&self) -> f64 {
        match self.grade.as_str() {
            "A" => 4.0,
            "A-" => 3.7,
            "B+" => 3.3,
            "B" => 3.0,
            "B-" => 2.7,
            "C+" => 2.3,
            "C" => 2.0,
            "C-" => 1.7,
            "D+" => 1.4,
            "D" => 1.0,
            "D-" => 0.7,
            "E" => 0.0,
            "F" => 0.0,
            _ => { panic!("Invalid grade given!"); }
        }
    }
}