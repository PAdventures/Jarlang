use std::ops::{Add, Div, Mul, Rem, Sub};

use crate::{
    frontend::ast,
    runtime::{
        environment::Environment,
        helpers,
        interpreter::{self, evaluate},
        values,
    },
};

pub fn evaluate_digit_binary_expression_result<T>(
    left_value: T,
    right_value: T,
    operator: String,
) -> Result<T, String>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Copy,
{
    match operator.as_str() {
        "-" => Ok(left_value - right_value),
        "*" => Ok(left_value * right_value),
        "+" => Ok(left_value + right_value),
        "/" => Ok(left_value / right_value),
        "%" => Ok(left_value % right_value),
        _ => Err(format!(
            "Unexpected operator found during binary expression evalutation, got: {}",
            operator
        )),
    }
}

pub fn evaluate_digit_binary_expression(
    left_hand_side: values::RuntimeValue,
    right_hand_side: values::RuntimeValue,
    operator: String,
) -> Result<values::RuntimeValue, String> {
    match (&left_hand_side, &right_hand_side) {
        (values::RuntimeValue::Integer8(lhs), values::RuntimeValue::Integer8(rhs)) => {
            let result = evaluate_digit_binary_expression_result(lhs.value, rhs.value, operator)?;
            Ok(values::Integer8Value::create(result).as_raw())
        }
        (values::RuntimeValue::Integer16(lhs), values::RuntimeValue::Integer16(rhs)) => {
            let result = evaluate_digit_binary_expression_result(lhs.value, rhs.value, operator)?;
            Ok(values::Integer16Value::create(result).as_raw())
        }
        (values::RuntimeValue::Integer32(lhs), values::RuntimeValue::Integer32(rhs)) => {
            let result = evaluate_digit_binary_expression_result(lhs.value, rhs.value, operator)?;
            Ok(values::Integer32Value::create(result).as_raw())
        }
        (values::RuntimeValue::Integer64(lhs), values::RuntimeValue::Integer64(rhs)) => {
            let result = evaluate_digit_binary_expression_result(lhs.value, rhs.value, operator)?;
            Ok(values::Integer64Value::create(result).as_raw())
        }
        (values::RuntimeValue::Integer128(lhs), values::RuntimeValue::Integer128(rhs)) => {
            let result = evaluate_digit_binary_expression_result(lhs.value, rhs.value, operator)?;
            Ok(values::Integer128Value::create(result).as_raw())
        }
        (
            values::RuntimeValue::UnsignedInteger8(lhs),
            values::RuntimeValue::UnsignedInteger8(rhs),
        ) => {
            let result = evaluate_digit_binary_expression_result(lhs.value, rhs.value, operator)?;
            Ok(values::UnsignedInteger8Value::create(result).as_raw())
        }
        (
            values::RuntimeValue::UnsignedInteger16(lhs),
            values::RuntimeValue::UnsignedInteger16(rhs),
        ) => {
            let result = evaluate_digit_binary_expression_result(lhs.value, rhs.value, operator)?;
            Ok(values::UnsignedInteger16Value::create(result).as_raw())
        }
        (
            values::RuntimeValue::UnsignedInteger32(lhs),
            values::RuntimeValue::UnsignedInteger32(rhs),
        ) => {
            let result = evaluate_digit_binary_expression_result(lhs.value, rhs.value, operator)?;
            Ok(values::UnsignedInteger32Value::create(result).as_raw())
        }
        (
            values::RuntimeValue::UnsignedInteger64(lhs),
            values::RuntimeValue::UnsignedInteger64(rhs),
        ) => {
            let result = evaluate_digit_binary_expression_result(lhs.value, rhs.value, operator)?;
            Ok(values::UnsignedInteger64Value::create(result).as_raw())
        }
        (
            values::RuntimeValue::UnsignedInteger128(lhs),
            values::RuntimeValue::UnsignedInteger128(rhs),
        ) => {
            let result = evaluate_digit_binary_expression_result(lhs.value, rhs.value, operator)?;
            Ok(values::UnsignedInteger128Value::create(result).as_raw())
        }
        (values::RuntimeValue::Float32(lhs), values::RuntimeValue::Float32(rhs)) => {
            let result = evaluate_digit_binary_expression_result(lhs.value, rhs.value, operator)?;
            Ok(values::Float32Value::create(result).as_raw())
        }
        (values::RuntimeValue::Float64(lhs), values::RuntimeValue::Float64(rhs)) => {
            let result = evaluate_digit_binary_expression_result(lhs.value, rhs.value, operator)?;
            Ok(values::Float64Value::create(result).as_raw())
        }
        _ => Err(format!(
            "Cannot perform the ({}) operator to a {:#?} and a {:#?}",
            operator,
            left_hand_side.as_value_type(),
            right_hand_side.as_value_type()
        )),
    }
}

pub fn evaluate_binary_expression(
    binary_expression: Box<ast::BinaryExpression>,
    environment: &mut Environment,
) -> Result<values::RuntimeValue, String> {
    let left_hand_side = match interpreter::evaluate(
        ast::Statement::Expression(binary_expression.left),
        environment,
    ) {
        Ok(runtime_val) => runtime_val,
        Err(m) => return Err(m),
    };
    let right_hand_side = match interpreter::evaluate(
        ast::Statement::Expression(binary_expression.right),
        environment,
    ) {
        Ok(runtime_val) => runtime_val,
        Err(m) => return Err(m),
    };
    if helpers::runtime_value_is_digit(&left_hand_side)
        && helpers::runtime_value_is_digit(&right_hand_side)
    {
        return evaluate_digit_binary_expression(
            left_hand_side,
            right_hand_side,
            binary_expression.operator,
        );
    }

    Ok(values::NullValue::create().as_raw())
}

pub fn evaluate_identifier_expression(
    ast_node: ast::IdentifierExpression,
    environment: &mut Environment,
) -> Result<values::RuntimeValue, String> {
    match environment.lookup_variable(ast_node.symbol.to_string()) {
        Some(value) => Ok(value),
        None => Err(format!("Variable \"{}\" does not exist", ast_node.symbol)),
    }
}

pub fn evaluate_assignment_expression(
    ast_node: Box<ast::VariableAssignmentExpression>,
    environment: &mut Environment,
) -> Result<values::RuntimeValue, String> {
    let assignee = match ast_node.assignee {
        ast::Expression::Identifier(identifier) => identifier,
        _ => {
            return Err(format!(
                "Invalid left hand side expression. Expected identifier, got {:#?}",
                &ast_node.assignee
            )
            .to_string())
        }
    };

    let variable_value = match environment.lookup_variable(assignee.symbol.to_owned()) {
        Some(runtime_value) => runtime_value,
        None => {
            return Err(format!(
                "Unknown variable: {} detected during variable assignment",
                assignee.symbol
            )
            .to_string())
        }
    };

    let new_variable_value = match evaluate(ast::Statement::Expression(ast_node.value), environment)
    {
        Ok(new_runtime_value) => match helpers::evaluate_variable_type(
            Some(ast::IdentifierExpression::create(
                variable_value.as_value_type().as_string(),
            )),
            assignee.symbol.to_owned(),
            new_runtime_value,
        ) {
            Ok(runtime_value) => runtime_value,
            Err(m) => return Err(m),
        },
        Err(m) => return Err(m),
    };

    match environment.assign_variable(assignee.symbol, new_variable_value.to_owned()) {
        Ok(_) => Ok(new_variable_value),
        Err(m) => return Err(m),
    }
}
