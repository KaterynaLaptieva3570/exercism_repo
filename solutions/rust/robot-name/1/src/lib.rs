use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static USED_NAMES: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        let mut robot = Robot {
            name: String::new(),
        };
        robot.reset_name();
        robot
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        let mut used = USED_NAMES.lock().unwrap();
        let mut rng = thread_rng();

        loop {
            let name = format!(
                "{}{}{:03}",
                (b'A' + rng.gen_range(0..26)) as char,
                (b'A' + rng.gen_range(0..26)) as char,
                rng.gen_range(0..1000)
            );
            if !used.contains(&name) {
                used.insert(name.clone());
                self.name = name;
                break;
            }
        }
    }
}
