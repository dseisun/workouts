use std::{io::BufReader, fs::File};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Exercise {
    pub date_added: String,
    pub name: String,
    pub category_id: CategoryType,
    pub side: Side,
    pub default_time: u16,
    pub repetition: u16,
    pub prompt: String
}

impl Exercise {
  pub fn total_time(&self) -> u16 {
    if self.side == Side::Both {self.default_time * self.repetition*2}
    else {self.default_time * self.repetition}
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Side {
  Left,
  Right,
  Both,
  Null
}

impl ToString for Side {
    fn to_string(&self) -> String {
        match self {
          Side::Left => String::from("Left"),
          Side::Right => String::from("Right"),
          Side::Both => String::from("Both"),
          Side::Null =>String::from("Null") 
        }
    }
}

#[test]
fn test_loading_exercises() {
    let exc = load_exercises_from_json(Default::default());
    assert_eq!(exc.first().unwrap().name, "Butt Kickers")
}


pub struct ExercisePath {pub path: String}
impl Default for ExercisePath {
    fn default() -> Self {
        Self { path: String::from("/Users/danielseisun/workspace/workout/exercises.json")}
    }
}

pub fn load_exercises_from_json(path: ExercisePath) -> Vec<Exercise> {
    let exc_json = std::fs::read_to_string(path.path).unwrap();
    serde_json::from_str(&exc_json).unwrap()
}


//Config models
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "categories")]
    pub category_config: Vec<CategoryConfig>,
    pub whitelist: Vec<u8>
}

impl Config {
  pub fn total_weight(&self) -> f32 {
    let mut tot_weight = 0_f32;
    for cat_config in &self.category_config {
      tot_weight += cat_config.weight
    }
    tot_weight
  }

}

#[test]
fn test_total_weight() {
  let config = load_config(Default::default());
  dbg!(config.total_weight());
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryConfig {
    #[serde(rename = "name")]
    pub category: CategoryType,
    pub weight: f32
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum CategoryType {
    PhysicalTherapy,
    Stretch,
    Strength,
    Rolling
}

#[test]
fn test_sample_json_deserializes() {
    let data = r#"
    {
        "categories": [
          {
            "name": "physical_therapy",
            "weight": 0.20
          },
          {
            "name": "stretch",
            "weight": 0.25
          },
          {
            "name": "strength",
            "weight": 0.35
          },
          {
            "name": "rolling",
            "weight": 0.20
          }
        ],
      
        "whitelist": []
      }"#;

      let t: Config = serde_json::from_str(&data).unwrap();
}


pub struct ConfigPath {
  path: String
}



impl Default for ConfigPath {
    fn default() -> Self {
        Self { path: String::from("/Users/danielseisun/workspace/workout/config.json")}
    }
}

pub fn load_config(config_path: ConfigPath) -> Config {
  let raw_config = std::fs::read_to_string(config_path.path).unwrap();
  serde_json::from_str(&raw_config).unwrap()
}

#[test]
fn test_default_load_config() {
  load_config(Default::default());
}