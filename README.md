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
codemd --input <INPUT> --language <LANGUAGE> [--output <OUTPUT>]
```

**Options:**

- `-l`, `--language <LANGUAGE>`: Language tag to search for (e.g. `python` or `py` for Python, `typescript` or `ts` for TypeScript..)
- `-i`, `--input <INPUT>`: Path to the input file (must be markdown)
- `-o`, `--output <OUTPUT>`: Path to the output file. If not provided, code will be printed to the console with syntax highlighting.
- `-c`, `--command <COMMAND>`: Command to execute in order to run the code. Can be used only when `--output` is set.
- `-h`, `--help`: Print help information
- `-V`, `--version`: Print version information

> [!NOTE]
>
> When using the `--language` option, be mindful of the fact that it means language _tag_: `python` and `py` are two distinct tags, even if they refer to the same language.

**Examples**

Read code and rich-print it to stdout:

```bash
codemd --input README.md --language python
```

Read code and write the output to a file:

```bash
codemd --input guide.md --language sh --output guide.sh
```

Read the code, write to a file and execute a command:

```bash
codemd --input tests.md --language python --output tests.py --command 'pytest tests.py'
```
