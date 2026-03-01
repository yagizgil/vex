use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::ast::expr::LiteralValue;

#[derive(Clone, Debug)]
pub struct Environment {
    pub values: HashMap<String, LiteralValue>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            values: HashMap::new(),
            enclosing: None,
        }))
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> LiteralValue {
        if let Some(value) = self.values.get(name) {
            return value.clone();
        }
        if let Some(ref outer) = self.enclosing {
            return outer.borrow().get(name);
        }
        panic!("Undefined variable '{}'.", name);
    }

    pub fn assign(&mut self, name: String, value: LiteralValue) {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            return;
        }
        if let Some(ref mut outer) = self.enclosing {
            outer.borrow_mut().assign(name, value);
            return;
        }
        panic!("'{}' is not defined", name);
    }
}