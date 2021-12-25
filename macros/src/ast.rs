pub mod rust;
pub mod location;
pub mod inline;
pub mod blocking;
pub mod page;

use proc_macro2::TokenStream;
use syn::parse::ParseStream;

pub trait Peek {
    fn peek(input: ParseStream) -> bool;
}

pub trait Expand {
    fn expand(&self) -> TokenStream;
}
