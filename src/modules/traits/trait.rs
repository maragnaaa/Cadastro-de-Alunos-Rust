use crate::modules::models::model_student::Student;

pub trait Validator {
    fn validate(student: Student) -> Result<Student, String>;
}