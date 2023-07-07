# Schedariu - A Markdown to HTML Converter

Schedariu is a command line utility written in Rust, designed to convert Markdown files into HTML. This utility is perfect for static site generation and other tasks that require bulk conversion of Markdown files into HTML.

## Features

- Converts a single Markdown file to HTML (`SingleFile` mode).
- Converts a directory of Markdown files to HTML, preserving the original directory structure (`MultipleFiles` mode).
- Robust error handling for file access and Markdown parsing errors.

## Usage

You can use Schedariu via the command line with the following syntax:

### SingleFile Mode

Use the `single-file` command followed by the input file path, output directory path, and desired output file name:

```bash
schedariu.exe single-file <input-file> <output-dir> <output-file-name>
```

Example:

```bash
schedariu.exe single-file /home/user/Documents/example.md /home/user/Output example.html
```

This will convert the `example.md` Markdown file into `example.html` in the `Output` directory.

### MultipleFiles Mode

Use the `multiple-files` command followed by the input directory path and output directory path:

```bash
schedariu.exe multiple-files <input-dir> <output-dir>
```

Example:

```bash
schedariu.exe multiple-files /home/user/inputfolder /home/user/outputfolder
```

This will convert all Markdown files in the `MarkdownFiles` directory into HTML, preserving the original directory structure in the `Output` directory.

## Building

To build Schedariu from source, you will need to have the Rust programming language installed. Once installed, you can build the project using the following command in the project root directory:

```bash
cargo build --release
```

The resulting executable will be placed in the `./target/release/` directory.
