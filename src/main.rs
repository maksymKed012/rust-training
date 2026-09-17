use clap::Parser;
mod matcher;
use matcher::find_matches;
use std::io::Read;
use thiserror::Error;

#[derive(Parser)]
struct Args {
    #[arg()]
    pattern: String,
    #[arg()]
    file_name: Option<String>,
    #[arg(short = 'i', long)]
    ignore_case: bool,
    #[arg(short = 'n', long)]
    line_number: bool,
    #[arg(short = 'v', long)]
    invert: bool,
}

#[derive(Error, Debug)]
pub enum GrepError {
    #[error("{0}")]
    FileNotFound(String),
    #[error("IO Error. {0}")]
    Io(#[from] std::io::Error),
}

fn get_input_source(source: Option<String>) -> Result<Box<dyn Read>, GrepError> {
    if source.is_none() {
        Ok(Box::new(std::io::stdin()))
    } else {
        let path = source.as_deref().unwrap();
        std::fs::File::open(path)
            .map(|file| Box::new(file) as Box<dyn Read>)
            .map_err(|e| match e.kind() {
                std::io::ErrorKind::NotFound => {
                    GrepError::FileNotFound(format!("File {path} not found. {e}"))
                }
                _ => GrepError::Io(e),
            })
    }
}

fn open_input_source(mut input_source: Box<dyn Read>) -> Result<String, GrepError> {
    let mut buffer: String = String::new();
    input_source
        .read_to_string(&mut buffer)
        .map(|_| buffer)
        .map_err(|e| match e.kind() {
            _ => GrepError::Io(e),
        })
}

fn run() -> Result<(), GrepError> {
    let cli_args = Args::parse();
    let input_source = get_input_source(cli_args.file_name)?;
    let file_content = open_input_source(input_source)?;

    for (i, line) in find_matches(
        &cli_args.pattern,
        &file_content,
        cli_args.ignore_case,
        cli_args.invert,
    ) {
        if cli_args.line_number {
            println!("{}: {}", i + 1, line);
        } else {
            println!("{}", line);
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
