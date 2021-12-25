use super::{Expand, Peek};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Error, Parse, ParseStream},
    token,
    LitStr,
};

#[derive(Debug, Clone)]
pub struct InternalLoc {
    pub prefix: token::Div,
    pub literal: LitStr,
}

impl Peek for InternalLoc {
    fn peek(input: ParseStream) -> bool {
        input.peek(token::Div)
    }
}

impl Parse for InternalLoc {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self { prefix: input.parse()?, literal: input.parse()? })
    }
}

impl Expand for InternalLoc {
    fn expand(&self) -> TokenStream {
        let lit = &self.literal;
        quote! {
            staticpedia::location::Location::internal(#lit)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Url {
    pub prefix: token::At,
    pub literal: LitStr,
}

impl Peek for Url {
    fn peek(input: ParseStream) -> bool {
        input.peek(token::At)
    }
}

impl Parse for Url {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self { prefix: input.parse()?, literal: input.parse()? })
    }
}

impl Expand for Url {
    fn expand(&self) -> TokenStream {
        let lit = &self.literal;
        quote! {
            staticpedia::location::Location::url(#lit)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Location {
    Internal(InternalLoc),
    Url(Url),
}

impl Peek for Location {
    fn peek(input: ParseStream) -> bool {
        InternalLoc::peek(input) || Url::peek(input)
    }
}

impl Parse for Location {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if InternalLoc::peek(input) {
            Ok(Location::Internal(input.parse()?))
        } else if Url::peek(input) {
            Ok(Location::Url(input.parse()?))
        } else {
            Err(Error::new(input.span(), "Expected `/` or `@`"))
        }
    }
}

impl Expand for Location {
    fn expand(&self) -> TokenStream {
        match self {
            Location::Internal(loc) => loc.expand(),
            Location::Url(loc) => loc.expand(),
        }
    }
}
