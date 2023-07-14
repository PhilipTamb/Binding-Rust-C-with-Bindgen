use std::ffi::c_int;

use crate::bindings::{add};

mod bindings;

fn main() {
    unsafe{

        let result: c_int= add(3, 5);
        println!("result: {}",result);

    }
}
