// drawio

use crate::xml::*;
use std::fs::File;
use std::path::PathBuf;

use srp::common::*;

use std::io;
use std::io::prelude::*;
impl<'a> Element<'a> {
    fn new_draw_io(id: &'a str) -> Self {
        Element::new("mxfile").inner(Element::new("mxfile").inner(Element::new("diagram")))

        // id: "mxfile",
        // data: vec![],
        // inner: vec![Element {
        //     id: "diagram",
        //     data: vec![],
        //     inner: vec![Element {
        //         id: "mxGraphModel",
        //         data: vec![
        //             Data("dx", "320"),
        //             Data("dy", "200"),
        //             Data("pageWidth", "320"),
        //             Data("pageHeight", "200"),
        //         ],
        //         inner: vec![Element {
        //             id: "root",
        //             data: vec![],
        //             inner: vec![
        //                 Element {
        //                     id: "mxCell",
        //                     data: vec![Data("id", "1")],
        //                     inner: vec![],
        //                 },
        //                 Element {
        //                     id: "mxCell",
        //                     data: vec![
        //                         Data("id", "2"),
        //                         Data("vertex", "1"),
        //                         Data("parent", "1"),
        //                     ],
        //                     inner: vec![Element {
        //                         id: "mxGeometry",
        //                         data: vec![
        //                             Data("x", "20"),
        //                             Data("y", "20"),
        //                             Data("width", "40"),
        //                             Data("height", "80"),
        //                             Data("as", "geometry"),
        //                         ],
        //                         inner: vec![],
        //                     }],
        //                 },
        //                 Element {
        //                     id: "mxCell",
        //                     data: vec![
        //                         Data("id", "3"),
        //                         Data("vertex", "1"),
        //                         Data("parent", "1"),
        //                     ],
        //                     inner: vec![Element {
        //                         id: "mxGeometry",
        //                         data: vec![
        //                             Data("x", "100"),
        //                             Data("y", "20"),
        //                             Data("width", "40"),
        //                             Data("height", "80"),
        //                             Data("as", "geometry"),
        //                         ],
        //                         inner: vec![],
        //                     }],
        //                 },
        //                 //     <mxCell id="vEEG278DrkbecYB9cJd1-3" value=""
        //                 // style="verticalLabelPosition=bottom;verticalAlign=top;html=1;
        //                 // shape=mxgraph.basic.pie;startAngle=0.25;endAngle=0.6109615943053768;
        //                 // fillColor=#1ba1e2;fontColor=#ffffff;strokeColor=#006EAF;" vertex="1" parent="1">
        //                 //     <mxGeometry x="120" y="80" width="120" height="120" as="geometry" />
        //                 //   </mxCell>
        //                 //   <mxCell id="vEEG278DrkbecYB9cJd1-4" value="" style="verticalLabelPosition=bottom;verticalAlign=top;html=1;shape=mxgraph.basic.pie;startAngle=0;endAngle=0.25;fillColor=#6a00ff;fontColor=#ffffff;strokeColor=#3700CC;" vertex="1" parent="1">
        //                 //     <mxGeometry x="120" y="80" width="120" height="120" as="geometry" />
        //                 //   </mxCell>
        //                 //   <mxCell id="vEEG278DrkbecYB9cJd1-5" value="" style="verticalLabelPosition=bottom;verticalAlign=top;html=1;shape=mxgraph.basic.pie;startAngle=0.6102574254185245;endAngle=0.8463754264756499;fillColor=#a20025;fontColor=#ffffff;strokeColor=#6F0000;" vertex="1" parent="1">
        //                 //     <mxGeometry x="120" y="80" width="120" height="120" as="geometry" />
        //                 //   </mxCell>
        //                 Element {
        //                     id: "mxCell",
        //                     data: vec![
        //                         Data("id", "4"),
        //                         Data("shape", "mxgraph.basic.pie"),
        //                         Data("startAngle", "0.25"),
        //                         Data("endAngle", "0.5"),
        //                         Data("vertex", "1"),
        //                         Data("parent", "1"),
        //                     ],
        //                     inner: vec![Element {
        //                         id: "mxGeometry",
        //                         data: vec![
        //                             Data("x", "150"),
        //                             Data("y", "20"),
        //                             Data("width", "40"),
        //                             Data("height", "80"),
        //                             Data("as", "geometry"),
        //                         ],
        //                         inner: vec![],
        //                     }],
        //                 },
        //             ],
        //         }],
        //     }],
        // }],
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

    #[test]
    fn test_srp() {
        let tasks = srp::task_sets::task_set1();
        tasks.store(&PathBuf::from("task_sets/task_set1.json")).ok();
        let tasks_loaded = Tasks::load(&PathBuf::from("task_sets/task_set1.json")).unwrap();
        assert_eq!(tasks, tasks_loaded);
    }
}
