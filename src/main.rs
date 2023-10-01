// TODO Add general timer (since the time actually exercised can be off due to include listing and bin packing issues)
// TODO: Deserialize a config into the objects you created in models.rs
// TODO: Add more notification on how many reps you will do and what rep you're on
// Use builder pattern for your workout obj: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
// Or use default struct pattern

mod args;
mod models;
mod generate_workout;
mod run_workout;
mod tts;

use crate::{args::Cli, models::{load_exercises_from_json, load_config, ConfigPath, ExercisePath}, generate_workout::{generate_exercise_categories, generate_workout}};
use clap::Parser;




// A workout needs a time, a config and a list of exercises as input
// Once generated I need to be able to hand it to something to be run (method on itself?), and then be logged (method on itself?)

fn main() {
    
    #[cfg(target_os = "macos")]
    let tts = crate::tts::OSXSpeak;
    
    //TODO: If you pick a really short time with many configs it still picks 1 of each that fits within the time. Are you okay with this behaviour?
    let cli = Cli::parse();
    
    let workout = generate_workout(cli.minutes, ConfigPath::default(), ExercisePath::default());
    println!("{:?}", workout); //TODO Either prettify this or make it a debug logline
    run_workout::run_workout(workout);
}
