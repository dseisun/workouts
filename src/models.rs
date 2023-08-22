use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Exercises {
    pub date_added: String,
    pub name: String,
    pub category_id: u8,
    pub side: String,
    pub default_time: u8,
    pub repetition: u8,
    pub prompt: String
}

#[test]
fn test_loading_exercises() {
    let exc = load_exercises_from_json();
    assert_eq!(exc.first().unwrap().name, "Butt Kickers")
}

pub fn load_exercises_from_json() -> Vec<Exercises> {
    let exc_json = std::fs::read_to_string("exercises.json").unwrap();
    serde_json::from_str(&exc_json).unwrap()
}

pub struct Config {
    category_configs: Vec<CategoryConfig>,
    whitelist: Vec<u8>
}

pub enum CategoryType {
    PhysicalTherapy,
    Stretch,
    Strength,
    Rolling
}

pub struct CategoryConfig {
    category: CategoryType,
    weight: u8
}
