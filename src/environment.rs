use std::collections::HashMap;

use crate::ast::Ast;

#[derive(Clone)]
pub struct Environment {
    definitions: HashMap<String, Ast>,
    outer_environment: Option<Box<Environment>>,
}

impl Environment {
    pub fn get(&self, key: &String) -> Option<Ast> {
        match self.definitions.get(key) {
            Some(value) => Some(value.clone()),
            None => match &self.outer_environment {
                Some(outer_environment) => outer_environment.get(key),
                None => None,
            },
        }
    }

    pub fn new(outer_environment: Option<Box<Environment>>) -> Self {
        Environment {
            definitions: HashMap::new(),
            outer_environment,
        }
    }

    pub fn set(&mut self, key: String, value: Ast) {
    	self.definitions.insert(key, value);
    }

    pub fn set_list(&mut self, keys: Vec<String>, values:Vec<Ast>) -> bool {
    	if keys.len() != values.len() {
    		false
    	} else {
    		for (key, value) in keys.into_iter().zip(values.into_iter()) {
    			self.definitions.insert(key, value);
    		}
    		true
    	}
    }
}
