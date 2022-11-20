#![allow(unused)]

use clap::Parser;
use anyhow::{Context, Result};
use std::io::{self, Write};

//Search for a pattern in a file and display the line that contain it
#[derive(Parser)]
#[derive(Debug)]
struct Cli {
    //The pattern to look for
    pattern: String,
    //Path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("couldn't read file {}", &args.path.display()))?;

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());
    // writeln!(handle, "file content: {}", content); locking method used, can also use BufWriter
    Ok(())
    // println!("{:?}", args); // display structure
}

#[test]
fn test_find_match() {
    let mut result = Vec::new();

    grrs::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n")
}