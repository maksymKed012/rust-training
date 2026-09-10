use clap::Parser;
use std::borrow::Cow;
use std::fmt::Formatter;

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

fn compare_lines(line: &str, pattern: &str, ignore_case: bool, invert: bool) -> bool {
    let mut pattern: Cow<'_, str> = Cow::Borrowed(pattern);
    if ignore_case {
        pattern = Cow::Owned(pattern.to_lowercase());
    }
    if invert {
        !line.contains(pattern.as_ref())
    } else {
        line.contains(pattern.as_ref())
    }
}

fn build_output_string(line: &str, line_number: i32, show_line_number: bool) -> String {
    let mut line_str_ = String::new();
    if show_line_number {
        line_str_ = format!("line: {}", line_number);
    }
    format!("{} {}", line_str_, line)
}

#[derive(Debug)]
enum GrepError {
    FileNotFound(String),
    IoError(std::io::Error),
}

impl std::fmt::Display for GrepError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            GrepError::FileNotFound(error) => write!(f, "File not found error: {error}"),
            GrepError::IoError(error) => write!(f, "IO error: {error}"),
        }
    }
}

fn run() -> Result<(), GrepError> {
    let cli_args = Args::parse();
    let pattern = &cli_args.pattern;
    let file_name = &cli_args.file_name;
    let ignore_case = cli_args.ignore_case;
    let invert = cli_args.invert;

    let mut file_content = std::fs::read_to_string(file_name).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => {
            GrepError::FileNotFound(format!("File {file_name} not found. {}", e.to_string()))
        }
        _ => GrepError::IoError(e),
    })?;
    // expect("Failed to read the file");
    if ignore_case {
        file_content = file_content.to_lowercase();
    }

    for (i, line) in file_content.lines().enumerate() {
        if compare_lines(line, pattern, ignore_case, invert) {
            let out_str = build_output_string(line, (i + 1) as i32, cli_args.line_number);
            println!("match found: {out_str}");
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error {e}");
        std::process::exit(1);
    }
}
