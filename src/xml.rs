// XML data structures

#[derive(Debug, Clone, PartialEq)]
// pub struct Data<'a>(pub(crate) &'a str, pub(crate) &'a str);

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
    pub fn attr(&mut self, key: &str, value: &str) -> &mut Self {
        self.attributes.push((key.to_string(), value.to_string()));
        self
    }

    pub fn attr_u32(&mut self, key: &str, value: u32) -> &mut Self {
        self.attributes.push((key.to_string(), value.to_string()));
        self
    }

    // set style
    pub fn style(&mut self, key: &str, value: &str) -> &mut Self {
        self.style.push((key.to_string(), value.to_string()));
        self
    }

    // set style
    pub fn style_u32(&mut self, key: &str, value: u32) -> &mut Self {
        self.style.push((key.to_string(), value.to_string()));
        self
    }

    // set style
    pub fn style_f32(&mut self, key: &str, value: f32) -> &mut Self {
        self.style.push((key.to_string(), value.to_string()));
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
    pub fn attr(mut self, key: &str, value: &str) -> Self {
        self.data.attr(key, value);
        self
    }

    /// add attribute to Element, allows tail chaining
    pub fn attr_u32(mut self, key: &str, value: u32) -> Self {
        self.data.attr_u32(key, value);
        self
    }

    /// add style to Element, allows tail chaining
    pub fn style(mut self, key: &str, value: &str) -> Self {
        self.data.style(key, value);
        self
    }

    /// add style to Element, allows tail chaining
    pub fn style_u32(mut self, key: &str, value: u32) -> Self {
        self.data.style_u32(key, value);
        self
    }

    /// add style to Element, allows tail chaining
    pub fn style_f32(mut self, key: &str, value: f32) -> Self {
        self.data.style_f32(key, value);
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

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for attr in &self.attributes {
            write!(f, " {}={:?}", attr.0, attr.1)?;
        }
        if self.style.len() > 0 {
            write!(
                f,
                " style = \"{}\"",
                self.style
                    .iter()
                    .map(|(k, v)| format!("{}={};", k, v))
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

impl<'a> fmt::Display for Tag {
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
        data.attr("x", "13");

        println!("{}", data);
        assert_eq!(" x=\"13\"", format!("{}", data));
    }

    #[test]
    fn test_data_style() {
        let data = Data {
            attributes: vec![],
            style: vec![("x".to_string(), "13".to_string())],
        };

        println!("{}", data);
        // assert_eq!(
        //     " value=\"\" style=\"shape=mxgraph.basic.pie;startAngle=0;endAngle=0.75;\"",
        //     format!("{}", data)
        // );
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
                    .style("z", "0.0"),
            )
            .inner(Tag::new("inner2").attr("y", "42"));

        println!("{}", element);
    }
}
