#![crate_type = "lib"];
#![crate_name = "simple_observables"];

pub mod traits;
pub mod structs;

use traits::*;

struct SimpleObserver {
    name: String,
}

impl SimpleObserver {
    fn new(name: &str) -> SimpleObserver {
        SimpleObserver { name: name.to_string() }
    }
}

impl Observer for SimpleObserver {
    type Observes = usize;

    fn update(&self, data: &usize) {
        println!("{}, {:?}", self.name, *data);
    }
}
