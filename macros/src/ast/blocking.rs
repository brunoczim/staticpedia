use super::{inline, rust, Peek};
use syn::{
    parse::{Error, Parse, ParseStream},
    Ident,
    LitStr,
};

#[derive(Debug, Clone)]
pub enum Component {
    Paragraph(Paragraph),
    Image(Image),
}

impl Peek for Component {
    fn peek(input: ParseStream) -> bool {
        Paragraph::peek(input) || Image::peek(input)
    }
}

impl Parse for Component {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if Paragraph::peek(input) {
            Ok(Component::Paragraph(input.parse()?))
        } else if Image::peek(input) {
            Ok(Component::Image(input.parse()?))
        } else {
            Err(Error::new(input.span(), "Expected `p` or `img`"))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Paragraph {
    pub prefix: Ident,
    pub content: inline::Component,
}

impl Paragraph {
    pub const PREFIX: &'static str = "p";
}

impl Peek for Paragraph {
    fn peek(input: ParseStream) -> bool {
        match input.fork().parse::<Ident>() {
            Ok(ident) if ident == Self::PREFIX => true,
            _ => false,
        }
    }
}

impl Parse for Paragraph {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let prefix = input.parse::<Ident>()?;
        if prefix == Self::PREFIX {
            Ok(Self { prefix, content: input.parse()? })
        } else {
            Err(Error::new(
                prefix.span(),
                format_args!("Expected `{}`", Self::PREFIX),
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    pub prefix: Ident,
    pub alt: rust::Inlinable<LitStr>,
    pub link: inline::Component,
}

impl Image {
    pub const PREFIX: &'static str = "img";
}

impl Peek for Image {
    fn peek(input: ParseStream) -> bool {
        match input.fork().parse::<Ident>() {
            Ok(ident) if ident == Self::PREFIX => true,
            _ => false,
        }
    }
}

impl Parse for Image {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let prefix = input.parse::<Ident>()?;
        if prefix == Self::PREFIX {
            Ok(Self { prefix, alt: input.parse()?, link: input.parse()? })
        } else {
            Err(Error::new(
                prefix.span(),
                format_args!("Expected `{}`", Self::PREFIX),
            ))
        }
    }
}
