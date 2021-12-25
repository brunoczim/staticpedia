use super::{blocking::BlockingComp, inline::InlineComp, Peek};
use syn::{
    bracketed,
    parse::{Error, Parse, ParseStream},
    punctuated::Punctuated,
    token,
    Ident,
    LitStr,
};

pub trait FieldType {
    type Value: Parse;

    fn name() -> &'static str;
}

#[derive(Debug, Clone)]
pub struct Field<T>
where
    T: FieldType,
{
    pub name: Ident,
    pub colon: token::Colon,
    pub value: T::Value,
}

impl<T> Peek for Field<T>
where
    T: FieldType,
{
    fn peek(input: ParseStream) -> bool {
        match input.fork().parse::<Ident>() {
            Ok(ident) if ident == T::name() => true,
            _ => false,
        }
    }
}

impl<T> Parse for Field<T>
where
    T: FieldType,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        if name == T::name() {
            Ok(Self { name, colon: input.parse()?, value: input.parse()? })
        } else {
            Err(Error::new(
                name.span(),
                format_args!("Expected `{}`", T::name()),
            ))
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IdField;

impl FieldType for IdField {
    type Value = LitStr;

    fn name() -> &'static str {
        "id"
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PageTitleField;

impl FieldType for PageTitleField {
    type Value = LitStr;

    fn name() -> &'static str {
        "title"
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SectionTitleField;

impl FieldType for SectionTitleField {
    type Value = InlineComp;

    fn name() -> &'static str {
        "title"
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BodyField;

impl FieldType for BodyField {
    type Value = Body;

    fn name() -> &'static str {
        "body"
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ChildrenField;

impl FieldType for ChildrenField {
    type Value = Children;

    fn name() -> &'static str {
        "children"
    }
}

#[derive(Debug, Clone)]
pub struct Page {
    pub title: Field<PageTitleField>,
    pub body: Field<BodyField>,
    pub children: Option<Field<ChildrenField>>,
}

impl Parse for Page {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut title = None;
        let mut body = None;
        let mut children = None;

        while !input.is_empty() {
            let key = input.parse::<Ident>()?;

            if key == PageTitleField::name() {
                if title.is_some() {
                    Err(Error::new(key.span(), "page title already declared"))?;
                }
                title = Some(input.parse()?);
            } else if key == BodyField::name() {
                if body.is_some() {
                    Err(Error::new(key.span(), "page body already declared"))?;
                }
                body = Some(input.parse()?);
            } else if key == ChildrenField::name() {
                if children.is_some() {
                    Err(Error::new(
                        key.span(),
                        "page children already declared",
                    ))?;
                }
                children = Some(input.parse()?);
            }
        }

        let title = title
            .ok_or_else(|| Error::new(input.span(), "missing page title"))?;
        let body =
            body.ok_or_else(|| Error::new(input.span(), "missing page body"))?;

        Ok(Self { title, body, children })
    }
}

#[derive(Debug, Clone)]
pub struct Section {
    pub id: Field<IdField>,
    pub title: Field<SectionTitleField>,
    pub body: Field<BodyField>,
    pub children: Option<Field<ChildrenField>>,
}

impl Parse for Section {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut id = None;
        let mut title = None;
        let mut body = None;
        let mut children = None;

        while !input.is_empty() {
            let key = input.parse::<Ident>()?;

            if key == IdField::name() {
                if id.is_some() {
                    Err(Error::new(key.span(), "section id already declared"))?;
                }
                id = Some(input.parse()?);
            } else if key == SectionTitleField::name() {
                if title.is_some() {
                    Err(Error::new(
                        key.span(),
                        "section title already declared",
                    ))?;
                }
                title = Some(input.parse()?);
            } else if key == BodyField::name() {
                if body.is_some() {
                    Err(Error::new(
                        key.span(),
                        "section body already declared",
                    ))?;
                }
                body = Some(input.parse()?);
            } else if key == ChildrenField::name() {
                if children.is_some() {
                    Err(Error::new(
                        key.span(),
                        "section children already declared",
                    ))?;
                }
                children = Some(input.parse()?);
            }
        }

        let id =
            id.ok_or_else(|| Error::new(input.span(), "missing section id"))?;
        let title = title
            .ok_or_else(|| Error::new(input.span(), "missing section title"))?;
        let body = body
            .ok_or_else(|| Error::new(input.span(), "missing section body"))?;

        Ok(Self { id, title, body, children })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Children {
    pub sections: Punctuated<Section, token::Bracket>,
}

impl Peek for Children {
    fn peek(input: ParseStream) -> bool {
        input.peek(token::Bracket)
    }
}

impl Parse for Children {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self { sections: Punctuated::new() };
        while Self::peek(input) {
            let content;
            let brackets = bracketed!(content in input);
            this.sections.push_punct(brackets);
            this.sections.push_value(content.parse()?);
        }
        Ok(this)
    }
}

#[derive(Debug, Clone)]
pub struct Body {
    pub terms: Punctuated<BlockingComp, token::Semi>,
}

impl Peek for Body {
    fn peek(input: ParseStream) -> bool {
        BlockingComp::peek(input)
    }
}

impl Parse for Body {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(token::Semi) {
            Ok(Self { terms: Punctuated::new() })
        } else {
            Ok(Self { terms: input.parse_terminated(BlockingComp::parse)? })
        }
    }
}
