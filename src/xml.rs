// XML data structures

#[derive(Debug, Clone, PartialEq)]
pub struct Data<'a>(pub(crate) &'a str, pub(crate) &'a str);

#[derive(Debug, Clone, PartialEq)]
pub struct Element<'a> {
    pub(crate) id: &'a str,
    pub(crate) data: Vec<Data<'a>>,
    pub(crate) inner: Vec<Element<'a>>,
}

/// builder pattern
impl<'a> Element<'a> {
    /// create a new Element
    pub fn new(id: &'a str) -> Self {
        Element {
            id,
            data: vec![],
            inner: vec![],
        }
    }

    /// add data to Element, allow tail chaining
    pub fn add_data(&mut self, data: Data<'a>) -> &mut Self {
        self.data.push(data);
        self
    }

    /// add inner to Element, allows tail chaining
    pub fn add_inner(&mut self, inner: Element<'a>) -> &mut Self {
        self.inner.push(inner);
        self
    }
}

use indented::indented;
use std::fmt;

impl<'a> fmt::Display for Data<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {}={:?}", self.0, self.1)
    }
}

impl<'a> fmt::Display for Element<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data: String = self.data.iter().fold("".to_string(), |mut acc, d| {
            acc.push_str(&format!("{}", d));
            acc
        });
        writeln!(f, "<{}{}>", self.id, data)?;
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
    fn test_data() {
        println!("{}", Data("x", "13"));
        assert_eq!(" x=\"13\"", format!("{}", Data("x", "13")));
    }

    #[test]
    fn test_element() {
        let element = Element {
            id: "elem",
            data: vec![Data("x", "13"), Data("y", "42")],
            inner: vec![
                Element {
                    id: "inner1",
                    data: vec![Data("x", "13"), Data("y", "42")],
                    inner: vec![],
                },
                Element {
                    id: "inner2",
                    data: vec![Data("x", "42"), Data("y", "13")],
                    inner: vec![],
                },
            ],
        };
        println!("{}", element);
    }

    #[test]
    fn test_builder() {
        let mut e = Element::new("elem");
        e.add_data(Data("x", "13")).add_data(Data("y", "42"));
        e.add_inner(Element {
            id: "inner1",
            data: vec![Data("x", "13"), Data("y", "42")],
            inner: vec![],
        })
        .add_inner(Element {
            id: "inner2",
            data: vec![Data("x", "42"), Data("y", "13")],
            inner: vec![],
        });

        assert_eq!(
            e,
            Element {
                id: "elem",
                data: vec![Data("x", "13"), Data("y", "42")],
                inner: vec![
                    Element {
                        id: "inner1",
                        data: vec![Data("x", "13"), Data("y", "42")],
                        inner: vec![],
                    },
                    Element {
                        id: "inner2",
                        data: vec![Data("x", "42"), Data("y", "13")],
                        inner: vec![],
                    },
                ],
            }
        );

        println!("{}", e);
    }
}
