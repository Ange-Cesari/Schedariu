use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum SchedariuError {
    #[error("File not found")]
    FileNotFound,

    #[error("IO error occurred: {0}")]
    IOError(#[from] io::Error),

    #[error("Failed to parse Markdown: {0}")]
    MarkdownParseError(String),

    #[error("Failed to generate HTML: {0}")]
    HTMLGenerationError(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
    // We can add more specific errors as we progress in the development
}
