use std::collections::HashMap;

use crate::interpreter::NumValue;

pub struct SymbolTable {
    symbols: HashMap<String, NumValue>,
    parent: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbols: HashMap::new(),
            parent: None,
        }
    }

    pub fn insert(&mut self, name: String, symbol: NumValue) {
        self.symbols.insert(name, symbol);
    }

    pub fn remove(&mut self, name: String) {
        self.symbols.remove(&name);
    }

    pub fn get(&self, name: &str) -> Option<&NumValue> {
        if self.symbols.get(name).is_some() {
            return self.symbols.get(name);
        } else if self.parent.is_some() {
            return self.parent.as_ref().unwrap().get(name);
        } else {
            return None;
        }
    }

    fn get_mut(&mut self, name: &str) -> Option<&mut NumValue> {
        if self.symbols.get(name).is_some() {
            return self.symbols.get_mut(name);
        } else if self.parent.is_some() {
            return self.parent.as_mut().unwrap().get_mut(name);
        } else {
            return None;
        }
    }

    fn set_parent(&mut self, parent: Box<SymbolTable>) {
        self.parent = Some(parent);
    }
}
