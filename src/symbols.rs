use std::{collections::{HashMap, hash_map::Entry}};

pub struct SymTab {
    table: HashMap<String, Symbol>
}
impl SymTab {
    pub fn init() -> Self {
        Self {
            table: HashMap::new()
        }
    }

    pub fn put(&mut self, name: String, sym: Symbol) {
        self.table.insert(name, sym);
    }

    pub fn extis(&mut self, name: &String) -> bool {
        self.table.contains_key(name)
    }

    pub fn entry(&mut self, name: &String) -> Entry<String, Symbol>{
      self.table.entry(name.to_string())
    }

    pub fn get(&mut self, name: &String) -> &Symbol{
      self.table.get(name).unwrap()
    }
}

pub struct Symbol {
    pub name: String,
    pub reg: Option<String>,
}

impl Symbol {
    pub fn new(name: String) -> Self {
        Self{
            name,reg: None
        }
    }
}

pub struct Scope {
    
}