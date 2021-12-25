use super::{location::Location, rust, Peek};
use syn::{
    parenthesized,
    parse::{Error, Parse, ParseStream},
    token,
    Ident,
    LitStr,
};

#[derive(Debug, Clone)]
pub struct Component {
    pub terms: Vec<rust::Inlinable<ComponentTerm>>,
}

impl Peek for Component {
    fn peek(input: ParseStream) -> bool {
        ComponentTerm::peek(input) || input.peek(token::Paren)
    }
}

impl Parse for Component {
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
pub enum ComponentTerm {
    Text(Text),
    Location(Location),
    Bold(Bold),
    Italic(Italic),
    Preformatted(Preformatted),
    Link(Link),
}

impl Peek for ComponentTerm {
    fn peek(input: ParseStream) -> bool {
        Text::peek(input)
            || Location::peek(input)
            || Bold::peek(input)
            || Italic::peek(input)
            || Preformatted::peek(input)
            || Link::peek(input)
    }
}

impl Parse for ComponentTerm {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if Text::peek(input) {
            Ok(ComponentTerm::Text(input.parse()?))
        } else if Location::peek(input) {
            Ok(ComponentTerm::Location(input.parse()?))
        } else if Bold::peek(input) {
            Ok(ComponentTerm::Bold(input.parse()?))
        } else if Italic::peek(input) {
            Ok(ComponentTerm::Italic(input.parse()?))
        } else if Preformatted::peek(input) {
            Ok(ComponentTerm::Preformatted(input.parse()?))
        } else if Link::peek(input) {
            Ok(ComponentTerm::Link(input.parse()?))
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
    pub target: Component,
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
    pub target: Component,
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
    pub target: Component,
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
    pub target: Component,
    pub location: rust::Inlinable<Location>,
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
