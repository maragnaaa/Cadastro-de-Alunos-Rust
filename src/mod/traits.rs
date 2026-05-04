use crate::r#mod::model_student::Student;

pub trait Validator {
    fn validate(student: Student) -> Result<Student, String>;
}