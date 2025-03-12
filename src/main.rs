use clap::Parser;

use std::{fs::File, path::PathBuf, process::exit};

mod document;

use document::Document;

#[derive(Parser, Debug)]
#[command(version, about, long_about = Some("A toy program written to view HTML documents."))]
struct Args {
    file: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let _ = match File::open(args.file) {
        Ok(file) => Document::new(file),
        Err(_) => {
            eprintln!("File not found.");
            exit(1);
        }
    }?;
    Ok(())
}
