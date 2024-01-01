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
    pub fn mxcell() -> Self {
        Tag::new("mxCell")
            .attr("id", mono::get_new())
            .attr("vertex", "1")
            .attr("parent", "0")
    }

    pub fn geometry(x: u32, y: u32, width: u32, height: u32) -> Self {
        Tag::new("mxGeometry")
            .attr("x", x)
            .attr("y", y)
            .attr("width", width)
            .attr("height", height)
            .attr("as", "geometry")
    }

    pub fn pie(x: u32, y: u32, width: u32, height: u32, start_angle: f32, end_angle: f32) -> Self {
        Tag::mxcell()
            // shape=mxgraph.basic.pie;startAngle={};endAngle={};
            .style("shape", "mxgraph.basic.pie")
            .style("startAngle", start_angle)
            .style("endAngle", end_angle)
            .inner(
                Tag::new("mxGeometry")
                    .attr("x", x)
                    .attr("y", y)
                    .attr("width", width)
                    .attr("height", height)
                    .attr("as", "geometry"),
            )
    }

    pub fn rect(x: u32, y: u32, width: u32, height: u32) -> Self {
        Tag::mxcell().inner(Tag::geometry(x, y, width, height))
    }

    pub fn root() -> Self {
        // root will have index "0"
        Tag::new("root").inner(Tag::new("mxCell").attr("id", "0"))
    }

    fn draw_io(root: Tag) -> Self {
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

    pub fn draw(inner: Vec<Tag>) -> Self {
        let mut root = Tag::root();
        for e in inner {
            root.inner_ref(e);
        }

        Tag::draw_io(root)
    }

    pub fn save(self, path: &PathBuf) -> io::Result<()> {
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
        let m1 = mono::get_new();
        let m2 = mono::get_new();
        assert!(m2 > m1);
    }

    #[test]
    fn test_rect() {
        let io = Tag::draw_io(Tag::root().inner(Tag::rect(20, 20, 40, 40)));
        println!("{}", io);
        io.save(&PathBuf::from_str("xml/rectangle.drawio").unwrap())
            .unwrap();
    }

    #[test]
    fn test_rectangles() {
        let io = Tag::draw(vec![Tag::rect(20, 20, 40, 40), Tag::rect(100, 20, 40, 40)]);
        println!("{}", io);
        io.save(&PathBuf::from_str("xml/rectangles.drawio").unwrap())
            .unwrap();
    }

    #[test]
    fn test_bar_chart() {
        let bars: Vec<_> = [100, 200, 50, 150]
            .iter()
            .enumerate()
            .map(|(x, y)| Tag::rect((x * 100) as u32, 300 - y, 50, *y))
            .collect();
        let io = Tag::draw(bars);

        println!("{}", io);
        io.save(&PathBuf::from_str("xml/bar_chart.drawio").unwrap())
            .unwrap();
    }

    #[test]
    fn test_pie() {
        let x = 100;
        let y = 100;
        let radius = 100;
        let io = Tag::draw(vec![
            Tag::pie(x, y, radius, radius, 0.0, 0.25),
            Tag::pie(x, y, radius, radius, 0.5, 0.75),
        ]);
        println!("{}", io);
        io.save(&PathBuf::from_str("xml/pie_chart.drawio").unwrap())
            .unwrap();
    }

    #[test]
    fn test_pie_color() {
        let x = 100;
        let y = 100;
        let radius = 100;
        let io = Tag::draw(vec![
            Tag::pie(x, y, radius, radius, 0.0, 0.25).style("fillColor", "#800000"),
            Tag::pie(x, y, radius, radius, 0.25, 0.45).style("fillColor", "#000080"),
            Tag::pie(x, y, radius, radius, 0.45, 0.87).style("fillColor", "#008000"),
        ]);
        println!("{}", io);
        io.save(&PathBuf::from_str("xml/pie_chart_color.drawio").unwrap())
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
