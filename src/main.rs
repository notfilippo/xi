mod env;
mod expr;
mod interpreter;
mod lexer;
mod parser;
mod report;
mod token;
mod value;

use std::{
    fs,
    path::{Path, PathBuf}, cell::RefCell, rc::Rc,
};

use anyhow::Context;
use clap::Parser as CliParser;
use env::Env;
use interpreter::Interpreter;
use miette::Result;
use rustyline::{error::ReadlineError, DefaultEditor};

use crate::lexer::Lexer;
use crate::parser::Parser;

#[derive(CliParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    file: Option<PathBuf>,
    #[arg(short, long)]
    code: Option<String>,
}

const PROMPT: &str = "ix >> ";

fn run(source: String, env: &Rc<RefCell<Env>>) -> Result<()> {
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.scan_tokens()?;
    let mut parser = Parser::new(&source, tokens);
    let statements = parser.parse()?;
    let mut interpreter = Interpreter::new(&source);

    println!("{:?}", statements);
    println!("{}", interpreter.interpret(env, &statements)?);

    Ok(())
}

fn repl() -> anyhow::Result<()> {
    let mut rl = DefaultEditor::new()?;
    rl.load_history("history.txt").ok();
    let env = Rc::new(RefCell::<Env>::default());
    loop {
        let result = match rl.readline(PROMPT) {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                run(line, &env)
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
    let result = run(source, &Rc::new(RefCell::<Env>::default()));
    if let Err(err) = result {
        println!("{:?}", err);
    }

    Ok(())
}

fn immediate(code: String) -> anyhow::Result<()> {
    let result = run(code, &Rc::new(RefCell::<Env>::default()));
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
