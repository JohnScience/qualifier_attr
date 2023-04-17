extern crate proc_macro as pm;
extern crate proc_macro2 as pm2;

use quote::ToTokens;
use syn::Item;

use crate::{
    parse::{CommonItemConst, CommonItemFn, CommonItemStatic, CommonItemType, Qualifiers},
    util::Qualify,
};

mod parse;
mod util;

#[proc_macro_attribute]
pub fn qualifiers(meta: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    fn inner(meta: pm::TokenStream, input: pm::TokenStream) -> syn::Result<pm::TokenStream> {
        let qualifiers = syn::parse::<Qualifiers>(meta)?;

        // 1. CommonItemConst
        if let Ok(mut input) = syn::parse::<CommonItemConst>(input.clone()) {
            input.qualify().apply(qualifiers.clone())?;
            return Ok(input.into_token_stream().into());
        }

        // 2. CommonItemFn
        if let Ok(mut input) = syn::parse::<CommonItemFn>(input.clone()) {
            input.qualify().apply(qualifiers.clone())?;
            return Ok(input.into_token_stream().into());
        }

        // 3. CommonItemStatic
        if let Ok(mut input) = syn::parse::<CommonItemStatic>(input.clone()) {
            input.qualify().apply(qualifiers.clone())?;
            return Ok(input.into_token_stream().into());
        }

        // 4. CommonItemType
        if let Ok(mut input) = syn::parse::<CommonItemType>(input.clone()) {
            input.qualify().apply(qualifiers.clone())?;
            return Ok(input.into_token_stream().into());
        }

        // Fall back to standard Item
        let mut input = syn::parse::<Item>(input)?;
        input.qualify().apply(qualifiers)?;
        Ok(input.into_token_stream().into())
    }

    match inner(meta, input) {
        Ok(output) => output,
        Err(error) => error.into_compile_error().into(),
    }
}
