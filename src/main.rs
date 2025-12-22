use clap::Parser;
use std::path::PathBuf;

/// CLI tool to search for files within a directory (Simple version of `find`)
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
  /// Pattern to search for
  pattern: String,

  /// Directory to search within
  directory: PathBuf,
}

fn main() {
  if let Err(error) = run() {
    eprintln!("filesearch: {}", error);
    std::process::exit(1)
  }
}

fn run() -> Result<(), String> {
  let args = parse_input();
  // println!("{:#?}", args);
  let _b = validate_args(&args)?;

  Ok(())
}

fn parse_input() -> CliArgs {
  CliArgs::parse()
}

// for now, we're not doing any validation
fn validate_args(args: &CliArgs) -> Result<&CliArgs, String> {
  Ok(args)
}
