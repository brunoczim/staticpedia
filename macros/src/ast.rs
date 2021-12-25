pub mod location;
pub mod inline;
pub mod blocking;
pub mod page;

use syn::parse::ParseStream;

pub trait Peek {
    fn peek(input: ParseStream) -> bool;
}
