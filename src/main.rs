// TODO: Deserialize a config into the objects you created in models.rs
// Use builder pattern for your workout obj: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html

mod args;
mod models;
use std::vec;

use crate::args::Cli;
use clap::Parser;
use models::Exercises;

enum WorkoutState {
    Empty,
    Generated,
    Complete
}

// A workout needs a time, a config and a list of exercises as input
// Once generated I need to be able to hand it to something to be run (method on itself?), and then be logged (method on itself?)

fn main() {
    let cli = Cli::parse();
    println!("{}", cli.time);
}
