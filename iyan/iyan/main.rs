extern crate actix;

use actix::prelude::*; 

fn main() {
    let system = System::new();
    let _ = system.run();
}
