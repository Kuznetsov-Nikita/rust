#![forbid(unsafe_code)]

use std::any::Any;
use std::collections::HashMap;

pub struct Context {
    map: HashMap<String, Box<dyn Any>>,
    singletons: HashMap<String, Box<dyn Any>>,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            singletons: HashMap::new(),
        }
    }

    pub fn insert<T: 'static, S: AsRef<str>>(&mut self, key: S, obj: T) {
        self.map.insert(key.as_ref().to_string(), Box::new(obj));
    }

    pub fn get<T: 'static>(&self, key: &str) -> &T {
        match self.map.get(key) {
            Some(value) => value.downcast_ref::<T>().unwrap(),
            None => panic!("There is not such key!"),
        }
    }

    pub fn insert_singletone<T: 'static>(&mut self, obj: T) {
        self.singletons
            .insert(std::any::type_name::<T>().to_string(), Box::new(obj));
    }

    pub fn get_singletone<T: 'static>(&self) -> &T {
        match self.singletons.get(std::any::type_name::<T>()) {
            Some(value) => value.downcast_ref::<T>().unwrap(),
            None => panic!("There is not such object!"),
        }
    }
}
