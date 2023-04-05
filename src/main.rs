mod builtin;
mod context;
mod dict;
mod env;
mod expr;
mod function;
mod interpreter;
mod lexer;
mod list;
mod parser;
mod report;
mod resolver;
mod token;
mod value;

use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
    time::SystemTime,
};

use anyhow::Context;
use clap::Parser as CliParser;
use env::Env;
use miette::Result;
use rustyline::{error::ReadlineError, DefaultEditor};

use crate::{
    context::Ctx,
    interpreter::{interpret, RuntimeError},
    lexer::Lexer,
    resolver::Resolver,
};
use crate::{parser::Parser, value::Value};

#[derive(CliParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    file: Option<PathBuf>,
    #[arg(short, long)]
    code: Option<String>,
}

const PROMPT: &str = "ix >> ";

fn run(source: String, env: &Rc<RefCell<Env>>) -> Result<()> {
    fn inner(source: &str, env: &Rc<RefCell<Env>>) -> Result<Value> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.scan_tokens()?;
        let mut parser = Parser::new(tokens);
        let statements = parser.parse()?;

        let mut resolver = Resolver::default();
        resolver.resolve(&statements)?;

        let context = Rc::new(RefCell::new(Ctx::new(env, Rc::new(resolver))));

        let result = interpret(&context, &statements);
        match result {
            Ok(value) => Ok(value),
            Err(RuntimeError::Return(value)) => Ok(value),
            Err(RuntimeError::Report(report)) => Err(report),
        }
    }

    let result = inner(&source, env).map_err(|error| error.with_source_code(source.clone()))?;

    println!("{}", result);

    Ok(())
}

fn repl() -> anyhow::Result<()> {
    let mut rl = DefaultEditor::new()?;
    rl.load_history("history.txt").ok();
    let env = Env::global();
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

    let start = SystemTime::now();
    let result = run(source, &Env::global());
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("Execution {} ms", duration.as_millis());

    if let Err(err) = result {
        println!("{:?}", err);
    }

    Ok(())
}

fn immediate(code: String) -> anyhow::Result<()> {
    let result = run(code, &Env::global());
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
