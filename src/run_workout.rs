use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle, ProgressIterator};
use strfmt::strfmt;
use crate::{models::{Exercise, ConfigPath, Side, load_exercises_from_json}, generate_workout, tts::{OSXSpeak, Speak, NotImplSpeak}, args::{GenerateParams, unwrap_or_default_exercises}};



#[cfg(not(test))]
use std::thread::sleep;

#[cfg(test)]
fn sleep(_dur: Duration) {}


pub fn run_workout(workout: Vec<Exercise>) {
    // Display the workout before starting
    let mut workout_length = 0.0;
    for (i, e) in workout.iter().enumerate() {
        workout_length += e.total_time() as f64;
        println!("{}: {}", i+1, e.name)
    }
    println!("Workout should take: {} minutes \n\n", workout_length / 60.0);

    let tts = crate::tts::get_speaker();

    for exercise in workout {
        run_exercise(&exercise, &tts);
    }
    tts.tts("Workout complete. You are going to be, so jacked")
}

// TODO set the tests up properly so rust doesn't think my imports above are unused
#[test]
fn test_running_workout() {
    let workout = generate_workout(GenerateParams{minutes: 1, config_path: None, exercises_path: None});
    run_workout(workout)
}

fn run_exercise(exercise: &Exercise, speaker: &Box<dyn Speak>) {
    //TODO: Show next exercise in the progress par (or println)
    //Using closure to capture the speaker ref
    let run_exercise_side = |side: &Side, excercise: &Exercise, rep: u16| {
        let style = ProgressStyle::with_template("{spinner} {elapsed} {percent}% {msg} {bar:40.cyan/blue}")
        .unwrap();
        let bar = ProgressBar::new(exercise.default_time.into());
        bar.set_style(style);
        
        let unformatted_message = format!("{}. Rep {} of {}", &excercise.prompt, rep+1, &excercise.repetition);
        let message = strfmt!(&unformatted_message, side => side.to_string(), name => exercise.name.clone()).unwrap();
        
        let tick = Duration::from_secs(1);

        println!("{}", message);
        println!("{}", exercise.long_desc);
        speaker.tts(&message);
        
        for secs_elapsed in 0..exercise.default_time {
            let time_remaining = exercise.default_time - secs_elapsed;
            bar.set_message(format!("{} Time remaining: {time_remaining}", message));
            
            bar.inc(1);
            sleep(tick)
        } 
    };
    for i in 0..exercise.repetition {
        if exercise.side == Side::Both {
            run_exercise_side(&Side::Left, &exercise, i);
            run_exercise_side(&Side::Right, &exercise, i)}
        else {run_exercise_side(&exercise.side, &exercise, i)}
    }
}

#[test]
fn test_running_exercise() {
    let exc = load_exercises_from_json(unwrap_or_default_exercises(None));
    run_exercise(exc.first().unwrap(), &crate::tts::get_speaker())
}