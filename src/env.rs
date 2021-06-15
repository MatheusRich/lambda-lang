use super::LValue;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
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

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
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

    pub fn set(&mut self, name: &str, value: &LValue) -> Result<LValue, String> {
        let is_global_scope = self.is_root();
        let scope = self.lookup(name);

        if scope.is_none() && !is_global_scope {
            return Err(format!(
                "attempting to assign to undefined variable {}",
                name
            ));
        };

        let scope = match scope {
            Some(s) => s,
            None => self,
        };

        scope.vars.insert(name.into(), value.clone());

        Ok(value.clone())
    }

    pub fn def(&mut self, name: String, value: &LValue) {
        self.vars.insert(name, value.clone());
    }

    fn lookup(&mut self, name: &str) -> Option<&mut Env> {
        if self.vars.contains_key(name) {
            return Some(self);
        }

        match &mut self.parent {
            Some(env) => env.lookup(name),
            None => None,
        }
    }
}
