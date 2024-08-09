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

fn evaluate_variable_type(
    value_type: Option<ast::IdentifierExpression>,
    identifier: String,
    runtime_val: values::RuntimeValue,
) -> Result<values::RuntimeValue, String> {
    if value_type.is_none() {
        Ok(runtime_val)
    } else {
        match value_type.to_owned().unwrap().symbol.as_str() {
            "i8" => {
                if helpers::runtime_digit_is_i8(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_i8() {
                        Ok(integer8) => Ok(integer8.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "i16" => {
                if helpers::runtime_digit_is_i16(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_i16() {
                        Ok(integer16) => Ok(integer16.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "i32" => {
                if helpers::runtime_digit_is_i32(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_i32() {
                        Ok(integer32) => Ok(integer32.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "i64" => {
                if helpers::runtime_digit_is_i64(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_i64() {
                        Ok(integer64) => Ok(integer64.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "i128" => {
                if helpers::runtime_digit_is_i128(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_i128() {
                        Ok(integer128) => Ok(integer128.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "u8" => {
                if helpers::runtime_digit_is_u8(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_u8() {
                        Ok(unsigned_integer8) => Ok(unsigned_integer8.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "u16" => {
                if helpers::runtime_digit_is_u16(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_u16() {
                        Ok(unsigned_integer16) => Ok(unsigned_integer16.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "u32" => {
                if helpers::runtime_digit_is_u32(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_u32() {
                        Ok(unsigned_integer32) => Ok(unsigned_integer32.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "u64" => {
                if helpers::runtime_digit_is_u64(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_u64() {
                        Ok(unsigned_integer64) => Ok(unsigned_integer64.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "u128" => {
                if helpers::runtime_digit_is_u128(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_u128() {
                        Ok(unsigned_integer128) => Ok(unsigned_integer128.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "f32" => {
                if helpers::runtime_digit_is_f32(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_f32() {
                        Ok(float32) => Ok(float32.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "f64" => {
                if helpers::runtime_digit_is_f64(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_f64() {
                        Ok(float64) => Ok(float64.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "char" => {
                if helpers::runtime_is_char(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    return Err(format!(
                        "Incorrect runtime value for {}, expected: char, got: {:#?}",
                        identifier,
                        runtime_val.as_value_type()
                    ));
                }
            }
            "str" => {
                if helpers::runtime_is_str(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    return Err(format!(
                        "Incorrect runtime value for {}, expected: str, got: {:#?}",
                        identifier,
                        runtime_val.as_value_type()
                    ));
                }
            }
            "bool" => {
                if helpers::runtime_is_bool(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    return Err(format!(
                        "Incorrect runtime value for {}, expected: bool, got: {:#?}",
                        identifier,
                        runtime_val.as_value_type()
                    ));
                }
            }
            _ => {
                return Err(format!(
                    "Unexpected variable type given during variable declaration evaluation, got {}",
                    value_type.unwrap().symbol.as_str()
                ))
            }
        }
    }
}

pub fn evaluate_variable_declaration(
    variable_declaration: ast::VariableDeclarationStatement,
    environment: &mut Environment,
) -> Result<values::RuntimeValue, String> {
    let value = match variable_declaration.to_owned().value {
        Some(expression) => {
            match interpreter::evaluate(ast::Statement::Expression(expression), environment) {
                Ok(runtime_val) => {
                    match evaluate_variable_type(
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
