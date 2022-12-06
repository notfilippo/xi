use std::{path::{PathBuf, Path}, fs};

use clap::Parser;
use rustyline::{Editor, error::ReadlineError};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    file: Option<PathBuf>,
}

const PROMPT: &str = "ix >> ";

fn run(source: String) -> anyhow::Result<()> {
    println!("{}", source);
    Ok(())
}

fn repl() -> anyhow::Result<()> {
    let mut rl = Editor::<()>::new()?;
    loop {
        match rl.readline(PROMPT) {
            Ok(line) => run(line)?,
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

fn file(path: &Path) -> anyhow::Result<()> {
    let source = fs::read_to_string(path)?;
    run(source)
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.file {
        Some(path) => file(&path),
        None => repl(),
    }
}
