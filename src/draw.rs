// drawio

use crate::xml::*;
use srp::common::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

mod mono {
    use std::sync::atomic::{AtomicU32, Ordering};
    static mut MONO: AtomicU32 = AtomicU32::new(1);

    pub(crate) fn get_new() -> u32 {
        let mono = unsafe { MONO.load(Ordering::SeqCst) };
        unsafe { MONO.store(mono + 1, Ordering::SeqCst) }
        mono
    }
}

impl Tag {
    fn new_mxcell() -> Self {
        Tag::new("mxCell")
            .attr_u32("id", mono::get_new())
            .attr("vertex", "1")
            .attr("parent", "0")
    }

    fn new_geometry(x: u32, y: u32, width: u32, height: u32) -> Self {
        Tag::new("mxGeometry")
            .attr_u32("x", x)
            .attr_u32("y", y)
            .attr_u32("width", width)
            .attr_u32("height", height)
            .attr("as", "geometry")
    }

    fn new_pie(x: u32, y: u32, width: u32, height: u32, start_angle: f32, end_angle: f32) -> Self {
        Tag::new_mxcell()
            // shape=mxgraph.basic.pie;startAngle={};endAngle={};
            .style("shape", "mxgraph.basic.pie")
            .style_f32("startAngle", start_angle)
            .style_f32("endAngle", end_angle)
            .inner(
                Tag::new("mxGeometry")
                    .attr_u32("x", x)
                    .attr_u32("y", y)
                    .attr_u32("width", width)
                    .attr_u32("height", height)
                    .attr("as", "geometry"),
            )
    }

    fn new_box(x: u32, y: u32, width: u32, height: u32) -> Self {
        Tag::new_mxcell().inner(Tag::new_geometry(x, y, width, height))
    }

    fn new_root() -> Self {
        // root will have index "0"
        Tag::new("root").inner(Tag::new("mxCell").attr("id", "0"))
    }

    fn new_draw_io(root: Tag) -> Self {
        Tag::new("mxfile").inner(
            Tag::new("diagram").inner(
                Tag::new("mxGraphModel")
                    .attr("dx", "320")
                    .attr("dy", "200")
                    .attr("pageWidth", "320")
                    .attr("pageHeight", "200")
                    .inner(root),
            ),
        )
    }

    fn new_draw(inner: Vec<Tag>) -> Self {
        let mut root = Tag::new_root();
        for e in inner {
            root.inner_ref(e);
        }

        Tag::new_draw_io(root)
    }

    fn save(self, path: &PathBuf) -> io::Result<()> {
        let mut file = File::create(&path)?;
        let io_str = format!("{}", self);
        file.write_all(io_str.as_bytes())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_mono() {
        assert_eq!(mono::get_new(), 1);
        assert_eq!(mono::get_new(), 2);
    }

    #[test]
    fn test_box() {
        let io = Tag::new_draw_io(Tag::new_root().inner(Tag::new_box(20, 20, 40, 40)));
        println!("{}", io);
        io.save(&PathBuf::from_str("xml/out.drawio").unwrap())
            .unwrap();
    }

    #[test]
    fn test_boxes() {
        let io = Tag::new_draw(vec![
            Tag::new_box(20, 20, 40, 40),
            Tag::new_box(100, 20, 40, 40),
        ]);
        println!("{}", io);
        io.save(&PathBuf::from_str("xml/out.drawio").unwrap())
            .unwrap();
    }

    #[test]
    fn test_bar_chart() {
        let bars: Vec<_> = [100, 200, 50, 150]
            .iter()
            .enumerate()
            .map(|(x, y)| Tag::new_box((x * 100) as u32, 300 - y, 50, *y))
            .collect();
        let io = Tag::new_draw(bars);

        println!("{}", io);
        io.save(&PathBuf::from_str("xml/out.drawio").unwrap())
            .unwrap();
    }

    #[test]
    fn test_pie() {
        let x = 100;
        let y = 100;
        let radius = 100;
        let io = Tag::new_draw(vec![
            Tag::new_pie(x, y, radius, radius, 0.0, 0.25),
            Tag::new_pie(x, y, radius, radius, 0.5, 0.75),
        ]);
        println!("{}", io);
        io.save(&PathBuf::from_str("xml/out.drawio").unwrap())
            .unwrap();
    }

    #[test]
    fn test_pie_color() {
        let x = 100;
        let y = 100;
        let radius = 100;
        let io = Tag::new_draw(vec![
            Tag::new_pie(x, y, radius, radius, 0.0, 0.25).style("fillColor", "#a20025"),
            Tag::new_pie(x, y, radius, radius, 0.25, 0.45).style("fillColor", "#000025"),
            Tag::new_pie(x, y, radius, radius, 0.45, 0.87).style("fillColor", "#008000"),
        ]);
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
