use clap::Parser;
use std::fs::File;
use thiserror::Error;

pub mod parser;
pub mod script;
pub mod sim;
pub mod util;

/// Simulates potential worlds in Blood on the Clock Tower
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Yaml file to read
    #[arg(short, long)]
    file: String,
}

#[derive(Error, Debug)]
enum CLIError {
    #[error("file error: {0}")]
    FileError(#[from] std::io::Error),
    #[error("yaml parsing error: {0}")]
    YAMLError(#[from] serde_yaml::Error),
    #[error("error parsing world: {0}")]
    ParseWorldError(#[from] script::ParseError),
}

fn main() -> Result<(), CLIError> {
    let args = Args::parse();
    let file = File::open(args.file)?;
    let world: parser::World = serde_yaml::from_reader(file)?;
    println!("yaml: {:#?}", world);

    let sim = script::parse_world_to_sim(&world)?;
    println!("sim: {:#?}", sim);

    return Ok(());
}
