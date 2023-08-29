use crate::models::{load_exercises_from_json, load_config, CategoryConfig, Exercise, CategoryType, ConfigPath, ExercisePath};
use std::{collections::HashMap, ops::Deref, default};
use rand::{thread_rng, seq::SliceRandom};


pub fn generate_workout(minutes: u16, config_path: ConfigPath, exercise_path: ExercisePath) -> Vec<Exercise> {
    //TODO: Set these as function params
    let total_secs = minutes as f32 * 60_f32;
    let exercises = load_exercises_from_json(exercise_path);
    let exercise_category_map = generate_excercise_categories(exercises);
    let conf = load_config(config_path);

    let mut workout: Vec<Exercise> = vec![];


    for CategoryConfig{category, weight} in &conf.category_config {
        let pct_weight = weight / conf.total_weight() as f32;
        let category_time_secs = (pct_weight * total_secs) as u16;
        let exercise_group = exercise_category_map.get(category).expect("Found no exercises for category type referenced in config");
        let mut cat = fill_category(category_time_secs, exercise_group);
        workout.append(&mut cat);
    }
    workout.shuffle(&mut thread_rng());
    workout
    
}

#[test]
fn test_generate_workout() {
    let workout = generate_workout(1000, ConfigPath::default(), ExercisePath::default());
}

pub fn fill_category(time_in_seconds: u16, exercise_group: &Vec<Exercise>) -> Vec<Exercise> {
    let mut workout_cat_exercises: Vec<Exercise> = vec![];
    let mut time_remaining: i32 = time_in_seconds.into();
    let mut rng = thread_rng();
    let mut e = exercise_group.to_vec();
    e.shuffle(&mut rng);
    
    while time_remaining > 0 { 
        if e.is_empty() {e = exercise_group.to_vec();} //This is going to be a problem no?
        let exc = e.pop().unwrap();
        time_remaining = time_remaining - exc.total_time() as i32;
        workout_cat_exercises.push(exc);
        
    }
    workout_cat_exercises
}

#[test]
fn test_fill_category() {
    let exercises = load_exercises_from_json(Default::default());
    let exercise_category_map = generate_excercise_categories(exercises);
    fill_category(600, exercise_category_map.get(&CategoryType::PhysicalTherapy).unwrap());
}

pub fn generate_excercise_categories(exercises: Vec<Exercise>) -> HashMap<CategoryType, Vec<Exercise>> {
    let mut exercise_category_map: HashMap<CategoryType, Vec<Exercise>> = HashMap::new();


    for exercise in exercises {
        // TODO: Explore using an actual set for the list of exercises? Or writing a test to check for duplicate ids
        // TODO: Exercise description for whne you come back to this after a while
        let cat_id = exercise_category_map.entry(exercise.category_id.clone());
        let cat_exercise_set = cat_id.or_insert(vec![]);
        cat_exercise_set.push(exercise);
    }
    exercise_category_map
}

