extern crate quote;

use proc_macro2::{Ident, Punct, Spacing, TokenStream};
use quote::{ToTokens, TokenStreamExt, format_ident, quote};

pub(crate) fn list_of<I>(iter :I) -> TokenStream 
where
    I: IntoIterator,
    I::Item: ToTokens,
{
    let mut ts = TokenStream::new();
    ts.append_separated(iter, Punct::new(',', Spacing::Joint));
    ts
}