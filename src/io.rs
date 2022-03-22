use std::fmt::{Display, Formatter};
use pathdiff;

use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub enum Error {
    Io(std::io::Error),
    UnknownFile(PathBuf),
    PathRelativizeFailure,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => write!(f, "IO error: {}", err),
            Error::UnknownFile(path) => write!(f, "Unknown file (path is neither a file nor a directory): {}", path.to_string_lossy()),
            Error::PathRelativizeFailure => write!(f, "Path relativize failure!"),
        }
    }
}

pub fn process(input: PathBuf, output: PathBuf, f: &mut impl FnMut(String) -> Option<String>) {
    let original_input = input.clone();

    match process_path(original_input.as_path(), input, output, f) {
        Ok(success) => if success {
            println!();
            println!("Done.");
        } else {
            println!();
            println!("Done with errors.");
        },
        Err(err) => println!("{}", err),
    }
}

pub fn process_path(original_input: &Path, input: PathBuf, output: PathBuf, f: &mut impl FnMut(String) -> Option<String>) -> Result<bool, Error> {
    let metadata = input.metadata()?;

    if metadata.is_dir() {
        std::fs::create_dir_all(&output)?;
        let mut error = false;
        let mut blank_line = false;

        for file_entry in std::fs::read_dir(&input)? {
            match file_entry {
                Ok(file_entry) => {
                    let path = file_entry.path();
                    let relative_path = relativize(&input, &path)?;

                    if blank_line {
                        println!();
                        blank_line = false;
                    }

                    match process_path(original_input, path, output.join(relative_path), f) {
                        Ok(success) => if !success {
                            error = true;
                            blank_line = true;
                        },
                        Err(err) => {
                            println!("Errors:\n- {}", err);
                            blank_line = true;
                        }
                    };
                },
                Err(err) => {
                    println!("IO error while iterating through directory: {}", err); // Skip this entry
                },
            }
        }

        if error {
            Ok(false)
        } else {
            Ok(true)
        }
    } else if metadata.is_file() {
        let relative_path_for_display = relativize(original_input, &input)?;
        println!("Processing {}", relative_path_for_display.to_string_lossy());

        let mut input_file = File::open(input)?;

        let mut input_str = String::new();
        input_file.read_to_string(&mut input_str)?;

        let output_str = match f(input_str) {
            Some(result) => result,
            None => return Ok(false),
        };

        drop(input_file);
        let mut output_file = File::create(output)?;
        output_file.write_all(output_str.as_bytes())?;
        Ok(true)
    } else {
        Err(Error::UnknownFile(input))
    }
}

fn relativize(base: &Path, path: &Path) -> Result<PathBuf, Error> {
    pathdiff::diff_paths(path, base).ok_or_else(|| Error::PathRelativizeFailure)
}
