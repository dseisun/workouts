// TODO: Deserialize a config into the objects you created in models.rs
// Use builder pattern for your workout obj: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
// Or use default struct pattern

mod args;
mod models;
mod generate_workout;
mod run_workout;

use crate::{args::Cli, models::{load_exercises_from_json, load_config, ConfigPath, ExercisePath}, generate_workout::{generate_excercise_categories, generate_workout}};
use clap::Parser;




// A workout needs a time, a config and a list of exercises as input
// Once generated I need to be able to hand it to something to be run (method on itself?), and then be logged (method on itself?)

fn main() {
    //TODO: If you pick a really short time with many configs it still picks 1 of each that fits within the time. Are you okay with this behaviour?
    let cli = Cli::parse();
    
    let workout = generate_workout(cli.minutes, ConfigPath::default(), ExercisePath::default());
    run_workout::run_workout(workout);
}
