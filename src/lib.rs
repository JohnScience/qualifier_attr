use proc_macro as pm;

mod fn_qualifiers;
mod struct_qualifiers;
mod mod_qualifiers;

#[proc_macro_attribute]
pub fn fn_qualifiers(meta: pm::TokenStream, func: pm::TokenStream) -> pm::TokenStream {
    fn_qualifiers::fn_qualifiers(meta, func)
}

#[proc_macro_attribute]
pub fn struct_qualifiers(meta: pm::TokenStream, item: pm::TokenStream) -> pm::TokenStream {
    struct_qualifiers::struct_qualifiers(meta, item)
}

#[proc_macro_attribute]
pub fn mod_qualifiers(meta: pm::TokenStream, item: pm::TokenStream) -> pm::TokenStream {
    mod_qualifiers::mod_qualifiers(meta, item)
}
