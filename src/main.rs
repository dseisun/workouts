// TODO: Overall progress bar / timer

// Use builder pattern for your workout obj: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
// Or use default struct pattern

mod args;
mod models;
mod generate_workout;
mod run_workout;
mod tts;

use crate::{args::Cli, models::{ConfigPath, ExercisePath}, generate_workout::generate_workout};
use clap::Parser;
use std::time::SystemTime;



// A workout needs a time, a config and a list of exercises as input
// Once generated I need to be able to hand it to something to be run (method on itself?), and then be logged (method on itself?)

fn main() {
    let start = SystemTime::now();
    #[cfg(target_os = "macos")]
    // let tts = crate::tts::OSXSpeak;
    
    //TODO: If you pick a really short time with many configs it still picks 1 of each that fits within the time. Are you okay with this behaviour?
    let cli = Cli::parse();
    
    let workout = generate_workout(cli.minutes, ConfigPath::default(), ExercisePath::default());
    println!("{:?}", workout); //TODO Either prettify this or make it a debug logline
    run_workout::run_workout(workout);
    let workout_duration = SystemTime::now().duration_since(start).unwrap();
    println!("Workout took: {:?}", workout_duration)
}
