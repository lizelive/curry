extern crate proc_macro;
extern crate quote;

use proc_macro2::{Group, Ident, Punct, Spacing, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, LitInt, Token,
};

pub(crate) fn sequance_of<I>(iter: I) -> TokenStream
where
    I: IntoIterator,
    I::Item: ToTokens,
{
    let mut ts = TokenStream::new();
    ts.append_separated(iter, Punct::new(',', Spacing::Alone));
    ts
}

pub(crate) fn tuple<I>(iter: I) -> TokenStream
where
    I: IntoIterator,
    I::Item: ToTokens,
{
    let mut ts = TokenStream::new();
    ts.append_terminated(iter, Punct::new(',', Spacing::Alone));
    Group::new(proc_macro2::Delimiter::Parenthesis, ts).into_token_stream()
}

struct MakeTupleDef {
    size: usize,
    ident: Ident,
}

impl Parse for MakeTupleDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let size: LitInt = input.parse()?;
        input.parse::<Token![,]>()?;
        let ident = input.parse()?;
        let size = size.base10_parse()?;
        Ok(MakeTupleDef { size, ident })
    }
}

#[proc_macro]
pub fn make_tuple_helpers(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //let MakeTupleDef { size, ident } = syn::parse2(input).expect("not size");
    let MakeTupleDef { size, ident } = parse_macro_input!(input as MakeTupleDef);

    let args_idents: Vec<_> = (1..size + 1).map(|i| format_ident!("x{}", i)).collect();

    let type_idents: Vec<_> = (1..size + 1).map(|i| format_ident!("T{}", i)).collect();

    let args: Vec<_> = (0..size)
        .map(|i| {
            let arg_ident = args_idents[i].clone();
            let type_ident = type_idents[i].clone();
            quote!(#arg_ident:#type_ident)
        })
        .collect();

    let args_idents = tuple(args_idents);
    let type_out = tuple(&type_idents);
    let type_args = sequance_of(type_idents);
    let args = sequance_of(args);

    let trait_impl = quote! {
        /// a #size-tuple aka #ident 
        pub fn #ident<#type_args>(#args) -> #type_out{
            (
                (#args_idents)
            )
        }
    };
    trait_impl.into()
}

struct RangeArgs {
    count: usize,
    prefix: Ident,
}

impl Parse for RangeArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let count: LitInt = input.parse()?;
        let count = count.base10_parse()?;
        input.parse::<Token![,]>()?;
        let prefix = input.parse()?;

        Ok(Self { count, prefix })
    }
}



#[proc_macro]
pub fn symbol_tuple_range(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let RangeArgs { count, prefix } = parse_macro_input!(token_stream as RangeArgs);

    let values = (1..count + 1).map(|i| format_ident!("{}{}", prefix, i));
    tuple(values).into()
}
