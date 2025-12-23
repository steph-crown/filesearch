use clap::Parser;
use globset::{Glob, GlobMatcher};
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

  let matcher = Glob::new(&pattern)
    .map_err(|_| String::from(format!("{}: invalid pattern provided", directory.display())))?
    .compile_matcher();

  find_and_write_matches(&directory, &mut std::io::stdout(), &matcher)
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
  directory: &std::path::Path,
  writer: &mut impl std::io::Write,
  matcher: &GlobMatcher,
) -> Result<(), io::Error> {
  let entries = std::fs::read_dir(directory)?;

  for entry in entries {
    let path = entry?.path();

    if is_match(&path, matcher) {
      writeln!(writer, "{}", path.display())?;
    }

    if path.is_dir() {
      if let Err(err) = find_and_write_matches(&path, writer, matcher) {
        let err_msg = get_msg_from_io_error(&err);
        eprintln!("filesearch: {}: {}", path.display(), err_msg);
      }
    }
  }

  Ok(())
}

fn is_match(directory: &std::path::Path, matcher: &GlobMatcher) -> bool {
  directory
    .file_name()
    .map(|str| matcher.is_match(str))
    .unwrap_or(false)
}
