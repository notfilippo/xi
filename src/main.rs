mod error;
mod lexer;
mod token;

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use clap::Parser;
use miette::Result;
use rustyline::{error::ReadlineError, Editor};

use crate::lexer::Lexer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    file: Option<PathBuf>,
}

const PROMPT: &str = "ix >> ";

fn run(source: String) -> Result<()> {
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.scan_tokens()?;
    println!("{:?}", tokens);
    Ok(())
}

fn repl() -> anyhow::Result<()> {
    let mut rl = Editor::<()>::new()?;
    loop {
        let result = match rl.readline(PROMPT) {
            Ok(line) => run(line),
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => Err(err).context("readline error")?,
        };

        if let Err(err) = result {
            println!("{:?}", err)
        }
    }

    Ok(())
}

fn file(path: &Path) -> anyhow::Result<()> {
    let source = fs::read_to_string(path)?;
    let result = run(source);
    if let Err(err) = result {
        println!("{:?}", err);
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.file {
        Some(path) => file(&path),
        None => repl(),
    }
}
