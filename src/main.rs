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
  let CliArgs { pattern, directory } = CliArgs::parse();

  find_and_write_matches(&pattern, &directory, &mut std::io::stdout())
    .map_err(|err| format!("{}: {}", directory.display(), get_msg_from_io_error(&err)))?;

  Ok(())
}

fn get_msg_from_io_error(err: &io::Error) -> String {
  let kind = err.kind();

  match kind {
    io::ErrorKind::NotFound => String::from("no such file or directory"),
    io::ErrorKind::PermissionDenied => String::from("permission denied"),
    _ => String::from("unknown error"),
  }
}

fn find_and_write_matches(
  pattern: &str,
  directory: &std::path::Path,
  writer: &mut impl std::io::Write,
) -> Result<(), io::Error> {
  let entries = std::fs::read_dir(directory)?;

  for entry in entries {
    let entry = entry?;
    let path = entry.path();

    if does_file_or_dir_match_pattern(pattern, &path) {
      writeln!(writer, "{}", path.display())?;
    }

    if path.is_dir() {
      if let Err(err) = find_and_write_matches(pattern, &path, writer) {
        let err_msg = get_msg_from_io_error(&err);
        eprintln!("filesearch: {}: {}", path.display(), err_msg);
      }
    }
  }

  Ok(())
}

fn does_file_or_dir_match_pattern(pattern: &str, directory: &std::path::Path) -> bool {
  directory
    .file_name()
    .and_then(|os_str| os_str.to_str())
    .map(|str| str.starts_with(pattern))
    .unwrap_or(false)
}
