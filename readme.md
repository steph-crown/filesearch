# filesearch

This is a CLI tool that allows you search for files that matches a given pattern within a directory. A simple version of `find` command.

## Installation

This is currently distributed as a source crate, therefore you need to have `rustup`, `cargo`, and a linker installed. Refer to [Rust's install guide](https://rust-book.cs.brown.edu/ch01-01-installation.html).

If you have the above set up, run

```bash
cargo install filesearch
```

## Usage

```bash
filesearch <pattern> [directory]
```

- `<pattern>` (required): Then search pattern to match against file names.

  - Can be a simple string, e.g. `main`
  - Can be a wildcard pattern, e.g. `test-*`, `*.rs`
  - Case sensitive

- `<directory>` (required): The directory to search (recursively) in.
  - Defaults to `.` (current working directory).
  - Must be a valid directory path
  - Example: `.`, `./src`

## Examples

```bash
# Find all Rust files in the current folder

filesearch "*.rs" .
```

```bash
# Find a specific config file in the /etc folder

filesearch "hosts" /etc
```

## Getting Help

To get more information about the tool, run:

```bash
filesearch --help
```

Or

```bash
filesearch -h
```

> ℹ️
>
> **Note**: Patterns must be an exact match unless wildcards are used. Searching for `main` will not find `main.rs`.
