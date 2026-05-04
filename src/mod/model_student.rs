pub struct Student {
    pub name: String,
    pub grade: String,
    pub status: bool,
}

impl Student {
    pub fn new(name: String, grade: String, status: bool) -> Self {
        Self {name, grade, status}
    }

    pub fn show_data(&self) -> (&String, &String, bool) {
        (&self.name, &self.grade, self.status)
    }

    pub fn change_status(&mut self, new_status: bool) {
        self.status = new_status
    }

    pub fn change_grade(&mut self, new_grade: String) {
        self.grade = new_grade
    }
}