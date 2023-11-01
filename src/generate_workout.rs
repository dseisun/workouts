use crate::{models::{load_exercises_from_json, load_config, CategoryConfig, Exercise, CategoryType, ConfigPath}, args::{GenerateParams, unwrap_or_default_config, unwrap_or_default_exercises}};
use std::{collections::HashMap, ops::Deref, default};
use rand::{thread_rng, seq::SliceRandom};

//for input into a fill category
pub struct HydratedCategoryConfig<'a> {
    category: &'a CategoryType,
    category_time_in_secs: u16,
    category_exercises: &'a Vec<&'a Exercise>,
    category_include: Vec<&'a Exercise>,
    category_omit: Vec<&'a Exercise>
}

pub fn generate_workout(generate_params: GenerateParams) -> Vec<Exercise> {
    let minutes = generate_params.minutes;
    let config_path: String = unwrap_or_default_config(generate_params.config_path);
    let exercises_path = unwrap_or_default_exercises(generate_params.exercises_path);

    let total_secs = minutes as f32 * 60_f32;
    let exercises = load_exercises_from_json(exercises_path);
    let exercise_category_map = generate_exercise_categories(&exercises);
    let conf = load_config(config_path);

    let mut workout: Vec<Exercise> = vec![];

    let include = get_exercises_from_name(&conf.include, &exercises);
    let omit = get_exercises_from_name(&conf.omit, &exercises);

    // Add workouts for a given category
    for CategoryConfig{category, weight} in &conf.category_config {
        let pct_weight = weight / conf.total_weight() as f32;
        let category_time_secs = (pct_weight * total_secs) as u16;
        

        let category_include: Vec<&Exercise> = include.iter().filter(|i| &i.category_id == category).cloned().collect();
        let category_omit: Vec<&Exercise> = omit.iter().filter(|i| &i.category_id == category).cloned().collect();
        
        let hydrated_cat_config = HydratedCategoryConfig {
            category: category,
            category_time_in_secs: category_time_secs,
            category_exercises: exercise_category_map.get(category).expect("Found no exercises for category type referenced in config"),
            category_include: category_include,
            category_omit: category_omit,
        };

        let mut cat = fill_category(&hydrated_cat_config);
        workout.append(&mut cat);
    }
    workout.shuffle(&mut thread_rng());
    workout
    
}

#[test]
fn test_generate_workout() {
    let workout = generate_workout(GenerateParams {minutes: 1000, config_path: None, exercises_path: None});
}

fn get_exercise_from_name<'a>(name: &str, exercises: &'a Vec<Exercise>) -> &'a Exercise {
    for e in exercises {
        if e.name == name {
            return &e
        }
    }
    panic!("{}", format!("Tried to get exercise {} and couldn't find by name", name))
}

fn get_exercises_from_name<'a, T: AsRef<str>>(names: &Vec<T>, exercises: &'a Vec<Exercise>) -> Vec<&'a Exercise> {
    let mut found_exercises = vec![];
    for name in names {
        found_exercises.push(get_exercise_from_name(name.as_ref(), exercises));
    }
    found_exercises
}


pub fn fill_category(hcc: &HydratedCategoryConfig) -> Vec<Exercise> {
    let mut workout_cat_exercises: Vec<Exercise> = vec![];
    let mut time_remaining: i32 = hcc.category_time_in_secs.into();


    for i in &hcc.category_include {
        workout_cat_exercises.push((*i).clone());
        time_remaining = time_remaining - i.total_time() as i32
    }

    let category_exercise_list: Vec<&Exercise> = hcc.category_exercises.iter().filter(|&exc| 
        !hcc.category_omit.iter().any(|e: &&Exercise| e.name == exc.name)).cloned().collect();
    let mut remaining_exercises = category_exercise_list.to_vec();
    remaining_exercises.shuffle(&mut thread_rng());
    while time_remaining > 0 { 
        if remaining_exercises.is_empty() {remaining_exercises = category_exercise_list.to_vec();} //Refill the pool of exercises if it gets empty
        let exc = remaining_exercises.pop().unwrap();
        time_remaining = time_remaining - exc.total_time() as i32;
        workout_cat_exercises.push(exc.clone());
        
    }
    workout_cat_exercises
}

#[test]
fn test_fill_category() {
    let exercises = load_exercises_from_json(unwrap_or_default_exercises(None));
    let exercise_category_map = generate_exercise_categories(&exercises);
    let hcc = HydratedCategoryConfig {
        category: &CategoryType::PhysicalTherapy,
        category_time_in_secs: 100,
        category_exercises: exercise_category_map.get(&CategoryType::PhysicalTherapy).unwrap(),
        category_include: vec![],
        category_omit: vec![],
    };
    fill_category(&hcc);
}


#[test]
fn test_fill_category_with_include() {
    let category = CategoryType::PhysicalTherapy;
    let exercises = load_exercises_from_json(unwrap_or_default_exercises(None));
    let exercise_category_map = generate_exercise_categories(&exercises);
    let include = vec!["Clamshells", "Butt Kickers"];
    let include_exercises = get_exercises_from_name(&include, &exercises);
    let category_include: Vec<&Exercise> = include_exercises.iter()
        .filter(|i| i.category_id == category)
        .cloned()
        .collect();

    let category_omit = vec![];

    let hcc = HydratedCategoryConfig {
        category: &CategoryType::PhysicalTherapy,
        category_time_in_secs: 100,
        category_exercises: exercise_category_map.get(&CategoryType::PhysicalTherapy).unwrap(),
        category_include: category_include,
        category_omit,
    };
    
    let cat_exercises = fill_category(&hcc);

    for include_exercise in include_exercises {
        if include_exercise.category_id == *hcc.category {
            assert!(cat_exercises.contains(include_exercise), "{:?} not found in generated filled_category", include_exercise.name);
        }
    }
}

#[test]
fn test_fill_category_with_omit() {
    let category = CategoryType::PhysicalTherapy;
    let exercises = load_exercises_from_json(unwrap_or_default_exercises(None));
    let exercise_category_map = generate_exercise_categories(&exercises);
    let omit = vec!["Clamshells", "Butt Kickers"];
    let omit_exercises = get_exercises_from_name(&omit.clone(), &exercises);
    let category_omit: Vec<&Exercise> = omit_exercises.iter()
        .filter(|i| i.category_id == category)
        .cloned()
        .collect();

    let category_include = vec![];

    let hcc = HydratedCategoryConfig {
        category: &CategoryType::PhysicalTherapy,
        category_time_in_secs: 10000,
        category_exercises: exercise_category_map.get(&CategoryType::PhysicalTherapy).unwrap(),
        category_include: category_include,
        category_omit: category_omit,
    };
    
    let cat_exercises = fill_category(&hcc);

    for omit_exc in omit {
        assert!(!cat_exercises.iter().any(|exc| omit_exc == &exc.name), "Found omitted exercise {}", omit_exc)
    }

}

pub fn generate_exercise_categories(exercises: &Vec<Exercise>) -> HashMap<CategoryType, Vec<&Exercise>> {
    let mut exercise_category_map: HashMap<CategoryType, Vec<&Exercise>> = HashMap::new();


    for exercise in exercises {
        // TODO: Explore using an actual set for the list of exercises? Or writing a test to check for duplicate ids
        let cat_id = exercise_category_map.entry(exercise.category_id.clone());
        let cat_exercise_set = cat_id.or_insert(vec![]);
        cat_exercise_set.push(exercise);
    }
    exercise_category_map
}

