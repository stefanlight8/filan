# Filan

A small utility to analyze a directory and show the size of each file type.

# Install

## Package Managers

Currently unavailable via Cargo, Brew, apt, or others.

## Download Binary

Download prebuilt binaries from the [latest release page](https://github.com/stefanlight8/filan/releases).

## Build Manually

To build for your current system:

```sh
make build
```

### Cross-Compiling for Other Platforms

To build for another target platform:

```sh
make build TARGET=<target-triple>
```

# Usage

```sh
$ filan --help
# Displays help message with available options.

$ filan
# Outputs analysis of the current directory, sorted by size, showing top 10 extensions.

$ filan /Your/Path/
# Outputs analysis of /Your/Path/, sorted by size, showing top 10 extensions.

$ filan --sort name
# Sorts the output by extension name instead of size.

$ filan --sort count --limit 5
# Sorts by file count and shows only top 5 extensions.
```
