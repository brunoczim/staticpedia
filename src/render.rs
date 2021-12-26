use crate::location::InternalPath;
use std::{borrow::Cow, fmt, rc::Rc, sync::Arc};

fn html_escape(ch: char) -> Option<&'static str> {
    match ch {
        '&' => Some("&amp;"),
        '<' => Some("&lt;"),
        '>' => Some("&gt;"),
        '"' => Some("&quot;"),
        '\'' => Some("&#39;"),
        '\\' => Some("&#92;"),
        _ => None,
    }
}

pub trait Render {
    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result;
}

impl<'this, T> Render for &'this T
where
    T: Render + ?Sized,
{
    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        (**self).to_html(fmt, ctx)
    }
}

impl<T> Render for Box<T>
where
    T: Render + ?Sized,
{
    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        (**self).to_html(fmt, ctx)
    }
}

impl<T> Render for Rc<T>
where
    T: Render + ?Sized,
{
    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        (**self).to_html(fmt, ctx)
    }
}

impl<T> Render for Arc<T>
where
    T: Render + ?Sized,
{
    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        (**self).to_html(fmt, ctx)
    }
}

impl<'cow, T> Render for Cow<'cow, T>
where
    T: Render + ToOwned + ?Sized,
    T::Owned: fmt::Debug,
{
    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        (**self).to_html(fmt, ctx)
    }
}

impl<T> Render for Vec<T>
where
    T: Render,
{
    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        for elem in self {
            elem.to_html(fmt, ctx)?;
        }
        Ok(())
    }
}

impl<T> Render for Option<T>
where
    T: Render,
{
    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        if let Some(component) = self {
            component.to_html(fmt, ctx)?;
        }
        Ok(())
    }
}

impl Render for str {
    fn to_html(&self, fmt: &mut fmt::Formatter, _ctx: Context) -> fmt::Result {
        let mut start = 0;
        let iter = self
            .char_indices()
            .filter_map(|(i, ch)| html_escape(ch).map(|s| (i, s)));

        for (end, escape) in iter {
            fmt.write_str(&self[start .. end])?;
            fmt.write_str(escape)?;
            start = end + 1;
        }

        fmt.write_str(&self[start ..])?;
        Ok(())
    }
}

impl Render for String {
    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        (**self).to_html(fmt, ctx)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Context<'loc> {
    location: &'loc InternalPath,
    /* site: &'site Site, */
}

impl<'loc> Context<'loc> {
    /// Creates the context from the current page's location and the given site.
    pub(crate) fn new(location: &'loc InternalPath) -> Self {
        Self { location }
    }

    /// The location of the current page.
    pub fn location(self) -> &'loc InternalPath {
        self.location
    }

    /// Creates a renderer over a component from this context. The `Display`
    /// trait can be used on the renderer.
    pub fn renderer<T>(self, target: T) -> Renderer<'loc, T>
    where
        T: Render,
    {
        Renderer { target, context: self }
    }
}

/// A renderer over a component. The `Display` trait can be used on the
/// renderer.
#[derive(Debug, Clone, Copy)]
pub struct Renderer<'loc, T>
where
    T: Render,
{
    /// The target component being rendered.
    pub target: T,
    /// The context at which the component will be rendered.
    pub context: Context<'loc>,
}

impl<'loc, T> fmt::Display for Renderer<'loc, T>
where
    T: Render,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.target.to_html(fmt, self.context)
    }
}
