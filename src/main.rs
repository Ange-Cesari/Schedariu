mod cli;
mod errors;
mod file_handler;
mod html_generator;
mod markdown_parser;

use cli::Cli;
use errors::SchedariuError;
use html_generator::generate_html;
use markdown_parser::MarkdownParser;

use structopt::StructOpt;

fn main() -> Result<(), SchedariuError> {
    let args = Cli::from_args();
    let parser = MarkdownParser::new();
    match args.cmd {
        cli::Command::SingleFile {
            input_file,
            output_dir,
            output_file_name,
        } => {
            let output_file_path = output_dir.join(output_file_name);
            file_handler::process_single_file(
                &input_file,
                &output_file_path,
                &|markdown| parser.parse(markdown),
                &generate_html,
            )?;
        }
        cli::Command::MultipleFiles {
            input_dir,
            output_dir,
        } => {
            // Implement processing for multiple files
            file_handler::process_directory(
                &input_dir,
                &output_dir,
                &|markdown| parser.parse(markdown),
                &generate_html,
            )?;
        }
    }
    Ok(())
}
