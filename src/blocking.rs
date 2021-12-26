use crate::render::Render;
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
