use clap::Parser;
mod matcher;
use matcher::find_matches;
use thiserror::Error;

#[derive(Parser)]
struct Args {
    #[arg()]
    pattern: String,
    #[arg()]
    file_name: String,
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

fn open_file(file_name: &str) -> Result<String, GrepError> {
    std::fs::read_to_string(file_name).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => {
            GrepError::FileNotFound(format!("File {file_name} not found. {e}"))
        }
        _ => GrepError::Io(e),
    })
}

fn run() -> Result<(), GrepError> {
    let cli_args = Args::parse();
    let file_content = open_file(&cli_args.file_name)?;

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
