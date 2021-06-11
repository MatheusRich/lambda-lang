use super::LValue;
use std::collections::HashMap;

// #[derive(Clone)]
pub struct Env {
    vars: HashMap<String, LValue>,
    parent: Option<Box<Env>>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            vars: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_enclosing(enclosing: Self) -> Self {
        Env {
            vars: HashMap::new(),
            parent: Some(Box::new(enclosing)),
        }
    }

    pub fn get(&self, name: String) -> Result<LValue, String> {
        if self.vars.contains_key(&name) {
            return Ok(self.vars[&name].clone());
        }

        match &self.parent {
            Some(parent) => parent.get(name),
            None => Err(format!("undefined variable {}", name)),
        }
    }

    pub fn set(&mut self, name: String, value: &LValue) -> Result<LValue, String> {
        if self.vars.contains_key(&name) {
            self.vars.insert(name, value.clone());
            return Ok(value.clone());
        }

        match &mut self.parent {
            Some(parent) => parent.set(name, value),
            None => Err(format!(
                "attempting to assign to undefined variable {}",
                name
            )),
        }
    }

    pub fn def(&mut self, name: String, value: &LValue) {
        self.vars.insert(name, value.clone());
    }
}
