use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle, ProgressIterator};
use strfmt::strfmt;
use crate::{models::{Exercise, ConfigPath, ExercisePath, Side, load_exercises_from_json}, generate_workout};


pub fn run_workout(workout: Vec<Exercise>) {
    // TODO: Speech to text
    
    for exercise in workout {
        println!("{:?}", strfmt!(&exercise.prompt, side => exercise.side.to_string(), name => exercise.name.clone()).unwrap())

    }
}

#[test]
fn test_running_workout() {
    let workout = generate_workout(100, ConfigPath::default(), ExercisePath::default());
    run_workout(workout)
}

fn run_exercise(exercise: &Exercise) {
    if exercise.side == Side::Both {
        run_exercise_side(&Side::Left, &exercise);
        run_exercise_side(&Side::Right, &exercise)
    }
    else {run_exercise_side(&exercise.side, &exercise)}

    fn run_exercise_side(side: &Side, exercise: &Exercise) {
        let style = ProgressStyle::with_template("{spinner} {elapsed} {percent}% {msg} {bar:40.cyan/blue}")
        .unwrap();
        let bar = ProgressBar::new(exercise.default_time.into());
        let message = strfmt!(&exercise.prompt, side => side.to_string(), name => exercise.name.clone()).unwrap();
        bar.set_style(style);
        let tick = Duration::from_secs(1);
        println!("{}", message);
        for secs_elapsed in 0..exercise.default_time {
            let time_remaining = exercise.default_time - secs_elapsed;
            
            bar.set_message(format!("{} Time remaining: {time_remaining}", message));
            bar.inc(1);
            std::thread::sleep(tick)
        } 
    }

}

#[test]
fn test_running_exercise() {
    let exc = load_exercises_from_json(ExercisePath::default());
    run_exercise(exc.first().unwrap());
    
}