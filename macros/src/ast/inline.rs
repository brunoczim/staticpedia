use super::{location::Location, Peek};
use syn::{
    parenthesized,
    parse::{Error, Parse, ParseStream},
    token,
    Ident,
    LitStr,
};

#[derive(Debug, Clone)]
pub struct InlineComp {
    pub terms: Vec<InlineCompTerm>,
}

impl Peek for InlineComp {
    fn peek(input: ParseStream) -> bool {
        InlineCompTerm::peek(input) || input.peek(token::Paren)
    }
}

impl Parse for InlineComp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self { terms: Vec::new() };
        while Self::peek(input) {
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

impl Bold {
    pub const PREFIX: &'static str = "b";
}

impl Peek for Bold {
    fn peek(input: ParseStream) -> bool {
        match input.fork().parse::<Ident>() {
            Ok(ident) if ident == Self::PREFIX => true,
            _ => false,
        }
    }
}

impl Parse for Bold {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let prefix = input.parse::<Ident>()?;
        if prefix == Self::PREFIX {
            Ok(Self { prefix, target: input.parse()? })
        } else {
            Err(Error::new(
                prefix.span(),
                format_args!("Expected `{}`", Self::PREFIX),
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Italic {
    pub prefix: Ident,
    pub target: InlineComp,
}

impl Italic {
    pub const PREFIX: &'static str = "i";
}

impl Peek for Italic {
    fn peek(input: ParseStream) -> bool {
        match input.fork().parse::<Ident>() {
            Ok(ident) if ident == Self::PREFIX => true,
            _ => false,
        }
    }
}

impl Parse for Italic {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let prefix = input.parse::<Ident>()?;
        if prefix == Self::PREFIX {
            Ok(Self { prefix, target: input.parse()? })
        } else {
            Err(Error::new(
                prefix.span(),
                format_args!("Expected `{}`", Self::PREFIX),
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Preformatted {
    pub prefix: Ident,
    pub target: InlineComp,
}

impl Preformatted {
    pub const PREFIX: &'static str = "c";
}

impl Peek for Preformatted {
    fn peek(input: ParseStream) -> bool {
        match input.fork().parse::<Ident>() {
            Ok(ident) if ident == Self::PREFIX => true,
            _ => false,
        }
    }
}

impl Parse for Preformatted {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let prefix = input.parse::<Ident>()?;
        if prefix == Self::PREFIX {
            Ok(Self { prefix, target: input.parse()? })
        } else {
            Err(Error::new(
                prefix.span(),
                format_args!("Expected `{}`", Self::PREFIX),
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Link {
    pub prefix: Ident,
    pub target: InlineComp,
    pub location: Location,
}

impl Link {
    pub const PREFIX: &'static str = "l";
}

impl Peek for Link {
    fn peek(input: ParseStream) -> bool {
        match input.fork().parse::<Ident>() {
            Ok(ident) if ident == Self::PREFIX => true,
            _ => false,
        }
    }
}

impl Parse for Link {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let prefix = input.parse::<Ident>()?;
        if prefix == Self::PREFIX {
            Ok(Self {
                prefix,
                target: input.parse()?,
                location: input.parse()?,
            })
        } else {
            Err(Error::new(
                prefix.span(),
                format_args!("Expected `{}`", Self::PREFIX),
            ))
        }
    }
}
