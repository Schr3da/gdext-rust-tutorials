use std::sync::{LazyLock, Mutex};

use godot::prelude::*;

pub struct MySingleton {
    pub data: Vec<String>,
}

impl MySingleton {
    pub fn add(&mut self, d: String) {
        self.data.push(d);
    }

    pub fn print(&mut self) {
        let message = format!("{:?}", self.data);
        godot_warn!("{:?}", message);
    }
}

pub static EXAMPLE_SINGLETON: LazyLock<Mutex<MySingleton>> =
    LazyLock::new(|| Mutex::new(MySingleton { data: Vec::new() }));
