use super::Peek;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    token,
    Expr,
};

#[derive(Debug, Clone)]
pub struct InlineRust {
    pub dollar: token::Dollar,
    pub brackets: token::Bracket,
    pub content: Expr,
}

impl Peek for InlineRust {
    fn peek(input: ParseStream) -> bool {
        input.peek(token::Dollar)
    }
}

impl Parse for InlineRust {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let dollar = input.parse()?;
        let content;
        let brackets = bracketed!(content in input);
        Ok(Self { dollar, brackets, content: content.parse()? })
    }
}

#[derive(Debug, Clone)]
pub enum Inlinable<T> {
    Plain(T),
    Inlined(InlineRust),
}

impl<T> Peek for Inlinable<T>
where
    T: Peek,
{
    fn peek(input: ParseStream) -> bool {
        T::peek(input) || InlineRust::peek(input)
    }
}

impl<T> Parse for Inlinable<T>
where
    T: Parse,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if InlineRust::peek(input) {
            Ok(Inlinable::Inlined(input.parse()?))
        } else {
            Ok(Inlinable::Plain(input.parse()?))
        }
    }
}
