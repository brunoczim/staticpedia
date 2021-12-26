use crate::render::{Context, Render};
use std::{borrow::Cow, fmt, rc::Rc, sync::Arc};

pub trait Component: Render {}

impl<'this, T> Component for &'this T where T: Component + ?Sized {}

impl<T> Component for Box<T> where T: Component + ?Sized {}

impl<T> Component for Rc<T> where T: Component + ?Sized {}

impl<T> Component for Arc<T> where T: Component + ?Sized {}

impl<'cow, T> Component for Cow<'cow, T>
where
    T: Component + ToOwned + ?Sized,
    T::Owned: fmt::Debug,
{
}

impl<T> Component for Vec<T> where T: Component {}

impl<T> Component for Option<T> where T: Component {}

impl Component for str {}

impl Component for String {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Bold<T>(pub T)
where
    T: Component;

impl<T> Render for Bold<T>
where
    T: Component,
{
    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {}
}
