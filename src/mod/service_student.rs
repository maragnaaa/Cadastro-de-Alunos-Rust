use super::traits::Validator;
use super::model_student::Student;
use regex::Regex;

pub struct StudentService;

impl Validator for StudentService {
    fn validate(student: Student) -> Result<Student, String> {
        if student.name.trim().is_empty() ||
        student.grade.trim().is_empty() {
            return Err(String::from("The name cannot be empty. Please try again.").into());
        }

        let re_grade = Regex::new(r"^[A-Fa-f]+$").unwrap();

        if !re_grade.is_match(&student.grade) {
            return Err(String::from("Invalid grade. Please try again.").into())
        }

        Ok(student)
    }
}