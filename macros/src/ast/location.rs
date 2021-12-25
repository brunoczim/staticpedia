use super::Peek;
use syn::{
    parse::{Error, Parse, ParseStream},
    token,
    LitStr,
};

#[derive(Debug, Clone)]
pub struct IdLoc {
    pub prefix: token::Pound,
    pub literal: LitStr,
}

impl Peek for IdLoc {
    fn peek(input: ParseStream) -> bool {
        input.peek(token::Pound)
    }
}

impl Parse for IdLoc {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self { prefix: input.parse()?, literal: input.parse()? })
    }
}

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

#[derive(Debug, Clone)]
pub enum Location {
    Id(IdLoc),
    Internal(InternalLoc),
    Url(Url),
}

impl Peek for Location {
    fn peek(input: ParseStream) -> bool {
        IdLoc::peek(input) || InternalLoc::peek(input) || Url::peek(input)
    }
}

impl Parse for Location {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if IdLoc::peek(input) {
            Ok(Location::Id(input.parse()?))
        } else if InternalLoc::peek(input) {
            Ok(Location::Internal(input.parse()?))
        } else if Url::peek(input) {
            Ok(Location::Url(input.parse()?))
        } else {
            Err(Error::new(input.span(), "Expected `#`, `/` or `@`"))
        }
    }
}
