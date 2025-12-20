# filesearch

This is a CLI tool that allows you search for files that matches a given pattern within a directory. A simple version of `find` command.

## Usage

```bash
filesearch <pattern> [directory] [flags]
```

- `<pattern>` (required): Then search pattern to match against file names.

  - Can be a simple string, e.g. `main`
  - Can be a wildcard pattern, e.g. `test-*`, `*.rs`
  - Case sensitive

- `<directory>` (optional): The directory to search (recursively) in.
  - Defaults to `.` (current working directory).
  - Must be a valid directory path
  - Example: `.`, `./src`

## Getting Help

To get more information about the tool, run:

```bash
filesearch --help
```

Or

```bash
filesearch -h
```
