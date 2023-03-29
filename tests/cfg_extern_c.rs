#![allow(dead_code)]

use qualifier_attr::fn_qualifiers;

// We can add a qualifier to a function
// with an attribute.
#[fn_qualifiers(const)]
fn const_fn() -> u32 { 42 }

const CONST_RES: u32 = const_fn();

// It's not so impresive on its own
// but with cfg_attr it can be conditional.

#[no_mangle]
#[fn_qualifiers(pub(crate), extern "C")]
fn extern_c_fn() -> u32 { 42 }

fn main() {
    println!("Hello, world!");
}