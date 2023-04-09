use std::fs;
use std::io;
use std::path::Path;

use thiserror::Error;

mod xml;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum MinifyError {
    #[error("could not read file: {0}")]
    IoError(#[from] io::Error),
    #[error("unsupported file type: {0}")]
    UnsupportedFileType(String),
    #[error("file has no extension")]
    NoFileExtension,
    #[error("{0}")]
    ParseXmlError(#[from] quick_xml::Error),
    #[error("{0}")]
    ParseUtf8Error(#[from] std::string::FromUtf8Error),
}

/// Minifies a file.
pub fn minify(path: impl AsRef<Path>) -> Result<String, MinifyError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path)?;

    let Some(extension) = path.extension().and_then(|s| s.to_str()) else {
        return Err(MinifyError::NoFileExtension);
    };

    match extension {
        "xml" | "svd" => xml::minify(&contents),
        _ => Err(MinifyError::UnsupportedFileType(extension.to_string())),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let path = args.get(0).expect("no path given");
    match minify(path) {
        Ok(minified) => println!("{}", minified),
        Err(e) => eprintln!("{}", e),
    }
}
