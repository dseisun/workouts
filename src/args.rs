use clap::{Parser, Args, Subcommand};

use crate::models::ConfigPath;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub workout_type: WorkoutType
}

#[derive(Subcommand, Debug)]
pub enum WorkoutType {
    Generate(GenerateParams),
    Template(TemplateParams)
}

#[derive(Debug, Args)]
pub struct GenerateParams {
    ///Time in minutes to work out for    
    #[arg(short, long)]
    pub minutes: u16,
    
    ///Path to workout generation config
    #[arg(short, long)]
    pub config_path: Option<String>,

    ///Path to json list of exercises  
    #[arg(short, long)]
    pub exercises_path: Option<String>
}

pub fn unwrap_or_default_config(c: Option<String>) -> String {
    c.unwrap_or(String::from("/Users/danielseisun/workspace/workout/config.json"))
}

pub fn unwrap_or_default_exercises (e: Option<String>) -> String {
    e.unwrap_or(String::from("/Users/danielseisun/workspace/workout/exercises.json"))
}

#[derive(Debug, Args)]

pub struct TemplateParams {
    #[arg(short, long)]
    pub workout_path: String
}

#[test]
fn test_cli_generate() {
    let cmd_opts: Vec<&str> = vec!["_binary_name", "generate", "--minutes", "10"];
    let cli = Cli::parse_from(cmd_opts);
}

#[test]
fn test_cli_template() {
    let cmd_opts: Vec<&str> = vec!["_binary_name", "template", "--workout-path", "/path/to/file.json"];
    let cli = Cli::parse_from(cmd_opts);
}

#[test]
fn test_cli_template_works() {
    let cmd_opts: Vec<&str> = vec!["_binary_name", "template", "--workout-path", "/path/to/file.json"];
    let cli = Cli::parse_from(cmd_opts);
    if let WorkoutType::Template(t) = cli.workout_type {
        assert_eq!("/path/to/file.json", t.workout_path)
    }
    else {panic!("if let not activated. Test is broken")}
}

// Generate workout -> Takes minutes, (optional) path to config and (optional) path to workout (we have a sane default)
// Run workout --> Takes path to workout
