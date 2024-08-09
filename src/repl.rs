use super::frontend::parser;
use super::runtime::environment::Environment;
use super::runtime::interpreter;
use super::runtime::values;

use std::borrow::BorrowMut;
use std::io::{self, BufRead, Write};
use std::process::exit;

pub fn start_session() -> Result<(), String> {
    println!("REPL v0.1");

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

    loop {
        print!("> ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => {
                println!("Failed to flush stdout");
                exit(1);
            }
        }

        let mut buffer = String::new();
        let mut handle = io::stdin().lock();
        match handle.read_line(&mut buffer) {
            Ok(n) => {
                if n <= 1 || buffer == "exit\n" {
                    return Ok(());
                }
            }
            Err(_) => {
                println!("Could't read line from stdin");
                exit(1);
            }
        }
        let program = match parser.produce_ast(buffer.as_str()) {
            Ok(program_statement) => program_statement,
            Err(m) => {
                println!("{}", m);
                exit(1);
            }
        };

        match interpreter::evaluate(program, environment.borrow_mut()) {
            Ok(runtime_val) => println!("{:#?}", runtime_val),
            Err(m) => {
                println!("{}", m);
                exit(1)
            }
        }
    }
}
