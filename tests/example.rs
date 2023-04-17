#![allow(dead_code)]

#[macro_use]
extern crate qualifier_attr;

// We can add a qualifier to a function
// with an attribute.
#[qualifiers(const)]
fn const_fn() -> u32 {
    42
}

const CONST_RES: u32 = const_fn();

// It's not so impresive on its own
// but with cfg_attr it can be conditional.
#[cfg_attr(feature = "extern_c", no_mangle, qualifiers(pub, extern "C"))]
fn extern_c_fn() -> u32 {
    42
}

// It even works with types, `use` statements, and more!
mod foo {
    #[qualifiers(pub)]
    struct Foo {
        x: i32,
        y: i32,
    }
}

#[qualifiers(pub)]
use foo::Foo;

// Traits and `impl`s too!?
#[cfg_attr(feature = "unsafe_quux", qualifiers(unsafe))]
trait Quux {
    fn quux_the_thing();
}

#[cfg_attr(feature = "unsafe_quux", qualifiers(unsafe))]
impl Quux for Foo {
    fn quux_the_thing() {
        println!("The thing was quuxed.");
    }
}
