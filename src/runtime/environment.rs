use super::values;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment {
    parent: Option<Box<Environment>>,
    variables: HashMap<String, values::RuntimeValue>,
    constants: Vec<String>,
}

impl Environment {
    pub fn create(parent: Option<Box<Environment>>) -> Self {
        Environment {
            parent,
            variables: HashMap::new(),
            constants: Vec::new(),
        }
    }

    pub fn init_global_scope(&mut self) -> Result<(), String> {
        match self.parent {
            Some(_) => return Err("Cannot create global variables in a child scope".to_string()),
            None => (),
        }

        self.declare_variable(
            String::from("true"),
            values::BooleanValue::create(true).as_raw(),
            true,
        )?;
        self.declare_variable(
            String::from("false"),
            values::BooleanValue::create(false).as_raw(),
            true,
        )?;
        self.declare_variable(
            String::from("null"),
            values::NullValue::create().as_raw(),
            true,
        )?;

        Ok(())
    }

    pub fn declare_variable(
        &mut self,
        name: String,
        value: values::RuntimeValue,
        constant: bool,
    ) -> Result<(), String> {
        match self.variables.get(&name) {
            Some(_) => {
                return Err(format!(
                    "Cannot declare variable {} as it has already been defined",
                    name
                ))
            }
            None => (),
        }

        self.variables.insert(name.to_owned(), value);

        if constant {
            self.constants.push(name)
        }

        Ok(())
    }

    pub fn assign_variable(
        &mut self,
        name: String,
        value: values::RuntimeValue,
    ) -> Result<(), String> {
        let environment = match self.resolve_variable(name.to_string()) {
            Ok(env) => env,
            Err(m) => return Err(m),
        };

        if environment.constants.contains(&name) {
            return Err(format!(
                "Cannot reasign to variable {} as it has been declared constant",
                name
            ));
        }

        environment.variables.insert(name, value);

        Ok(())
    }

    pub fn lookup_variable(&mut self, name: String) -> Option<values::RuntimeValue> {
        let environment = match self.resolve_variable(name.to_string()) {
            Ok(env) => env,
            Err(_) => return None,
        };

        match environment.variables.get(&name) {
            Some(v) => Some(v.to_owned()),
            None => None,
        }
    }

    pub fn resolve_variable(&mut self, variable_name: String) -> Result<&mut Environment, String> {
        match self.variables.get(&variable_name) {
            Some(_) => return Ok(self),
            None => (),
        }
        match self.parent {
            Some(_) => (),
            None => {
                return Err(format!(
                    "Cannot resolve {} as it does not exist",
                    variable_name
                ))
            }
        }

        match self
            .parent
            .as_mut()
            .unwrap()
            .resolve_variable(variable_name)
        {
            Ok(env) => Ok(env),
            Err(m) => Err(m),
        }
    }
}
