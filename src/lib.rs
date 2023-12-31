// example header

// <mxfile host="Electron" modified="2023-12-31T09:23:31.582Z" agent="Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) draw.io/22.1.16 Chrome/120.0.6099.109 Electron/28.1.0 Safari/537.36" etag="1o7Z_KPdkwODHp7ASC18" version="22.1.16" type="device">
//   <diagram name="Page-1" id="Gpl6vU_g16wYDfX3Skah">
//     <mxGraphModel dx="1098" dy="988" grid="1" gridSize="10" guides="1" tooltips="1" connect="1" arrows="1" fold="1" page="1" pageScale="1" pageWidth="827" pageHeight="1169" math="0" shadow="0">
//       <root>
//         <mxCell id="0" />
//         <mxCell id="1" parent="0" />
//         <mxCell id="pQSwlj2l13oX5aKklSoz-1" value="" style="rounded=0;whiteSpace=wrap;html=1;" vertex="1" parent="1">
//           <mxGeometry x="160" y="320" width="40" height="80" as="geometry" />
//         </mxCell>
//       </root>
//     </mxGraphModel>
//   </diagram>
// </mxfile>

// but a stripped down version like this is accepted as well:

// <mxfile>
//   <diagram>
//     <mxGraphModel dx="320" dy="200" grid="1" gridSize="10" guides="1" tooltips="1" connect="1" arrows="1" fold="1" page="1" pageScale="1" pageWidth="320" pageHeight="200" math="0" shadow="0">
//       <root>
//         <mxCell id="0" />
//         <mxCell id="1" parent="0" />
//         <mxCell id="-" value="" style="rounded=0;whiteSpace=wrap;html=1;" vertex="1" parent="1">
//           <mxGeometry x="20" y="20" width="40" height="80" as="geometry" />
//         </mxCell>
//       </root>
//     </mxGraphModel>
//   </diagram>
// </mxfile>

// this seems to be a minimal config for a box

// <mxfile>
//   <diagram>
//     <mxGraphModel pageWidth="320" pageHeight="200">
//       <root>
//         <mxCell vertex="1" >
//           <mxGeometry x="20" y="20" width="40" height="80" as="geometry" />
//         </mxCell>
//       </root>
//     </mxGraphModel>
//   </diagram>
// </mxfile>

// pub struct DrawIo {
//     dx: u32,
//     dy: u32,
//     grid: u32,
//     gridSize: u32,
//     guides: u32,
//     tooltips: u32,
//     connect: u32,
//     arrows: u32,
//     fold: u32,
//     page: u32,
//     pageScale: u32,
//     pageWidth: u32,
//     math: u32,
//     shadow: u32,
// }

#[derive(Debug, Clone, PartialEq)]
pub struct Data<'a>(&'a str, &'a str);

#[derive(Debug, Clone, PartialEq)]
pub struct Element<'a> {
    id: &'a str,
    data: Vec<Data<'a>>,
    inner: Vec<Element<'a>>,
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
        write!(f, "{}={:?} ", self.0, self.1)
    }
}

impl<'a> fmt::Display for Element<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data: String = self.data.iter().fold("".to_string(), |mut acc, d| {
            acc.push_str(&format!("{}", d));
            acc
        });
        writeln!(f, "<{} {}>", self.id, data)?;
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
        assert_eq!("x=\"13\" ", format!("{}", Data("x", "13")));
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
