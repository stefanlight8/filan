# 0.1.2

- Added limit for how many extensions to display.
- Added sorting options: by name, count, and size.
- Added CLI with --help, --limit, and --sort flags.
- Added Windows support (untested).

# 0.1.1

- Improved readability of analyze output.
- Improved code quality: Moved all operations and output handling into the `analyze` function. (Thanks to @nanoqsh)
- Removed build dependency `walkdir` by replacing it with a function. (Thanks to @nanoqsh)
- Removed `Arc<Mutex<>>` since `get_files_types` does not involve parallelism (yet?).

# 0.1.0

Initial version with base functionality.
