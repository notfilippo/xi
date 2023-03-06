mod expr;
mod interpreter;
mod lexer;
mod parser;
#[allow(dead_code)]
mod report;
mod token;
mod value;

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use clap::Parser as CliParser;
use expr::Visitor;
use interpreter::Interpreter;
use miette::{Result, SourceSpan};
use rustyline::{error::ReadlineError, DefaultEditor};

use crate::parser::Parser;
use crate::{lexer::Lexer, report::ExpectedToken};

#[derive(CliParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    file: Option<PathBuf>,
    #[arg(short, long)]
    code: Option<String>,
}

const PROMPT: &str = "ix >> ";

fn run(source: String) -> Result<()> {
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.scan_tokens()?;
    print!("{:?}\n\n", tokens);
    let mut parser = Parser::new(&source, tokens);
    let expr = parser.scan_exprs()?;
    print!("{:?}\n\n", expr);
    let mut interpreter = Interpreter::new(&source);
    let value = interpreter.visit_expr(&expr).map_err(|_| ExpectedToken {
        span: SourceSpan::new(0.into(), 1.into()),
        src: source,
    })?;
    println!("{}", value);
    Ok(())
}

fn repl() -> anyhow::Result<()> {
    let mut rl = DefaultEditor::new()?;
    rl.load_history("history.txt").ok();
    loop {
        let result = match rl.readline(PROMPT) {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                run(line)
            }
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
        rl.save_history("history.txt")?;
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

fn immediate(code: String) -> anyhow::Result<()> {
    let result = run(code);
    if let Err(err) = result {
        println!("{:?}", err);
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match (cli.file, cli.code) {
        (Some(path), None) => file(&path),
        (None, Some(code)) => immediate(code),
        (None, None) => repl(),
        (Some(_), Some(_)) => unimplemented!(),
    }
}
