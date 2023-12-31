// XML data structures

#[derive(Debug, Clone, PartialEq)]
// pub struct Data<'a>(pub(crate) &'a str, pub(crate) &'a str);

pub struct Data<'a> {
    pub(crate) attributes: Vec<(&'a str, &'a str)>,
    pub(crate) style: Style,
}

impl<'a> Data<'a> {
    /// create a new Element
    pub fn new() -> Self {
        Data {
            attributes: vec![],
            style: Style::No,
        }
    }

    // add attribute
    pub fn attr(&mut self, key: &'a str, value: &'a str) -> &mut Self {
        self.attributes.push((key, value));
        self
    }

    // set style
    pub fn style(&mut self, style: Style) -> &mut Self {
        self.style = style;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Style {
    Pie { start_angle: f32, end_angle: f32 },
    No,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Element<'a> {
    pub(crate) id: &'a str,
    pub(crate) data: Data<'a>,
    pub(crate) inner: Vec<Element<'a>>,
}

/// builder pattern
impl<'a> Element<'a> {
    /// create a new Element
    pub fn new(id: &'a str) -> Self {
        Element {
            id,
            data: Data::new(),
            inner: vec![],
        }
    }

    /// add attribute to Element, allows tail chaining
    pub fn attr(mut self, key: &'a str, value: &'a str) -> Self {
        self.data.attr(key, value);
        self
    }

    /// add style to Element, allows tail chaining
    pub fn style(mut self, style: Style) -> Self {
        self.data.style(style);
        self
    }

    /// add inner to Element, allows tail chaining
    pub fn inner(mut self, inner: Element<'a>) -> Self {
        self.inner.push(inner);
        self
    }
}

use indented::indented;
use std::fmt;

impl<'a> fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Style::Pie {
                start_angle,
                end_angle,
            } => write!(
                f,
                " value=\"\" style=\"shape=mxgraph.basic.pie;startAngle={};endAngle={};\"",
                start_angle, end_angle
            ),
            Style::No => Ok(()),
        }
    }
}

impl<'a> fmt::Display for Data<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for attr in &self.attributes {
            write!(f, " {}={:?}", attr.0, attr.1)?;
        }
        write!(f, "{}", self.style)
    }
}

impl<'a> fmt::Display for Element<'a> {
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
            attributes: vec![("x", "13")],
            style: Style::No,
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
            style: Style::Pie {
                start_angle: 0.0,
                end_angle: 0.75,
            },
        };

        println!("{}", data);
        assert_eq!(
            " value=\"\" style=\"shape=mxgraph.basic.pie;startAngle=0;endAngle=0.75;\"",
            format!("{}", data)
        );
    }

    #[test]
    fn test_element() {
        let element = Element::new("elem")
            .attr("x", "13")
            .attr("y", "42")
            .inner(
                Element::new("inner1")
                    .attr("x", "13")
                    .attr("y", "42")
                    .style(Style::Pie {
                        start_angle: 0.0,
                        end_angle: 0.75,
                    }),
            )
            .inner(Element::new("inner2").attr("y", "42"));

        println!("{}", element);
    }
}
