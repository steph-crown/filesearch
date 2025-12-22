use clap::Parser;
use std::{io, path::PathBuf};

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
  let CliArgs { pattern, directory } = validate_args(&args)?;

  find_and_write_matches(pattern, directory, &mut std::io::stdout())
    .map_err(|err| format!("{}: {}", directory.display(), get_msg_from_io_error(&err)))?;

  Ok(())
}

fn get_msg_from_io_error(err: &io::Error) -> String {
  let kind = err.kind();

  if kind == io::ErrorKind::NotFound {
    String::from("no such file or directory")
  } else {
    String::from("unknown error")
  }
}

fn parse_input() -> CliArgs {
  CliArgs::parse()
}

fn find_and_write_matches(
  pattern: &String,
  directory: &PathBuf,
  writer: &mut impl std::io::Write,
) -> Result<(), io::Error> {
  let entries = std::fs::read_dir(directory)?;

  for entry in entries {
    let entry = entry?;
    let path = entry.path();

    if does_file_or_dir_match_pattern(&pattern, &path) {
      writeln!(writer, "{}", path.display()).unwrap();
    }

    if path.is_dir() {
      let _ = find_and_write_matches(pattern, &path, writer);
    }
  }

  Ok(())
}

fn does_file_or_dir_match_pattern(pattern: &String, directory: &PathBuf) -> bool {
  if let Some(file_or_dir_name) = directory.file_name() {
    let file_or_dir_name = file_or_dir_name.display().to_string();

    file_or_dir_name.starts_with(pattern)
  } else {
    false
  }
}

// for now, we're not doing any validation
fn validate_args(args: &CliArgs) -> Result<&CliArgs, String> {
  Ok(args)
}
