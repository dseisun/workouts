use clap::Parser;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    ///Time in minutes to work out for
    #[arg(short, long)]
    pub time: u8
}