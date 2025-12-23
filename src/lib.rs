use std::io;

use globset::GlobMatcher;

pub fn get_msg_from_io_error(err: &io::Error) -> String {
  let kind = err.kind();

  match kind {
    io::ErrorKind::NotFound => String::from("no such file or directory"),
    io::ErrorKind::PermissionDenied => String::from("permission denied"),
    _ => String::from("unknown error"),
  }
}

pub fn find_and_write_matches(
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
