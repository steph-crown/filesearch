use clap::Parser;
use globset::Glob;
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
  let CliArgs { pattern, directory } = CliArgs::parse();

  let matcher = Glob::new(&pattern)
    .map_err(|_| String::from(format!("{}: invalid pattern provided", directory.display())))?
    .compile_matcher();

  filesearch::find_and_write_matches(&directory, &mut std::io::stdout(), &matcher).map_err(
    |err| {
      format!(
        "{}: {}",
        directory.display(),
        filesearch::get_msg_from_io_error(&err)
      )
    },
  )?;

  Ok(())
}
