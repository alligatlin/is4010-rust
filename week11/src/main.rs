mod student;

fn main() {
    println!("Student management system");

    // I removed the /* from here...
    use student::{CourseGrade, Grade, Student, StudentDatabase};

    let mut db = StudentDatabase::new();

    let mut alice = Student::new(
        String::from("S001"),
        String::from("Alice Johnson"),
        String::from("alice@example.com"),
    );

    alice.add_grade(CourseGrade::new(
        String::from("IS4010"),
        String::from("App Dev with AI"),
        3,
        Grade::A,
    ));

    println!("Student: {}", alice.name);
    println!("GPA: {:.2}", alice.calculate_gpa());
    println!("Standing: {}", alice.class_standing());

    db.add_student(alice).unwrap();
    println!("Students in database: {}", db.student_count());
    // ...and I removed the */ from here!
}
