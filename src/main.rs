mod command;
mod frontend;
mod repl;
mod runtime;

use command::CLI;

use clap::Parser;
use frontend::parser;
use runtime::environment::Environment;
use runtime::{interpreter, values};
use std::borrow::BorrowMut;
use std::fs;
use std::process::exit;

fn run_file(path: &str) -> Result<(), String> {
    println!("Running {}", path);
    let buff: String;
    match fs::read_to_string(path) {
        Ok(source) => buff = source,
        Err(_) => {
            println!("Failed to read {}", path);
            exit(1);
        }
    }

    let mut parser = parser::Parser::new();
    let mut environment = Environment::create(None);

    environment.declare_variable(
        String::from("true"),
        values::BooleanValue::create(true).as_raw(),
        true,
    )?;
    environment.declare_variable(
        String::from("false"),
        values::BooleanValue::create(false).as_raw(),
        true,
    )?;
    environment.declare_variable(
        String::from("null"),
        values::NullValue::create().as_raw(),
        true,
    )?;

    let program = match parser.produce_ast(buff.as_str()) {
        Ok(program_statement) => program_statement,
        Err(m) => {
            println!("{}", m);
            exit(1);
        }
    };

    match interpreter::evaluate(program, environment.borrow_mut()) {
        Ok(runtime_val) => {
            println!("{:#?}", runtime_val);
            Ok(())
        }
        Err(m) => {
            println!("{}", m);
            exit(1)
        }
    }
}

fn main() {
    let cli = CLI::parse();

    if let Some(path) = cli.path.as_deref() {
        match run_file(path) {
            Ok(_) => exit(0),
            Err(m) => {
                println!("{}", m);
                exit(1)
            }
        }
    }

    match repl::start_session() {
        Ok(_) => (),
        Err(m) => {
            println!("{}", m);
            exit(1)
        }
    }
}
