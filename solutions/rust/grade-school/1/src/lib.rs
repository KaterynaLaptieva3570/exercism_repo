use std::collections::BTreeMap;

pub struct School {
    grades: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        // Перевіряємо, чи студент вже існує в будь-якому класі
        if self
            .grades
            .values()
            .any(|students| students.contains(&student.to_string()))
        {
            return; // якщо є — не додаємо ніде
        }

        // Додаємо у потрібний клас
        let students = self.grades.entry(grade).or_insert(Vec::new());
        students.push(student.to_string());
        students.sort();
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().copied().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grades
            .get(&grade)
            .cloned()
            .unwrap_or_else(Vec::new)
    }
}
