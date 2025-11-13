use std::collections::HashMap;
use crate::runtime::Value;

/// Environment holds variables and whether they are constant.
///
/// name -> (value, is_const)
#[derive(Debug, Default)]
pub struct Environment {
    store: HashMap<String, (Value, bool)>,
}

impl Environment {
    pub fn new() -> Self {
        Self { store: HashMap::new() }
    }

    /// Define a new variable. Returns error if already exists and is const.
    pub fn define(&mut self, name: String, value: Value, is_const: bool) -> Result<(), String> {
        if let Some((_, existing_const)) = self.store.get(&name) {
            if *existing_const {
                return Err(format!("Cannot redefine constant '{}'", name));
            }
        }
        self.store.insert(name, (value, is_const));
        Ok(())
    }

    /// Assign to an existing variable. Error if it doesn't exist or is const.
    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if let Some(entry) = self.store.get_mut(name) {
            if entry.1 {
                return Err(format!("Cannot assign to constant '{}'", name));
            }
            entry.0 = value;
            Ok(())
        } else {
            Err(format!("Undefined variable '{}'", name))
        }
    }

    /// Get a variable's value.
    pub fn get(&self, name: &str) -> Option<Value> {
        self.store.get(name).map(|(v, _)| v.clone())
    }
}
