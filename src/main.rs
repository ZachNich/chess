use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)?;
    println!("file content: {}", content);
    Ok(())
}