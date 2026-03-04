// environment for our lisp interpreter
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::types::MalVal;

struct Env(HashMap<String, MalVal>); // newtype for environments

impl Env {
    pub fn new() -> Self {
        Env(HashMap::new())
    }

    pub fn get(&self, key: &String) -> Option<&MalVal> {
        self.0.get(key)
    }

    pub fn set(&mut self, key: String, val: MalVal) -> Option<MalVal> {
        self.0.insert(key, val)
    }
}

pub struct REnv {
    data: Env,
    outer: Env,
}

pub struct ReplEnv(Rc<RefCell<REnv>>);

impl ReplEnv {
    pub fn new() -> Self {
        ReplEnv(Rc::new(RefCell::new(REnv {
            data: Env::new(),
            outer: Env::new(),
        })))
    }

    pub fn set(&self, key: String, val: MalVal) -> Option<MalVal> {
        self.0.borrow_mut().data.set(key, val)
    }

    pub fn get(&self, key: &String) -> Option<MalVal> {
        match self.0.borrow_mut().data.get(key) {
            Some(val) => Some(val.clone()),
            None => match self.0.borrow_mut().outer.get(key) {
                Some(val) => Some(val.clone()),
                None => None,
            },
        }
    }
}
