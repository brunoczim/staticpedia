use proc_macro2::Span;
use syn::{
    parenthesized,
    parse::{Error, Parse, ParseStream},
    punctuated::Punctuated,
    token,
    Ident,
    LitStr,
};

pub trait Peek {
    fn peek(input: ParseStream) -> bool;
}

#[derive(Debug, Clone)]
pub struct Page {
    pub title: LitStr,
    pub body: SectionBody,
    pub children: Vec<Section>,
}

/**
 *  title: "foo"
 *  body: p "ahsdahsdh"
 *  children:
 *  {
 *      id: "bar"
 *      title: i "Avocado"
 *      body: p "asjdjasdjajsdj"
 *      children:
 *      {
 *          id: "jaj"
 *          title: b i c "kak"
 *          body: p b i c "hahahahah"
 *      }
 *  }
 *  {
 *      id: "baz"
 *      title: c "Potato"
 *      body: p "hahaha"
 *  }
 */
#[derive(Debug, Clone)]
pub struct Section {
    pub id: LitStr,
    pub title: InlineComp,
    pub body: SectionBody,
    pub children: Vec<Section>,
}

impl Parse for Section {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut id = None;
        let mut title = None;
        let mut body = None;
        let mut children = None;

        while !input.is_empty() {
            let key = input.parse::<Ident>()?;
            if key == "id" {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct SectionBody {
    pub terms: Punctuated<BlockingComp, token::Semi>,
}

impl Peek for SectionBody {
    fn peek(input: ParseStream) -> bool {
        BlockingComp::peek(input)
    }
}

impl Parse for SectionBody {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(token::Semi) {
            Ok(Self { terms: Punctuated::new() })
        } else {
            Ok(Self { terms: input.parse_terminated(BlockingComp::parse)? })
        }
    }
}

#[derive(Debug, Clone)]
pub enum BlockingComp {
    Paragraph(Paragraph),
    Image(Image),
}

impl Peek for BlockingComp {
    fn peek(input: ParseStream) -> bool {
        Paragraph::peek(input) || Image::peek(input)
    }
}

impl Parse for BlockingComp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if Paragraph::peek(input) {
            Ok(BlockingComp::Paragraph(input.parse()?))
        } else if Image::peek(input) {
            Ok(BlockingComp::Image(input.parse()?))
        } else {
            Err(Error::new(input.span(), "Expected `p` or `img`"))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Paragraph {
    pub prefix: Ident,
    pub content: InlineComp,
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
    pub alt: LitStr,
    pub link: InlineComp,
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
