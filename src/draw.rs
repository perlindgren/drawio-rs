// drawio

use crate::xml::*;
use std::fs::File;
use std::path::PathBuf;

use std::io;
use std::io::prelude::*;
impl<'a> Element<'a> {
    fn new_draw_io(id: &'a str) -> Self {
        Element {
            id: "mxfile",
            data: vec![],
            inner: vec![Element {
                id: "diagram",
                data: vec![],
                inner: vec![Element {
                    id: "mxGraphModel",
                    data: vec![
                        Data("dx", "320"),
                        Data("dy", "200"),
                        Data("pageWidth", "320"),
                        Data("pageHeight", "200"),
                    ],
                    inner: vec![Element {
                        id: "root",
                        data: vec![],
                        inner: vec![
                            // Element {
                            //     id: "mxCell",
                            //     data: vec![Data("id", "0")],
                            //     inner: vec![],
                            // },
                            Element {
                                id: "mxCell",
                                data: vec![Data("id", "1") /* Data("parent", "0") */],
                                inner: vec![],
                            },
                            Element {
                                id: "mxCell",
                                data: vec![
                                    Data("id", "2"),
                                    Data("vertex", "1"),
                                    Data("parent", "1"),
                                ],
                                inner: vec![Element {
                                    id: "mxGeometry",
                                    data: vec![
                                        Data("x", "20"),
                                        Data("y", "20"),
                                        Data("width", "40"),
                                        Data("height", "80"),
                                        Data("as", "geometry"),
                                    ],
                                    inner: vec![],
                                }],
                            },
                            Element {
                                id: "mxCell",
                                data: vec![
                                    Data("id", "3"),
                                    Data("vertex", "1"),
                                    Data("parent", "1"),
                                ],
                                inner: vec![Element {
                                    id: "mxGeometry",
                                    data: vec![
                                        Data("x", "100"),
                                        Data("y", "20"),
                                        Data("width", "40"),
                                        Data("height", "80"),
                                        Data("as", "geometry"),
                                    ],
                                    inner: vec![],
                                }],
                            },
                        ],
                    }],
                }],
            }],
        }
    }

    fn save(self, path: &PathBuf) -> io::Result<()> {
        let mut file = File::create(&path)?;
        let io_str = format!("{}", self);
        file.write_all(io_str.as_bytes())
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_simple() {
        let io = Element::new_draw_io("hello world");
        println!("{}", io);
        io.save(&PathBuf::from_str("xml/out.drawio").unwrap())
            .unwrap();
    }
}
