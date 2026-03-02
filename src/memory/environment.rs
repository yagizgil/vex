use crate::ast::expr::LiteralValue;
use crate::utils::logger::error::ErrorCode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Environment {
    pub values: Vec<LiteralValue>,
    pub names: HashMap<String, usize>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            values: Vec::new(),
            names: HashMap::new(),
            enclosing: None,
        }))
    }

    pub fn with_enclosing(outer: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            values: Vec::new(),
            names: HashMap::new(),
            enclosing: Some(outer),
        }))
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        let index = self.values.len();
        self.values.push(value);
        self.names.insert(name, index);
    }

    pub fn get_global(&self, name: &str) -> LiteralValue {
        if let Some(&index) = self.names.get(name) {
            return self.values[index].clone();
        }

        if let Some(outer) = &self.enclosing {
            return outer.borrow().get_global(name);
        }

        vex_mem_panic!(
            ErrorCode::Memory,
            Some(format!("Global variable '{}' not found.", name))
        );
    }

    pub fn assign_global(&mut self, name: String, value: LiteralValue) {
        if let Some(&index) = self.names.get(&name) {
            self.values[index] = value;
            return;
        }

        if let Some(outer) = &self.enclosing {
            outer.borrow_mut().assign_global(name, value);
            return;
        }

        vex_mem_panic!(
            ErrorCode::Memory,
            Some(format!("Global variable '{}' not found.", name))
        );
    }

    pub fn get_at(&self, distance: usize, index: usize) -> LiteralValue {
        if distance == 0 {
            return self.values.get(index).cloned().unwrap_or_else(|| {
                vex_mem_panic!(
                    ErrorCode::Memory,
                    Some(format!(
                        "Local index out of bounds: index {} but len is {}",
                        index,
                        self.values.len()
                    ))
                );
            });
        }

        let ancestor = self.ancestor(distance);
        let ancestor_borrow = ancestor.borrow();
        ancestor_borrow
            .values
            .get(index)
            .cloned()
            .unwrap_or_else(|| {
                vex_mem_panic!(
                    ErrorCode::Memory,
                    Some(format!(
                        "Ancestor index out of bounds: distance {} index {} but len is {}",
                        distance,
                        index,
                        ancestor_borrow.values.len()
                    ))
                );
            })
    }

    pub fn assign_at(&mut self, distance: usize, index: usize, value: LiteralValue) {
        if distance == 0 {
            self.values[index] = value;
            return;
        }

        let ancestor = self.ancestor(distance);
        ancestor.borrow_mut().values[index] = value;
    }

    fn ancestor(&self, distance: usize) -> Rc<RefCell<Environment>> {
        let mut curr = self
            .enclosing
            .clone()
            .expect("Ancestor distance out of bounds");
        for _ in 1..distance {
            let next = curr
                .borrow()
                .enclosing
                .clone()
                .expect("Ancestor distance out of bounds");
            curr = next;
        }
        curr
    }
}
