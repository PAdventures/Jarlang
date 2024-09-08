use super::super::frontend::ast;
use super::environment::Environment;
use super::evaluation::{expressions, statements};
use super::values::{self};

pub fn evaluate(
    ast_node: ast::Statement,
    environment: &mut Environment,
) -> Result<values::RuntimeValue, String> {
    match ast_node {
        ast::Statement::Program(program) => statements::evaluate_program(program, environment),
        ast::Statement::VariableDeclaration(variable_declaration) => {
            statements::evaluate_variable_declaration(variable_declaration, environment)
        }
        ast::Statement::Expression(expression) => match expression {
            ast::Expression::Float(float) => {
                Ok(values::Float32Value::create(float.value.parse::<f32>().unwrap()).as_raw())
            }
            ast::Expression::Integer(integer) => {
                Ok(values::Integer32Value::create(integer.value.parse::<i32>().unwrap()).as_raw())
            }
            ast::Expression::Identifier(identifier) => {
                expressions::evaluate_identifier_expression(identifier, environment)
            }
            ast::Expression::Binary(binary_expression) => {
                expressions::evaluate_binary_expression(binary_expression, environment)
            }
            ast::Expression::VariableAssignment(variable_assignment_expression) => {
                expressions::evaluate_assignment_expression(
                    variable_assignment_expression,
                    environment,
                )
            }
            _ => {
                return Err(format!(
                    "This AST Node has not yet been implemented for interpretation: {:#?}",
                    expression
                ))
            }
        },
    }
}
