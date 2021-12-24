use proc_macro2::Span;
use syn::{
    parenthesized,
    parse::{Error, Parse, ParseStream},
    token,
    Ident,
    LitStr,
};

pub trait Peek {
    fn peek(input: ParseStream) -> bool;
}

#[derive(Debug, Clone)]
pub struct InlineComp {
    terms: Vec<InlineCompTerm>,
}

impl Peek for InlineComp {
    fn peek(input: ParseStream) -> bool {
        InlineCompTerm::peek(input)
    }
}

impl Parse for InlineComp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self { terms: Vec::new() };
        while InlineCompTerm::peek(input) {
            if input.peek(token::Paren) {
                let content;
                parenthesized!(content in input);
                let mut child = content.parse::<Self>()?;
                this.terms.append(&mut child.terms);
            } else {
                this.terms.push(input.parse()?);
            }
        }
        Ok(this)
    }
}

#[derive(Debug, Clone)]
pub enum InlineCompTerm {
    Text(Text),
    Location(Location),
    Bold(Bold),
    Italic(Italic),
    Preformatted(Preformatted),
    Link(Link),
}

impl Peek for InlineCompTerm {
    fn peek(input: ParseStream) -> bool {
        Text::peek(input)
            || Location::peek(input)
            || Bold::peek(input)
            || Italic::peek(input)
            || Preformatted::peek(input)
            || Link::peek(input)
    }
}

impl Parse for InlineCompTerm {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if Text::peek(input) {
            Ok(InlineCompTerm::Text(input.parse()?))
        } else if Location::peek(input) {
            Ok(InlineCompTerm::Location(input.parse()?))
        } else if Bold::peek(input) {
            Ok(InlineCompTerm::Bold(input.parse()?))
        } else if Italic::peek(input) {
            Ok(InlineCompTerm::Italic(input.parse()?))
        } else if Preformatted::peek(input) {
            Ok(InlineCompTerm::Preformatted(input.parse()?))
        } else if Link::peek(input) {
            Ok(InlineCompTerm::Link(input.parse()?))
        } else {
            Err(Error::new(
                input.span(),
                "Expected string literal, `#`, `/`, `@`, `b`, `i`, `c` or `l`",
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Text {
    pub literal: LitStr,
}

impl Peek for Text {
    fn peek(input: ParseStream) -> bool {
        input.peek(LitStr)
    }
}

impl Parse for Text {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self { literal: input.parse()? })
    }
}

#[derive(Debug, Clone)]
pub struct Bold {
    pub prefix: Ident,
    pub target: InlineComp,
}

impl Peek for Bold {
    fn peek(input: ParseStream) -> bool {
        input.peek(|_| Ident::new("b", Span::mixed_site()))
    }
}

impl Parse for Bold {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if Self::peek(input) {
            Ok(Self { prefix: input.parse()?, target: input.parse()? })
        } else {
            Err(Error::new(input.span(), "Expected `b`"))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Italic {
    pub prefix: Ident,
    pub target: InlineComp,
}

impl Peek for Italic {
    fn peek(input: ParseStream) -> bool {
        input.peek(|_| Ident::new("i", Span::mixed_site()))
    }
}

impl Parse for Italic {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if Self::peek(input) {
            Ok(Self { prefix: input.parse()?, target: input.parse()? })
        } else {
            Err(Error::new(input.span(), "Expected `i`"))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Preformatted {
    pub prefix: Ident,
    pub target: InlineComp,
}

impl Peek for Preformatted {
    fn peek(input: ParseStream) -> bool {
        input.peek(|_| Ident::new("c", Span::mixed_site()))
    }
}

impl Parse for Preformatted {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if Self::peek(input) {
            Ok(Self { prefix: input.parse()?, target: input.parse()? })
        } else {
            Err(Error::new(input.span(), "Expected `c`"))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Link {
    pub prefix: Ident,
    pub target: InlineComp,
    pub location: Location,
}

impl Peek for Link {
    fn peek(input: ParseStream) -> bool {
        input.peek(|_| Ident::new("l", Span::mixed_site()))
    }
}

impl Parse for Link {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if Self::peek(input) {
            Ok(Self {
                prefix: input.parse()?,
                target: input.parse()?,
                location: input.parse()?,
            })
        } else {
            Err(Error::new(input.span(), "Expected `l`"))
        }
    }
}

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
