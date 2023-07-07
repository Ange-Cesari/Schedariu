use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write, Read};
use std::path::{Path};
use anyhow::Result;
use crate::errors::SchedariuError;

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_file<P: AsRef<Path>>(path: P, content: &str) -> Result<()> {
    let file = File::create(path)?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(content.as_bytes())?;
    Ok(())
}


pub fn copy_file_to<P: AsRef<Path>>(from: P, to: P) -> Result<()> {
    fs::copy(from, to)?;
    Ok(())
}

pub fn process_single_file<P: AsRef<Path>>(input_file: P, output_file: P, parse: &dyn Fn(&str) -> String, generate: &dyn Fn(&str) -> String) -> Result<(), SchedariuError> {
    if !input_file.as_ref().exists() {
        return Err(SchedariuError::FileNotFound);
    }

    let markdown = read_file(&input_file)?;
    let ast = parse(&markdown);
    let html = generate(&ast);

    if let Some(parent) = output_file.as_ref().parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    write_file(&output_file, &html)?;
    Ok(())
}


pub fn process_directory<P: AsRef<Path>>(input_dir: P, output_dir: P, parse: &dyn Fn(&str) -> String, generate: &dyn Fn(&str) -> String) -> Result<(), SchedariuError> {
    // Vérifiez si le répertoire de sortie existe, sinon créez-le
    if !input_dir.as_ref().exists() {
        return Err(SchedariuError::FileNotFound);
    }

    // Check if the output directory exists, if not create it
    if !output_dir.as_ref().exists() {
        fs::create_dir_all(&output_dir)?;
    }

    // Parcourez chaque fichier dans le répertoire source
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a directory
        if path.is_dir() {
            // Create the new directory path in the output directory
            let mut new_path = output_dir.as_ref().to_path_buf();
            new_path.push(path.file_name().unwrap());

            // Process the directory recursively
            process_directory(&path, &new_path, parse, generate)?;
        } else if path.is_file() {
            // Create the new file path in the output directory
            let mut new_path = output_dir.as_ref().to_path_buf();
            new_path.push(path.file_name().unwrap());

            // Check if the file is a markdown file
            if path.extension().unwrap_or_default() == "md" {
                new_path.set_extension("html");
                
                // Process the file
                process_single_file(&path, &new_path, parse, generate)?;
            } else {
                // Copy the non-markdown file to the output directory
                copy_file_to(&path, &new_path)?;
            }
        }
    }

    Ok(())
}
