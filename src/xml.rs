// XML data structures
// use std::convert::Into;
use std::fmt::Display;
// use std::string::ToString;
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Data {
    pub(crate) attributes: Vec<(String, String)>,
    pub(crate) style: Vec<(String, String)>,
}

impl Data {
    /// create a new Element
    pub fn new() -> Self {
        Data {
            attributes: vec![],
            style: vec![],
        }
    }

    // add attribute
    pub fn attr<T>(&mut self, key: &str, value: T) -> &mut Self
    where
        T: Display,
    {
        self.attributes.push((key.into(), value.to_string()));
        self
    }

    // set style
    pub fn style<T>(&mut self, key: &str, value: T) -> &mut Self
    where
        T: Display,
    {
        self.style.push((key.into(), value.to_string()));
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    pub(crate) id: String,
    pub(crate) data: Data,
    pub(crate) inner: Vec<Tag>,
}

/// builder pattern
impl Tag {
    /// create a new Element
    pub fn new(id: &str) -> Self {
        Tag {
            id: id.to_string(),
            data: Data::new(),
            inner: vec![],
        }
    }

    /// add attribute to Element, allows tail chaining
    pub fn attr<T>(mut self, key: &str, value: T) -> Self
    where
        T: Display,
    {
        self.data.attr(key, value);
        self
    }

    /// add style to Element, allows tail chaining
    pub fn style<T>(mut self, key: &str, value: T) -> Self
    where
        T: Display,
    {
        self.data.style(key, value);
        self
    }

    /// add inner to Element, allows tail chaining
    pub fn inner(mut self, inner: Tag) -> Self {
        self.inner.push(inner);
        self
    }

    /// add inner to Element, allows tail chaining
    pub fn inner_ref(&mut self, inner: Tag) {
        self.inner.push(inner);
    }
}

use indented::indented;
use std::fmt;
use std::fmt::Write;

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for attr in &self.attributes {
            write!(f, " {}={:?}", attr.0, attr.1)?;
        }
        if !self.style.is_empty() {
            write!(
                f,
                " style = \"{}\"",
                self.style.iter().fold(String::new(), |mut output, (k, v)| {
                    let _ = write!(output, "{}={};", k, v);
                    output
                })
            )?;
        }
        Ok(())
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "<{}{}>", self.id, self.data)?;
        for i in &self.inner {
            write!(f, "{}", indented(i))?;
        }
        writeln!(f, "</{}>", self.id)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_data_attributes() {
        let data = Data {
            attributes: vec![("x".to_string(), "13".to_string())],
            style: vec![],
        };

        println!("{}", data);
        assert_eq!(" x=\"13\"", format!("{}", data));
    }

    #[test]
    fn test_builder_data_attributes() {
        let mut data = Data::new();
        data.attr("x", "13").attr("y", 13);

        println!("{}", data);
        assert_eq!(" x=\"13\" y=\"13\"", format!("{}", data));
    }

    #[test]
    fn test_data_style() {
        let mut data = Data::new();
        data.style("x", "13").style("y", 13u32);

        println!("{}", data);
        assert_eq!(" style = \"x=13;y=13;\"", format!("{}", data));
    }

    #[test]
    fn test_element() {
        let element = Tag::new("elem")
            .attr("x", "13")
            .attr("y", "42")
            .inner(
                Tag::new("inner1")
                    .attr("x", "13")
                    .attr("y", "42")
                    .style("z", 0.0),
            )
            .inner(Tag::new("inner2").attr("y", "42"));

        println!("{}", element);
    }
}
