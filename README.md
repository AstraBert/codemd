# codemd

Command line tool to extract code from markdown files.

## Installation

```sh
# with npm
npm install @cle-does-things/codemd

# with cargo
cargo install codemd
```

Check installation:

```sh
codemd --help
```

## Usage

```sh
codemd --input <CATEGORY> --language <LANGUAGE> [--output <OUTPUT>]
```

**Options:**

- `-l`, `--language <LANGUAGE>`: Language tag to search for (e.g. `python` or `py` for Python, `typescript` or `ts` for TypeScript..)
- `-i`, `--input <INPUT>`: Path to the input file (must be markdown)
- `-o`, `--output <OUTPUT>`: Path to the output file. If not provided, code will be printed to the console with syntax highlighting.
- `-h`, `--help`: Print help information
- `-V`, `--version`: Print version information

## Roadmap 

- [ ] Add the possibility to execute extracted code (next release)
