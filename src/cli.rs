use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "Schedariu", about = "A static site generator.")]
pub struct Cli {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "single-file")]
    SingleFile {
        #[structopt(parse(from_os_str))]
        input_file: PathBuf,

        #[structopt(parse(from_os_str))]
        output_dir: PathBuf,

        output_file_name: String,
    },

    #[structopt(name = "multiple-files")]
    MultipleFiles {
        #[structopt(parse(from_os_str))]
        input_dir: PathBuf,

        #[structopt(parse(from_os_str))]
        output_dir: PathBuf,
    },
}
