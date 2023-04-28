extern crate proc_macro as pm;
extern crate proc_macro2 as pm2;

use quote::ToTokens;
use syn::Item;

use crate::{
    parse::{FlexibleItemConst, FlexibleItemFn, FlexibleItemStatic, FlexibleItemType, Qualifiers},
    util::Qualify,
};

mod parse;
mod util;

#[proc_macro_attribute]
pub fn qualifiers(meta: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    fn inner(meta: pm::TokenStream, input: pm::TokenStream) -> syn::Result<pm::TokenStream> {
        let qualifiers = syn::parse::<Qualifiers>(meta)?;

        // Try "flexible" items first.
        if let Ok(mut input) = syn::parse::<FlexibleItemConst>(input.clone()) {
            input.qualify().apply(qualifiers.clone())?;
            return Ok(input.into_token_stream().into());
        }

        if let Ok(mut input) = syn::parse::<FlexibleItemFn>(input.clone()) {
            input.qualify().apply(qualifiers.clone())?;
            return Ok(input.into_token_stream().into());
        }

        if let Ok(mut input) = syn::parse::<FlexibleItemStatic>(input.clone()) {
            input.qualify().apply(qualifiers.clone())?;
            return Ok(input.into_token_stream().into());
        }

        if let Ok(mut input) = syn::parse::<FlexibleItemType>(input.clone()) {
            input.qualify().apply(qualifiers.clone())?;
            return Ok(input.into_token_stream().into());
        }

        // Fallback to normal items.
        let mut input = syn::parse::<Item>(input)?;
        input.qualify().apply(qualifiers)?;
        Ok(input.into_token_stream().into())
    }

    match inner(meta, input) {
        Ok(output) => output,
        Err(error) => error.into_compile_error().into(),
    }
}
