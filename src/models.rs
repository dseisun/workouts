use std::{io::BufReader, fs::File};

use serde::{Serialize, Deserialize};

use crate::args::{unwrap_or_default_config, unwrap_or_default_exercises};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Exercise {
    pub date_added: String,
    pub name: String,
    pub category_id: CategoryType,
    pub side: Side,
    pub default_time: u16,
    pub repetition: u16,
    pub prompt: String,
    pub long_desc: String,
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
    let exc = load_exercises_from_json(unwrap_or_default_exercises(None));
    assert_eq!(exc.first().unwrap().name, "Butt Kickers")
}



//TODO: Make this take a proper Path instead of a string
pub fn load_exercises_from_json(path: String) -> Vec<Exercise> {
    let exc_json = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&exc_json).unwrap()
}


//Config models
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "categories")]
    pub category_config: Vec<CategoryConfig>,
    pub include: Vec<String>,
    pub omit: Vec<String>
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
  let config = load_config(unwrap_or_default_config(None));
  dbg!(config.total_weight());
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryConfig {
    #[serde(rename = "name")]
    pub category: CategoryType,
    pub weight: f32
}

//TODO - Switch category type from enum back to a string
// Benefits are ability to easily add new category types, and you benefit almost nothing from the type safety
// A specific feature you want is to be able to create a new custom category type
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
      
        "include": ["this is a test"],
        "omit": []
      }"#;

      let t: Config = serde_json::from_str(&data).unwrap();
}



//TODO: Make this take a proper Path instead of a string
pub fn load_config(config_path: String) -> Config {
  let raw_config = std::fs::read_to_string(config_path).unwrap();
  serde_json::from_str(&raw_config).unwrap()
}

#[test]
fn test_default_load_config() {
  load_config(unwrap_or_default_config(None));
}


pub enum ConfigPath {
  None,
  Some(String)
}

impl ConfigPath {
    pub fn unwrap_or_default(self) -> String {
      match self {
        ConfigPath::None => String::from("/Users/danielseisun/workspace/workout/config.json"),
        ConfigPath::Some(p) => p
      }
    }
}

pub enum WorkoutPath {
  None,
  Some(String)
}

impl WorkoutPath {
  fn unwrap_or_default(self) -> String {
    match self {
      WorkoutPath::None => String::from("/Users/danielseisun/workspace/workout/exercises.json"),
      WorkoutPath::Some(p) => p
    }
  }
}

impl Into<ConfigPath> for Option<String> {
  fn into(self) -> ConfigPath {
      match self {
          None => ConfigPath::None,
          Some(t) => ConfigPath::Some(t)
      }
  }
}

impl Into<WorkoutPath> for Option<String> {
  fn into(self) -> WorkoutPath {
      match self {
          None => WorkoutPath::None,
          Some(t) => WorkoutPath::Some(t)
      }
  }
}