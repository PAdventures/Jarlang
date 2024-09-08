use crate::{
    frontend::ast,
    runtime::{environment::Environment, helpers, interpreter, values},
};

pub fn evaluate_program(
    program: ast::ProgramStatement,
    environment: &mut Environment,
) -> Result<values::RuntimeValue, String> {
    let mut last_evaluated = values::NullValue::create().as_raw();

    for statment in program.body {
        last_evaluated = match interpreter::evaluate(statment, environment) {
            Ok(runtime_val) => runtime_val,
            Err(m) => return Err(m),
        }
    }

    Ok(last_evaluated)
}

pub fn evaluate_variable_declaration(
    variable_declaration: ast::VariableDeclarationStatement,
    environment: &mut Environment,
) -> Result<values::RuntimeValue, String> {
    let value = match variable_declaration.to_owned().value {
        Some(expression) => {
            match interpreter::evaluate(ast::Statement::Expression(expression), environment) {
                Ok(runtime_val) => {
                    match helpers::evaluate_variable_type(
                        variable_declaration.to_owned().value_type,
                        variable_declaration.to_owned().identifier,
                        runtime_val.to_owned(),
                    ) {
                        Ok(runtime_value) => runtime_value,
                        Err(m) => return Err(m),
                    }
                }
                Err(m) => return Err(m),
            }
        }
        None => values::NullValue::create().as_raw(),
    };

    match environment.declare_variable(
        variable_declaration.identifier,
        value.to_owned(),
        variable_declaration.constant,
    ) {
        Ok(_) => (),
        Err(m) => return Err(m),
    }

    Ok(value)
}
