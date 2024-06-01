use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file {}", args.path.display()))?;
    println!("file content: {}", content);
    Ok(())
}