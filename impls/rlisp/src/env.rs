// environment for our lisp interpreter
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::prims::*;
use crate::types::{Fun, MalVal};

struct Env(HashMap<u32, MalVal>); // newtype for environments

impl Env {
    pub fn new() -> Self {
        Env(HashMap::new())
    }

    pub fn get(&self, key: &u32) -> Option<&MalVal> {
        self.0.get(key)
    }

    pub fn set(&mut self, key: u32, val: MalVal) -> Option<MalVal> {
        self.0.insert(key, val)
    }
}

pub struct REnv {
    data: Env,
    outer: Env,
}

// TODO: use a weak pointer, we are highly likely to leak memory with Rc because
// circular references mean the reference count never gets to 0
pub struct ReplEnv(Rc<RefCell<REnv>>);

impl ReplEnv {
    pub fn new() -> Self {
        let mut env = ReplEnv(Rc::new(RefCell::new(REnv {
            data: Env::new(),
            outer: Env::new(),
        })));
        env.set(Fun::ADD, builtin_add);
        env.set(Fun::SUB, builtin_sub);
        env.set(Fun::DIV, builtin_div);
        env.set(Fun::MUL, builtin_mul);
    }

    pub fn set(&self, key: Fun, val: MalVal) -> Option<MalVal> {
        self.0.borrow_mut().data.set(key.0, val)
    }

    pub fn get(&self, key: &Fun) -> Option<MalVal> {
        match self.0.borrow_mut().data.get(key.0) {
            Some(val) => Some(val.clone()),
            None => match self.0.borrow_mut().outer.get(key) {
                Some(val) => Some(val.clone()),
                None => None,
            },
        }
    }
}
