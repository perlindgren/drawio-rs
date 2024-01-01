// drawio

use crate::xml::*;
use srp::common::*;
use std::default::Default;

#[derive(Debug)]
struct BarChart {
    title: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    label_margin: u32,
    bar_margin: u32,
    //  label, data, color
    data: Vec<(String, (u32, String))>,
}

impl Default for BarChart {
    fn default() -> Self {
        BarChart {
            title: "Bar Chart".to_string(),
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            bar_margin: 10,
            label_margin: 10,
            data: vec![],
        }
    }
}

impl BarChart {
    pub fn new(
        title: String,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        bar_margin: u32,
        label_margin: u32,

        data: Vec<(String, (u32, String))>,
    ) -> Self {
        BarChart {
            title,
            x,
            y,
            width,
            height,
            bar_margin,
            label_margin,
            data,
        }
    }

    pub fn draw(self) -> Tag {
        let bars_height = self.height - self.label_margin;
        let x_scale = self.width as f32 / (self.data.len() + 1) as f32;

        let (id_vec, y_c_vec): (Vec<_>, Vec<_>) = self.data.into_iter().unzip();

        let (y_vec, _c_vec): (Vec<_>, Vec<_>) = y_c_vec.clone().into_iter().unzip();

        let y_max = *y_vec.iter().max().unwrap();

        println!("id_vec {:?}", id_vec);
        println!("y_vec {:?}", y_vec);
        println!("y_max {:?}", y_max);

        let y_scale = bars_height as f32 / y_max as f32;

        println!("x_scale {}, y_scale {}", x_scale, y_scale);

        let mut bars: Vec<_> = y_c_vec
            .iter()
            .enumerate()
            .map(|(x, (y, color))| {
                let y_scaled = (y_scale * *y as f32) as u32;
                Tag::rect(
                    ((x as f32 + 0.5) * x_scale) as u32 + self.label_margin / 2,
                    (self.height - self.label_margin) - y_scaled,
                    x_scale as u32 - self.label_margin,
                    y_scaled,
                )
                .style("fillColor", color)
            })
            .collect();

        let mut texts: Vec<_> = id_vec
            .iter()
            .enumerate()
            .map(|(x, id)| {
                Tag::text(
                    id,
                    ((x as f32 + 0.5) * x_scale) as u32,
                    self.height - self.label_margin / 2,
                    x_scale as u32,
                    self.label_margin,
                )
            })
            .collect();

        bars.append(&mut texts);
        let io = Tag::draw(bars);

        io
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_bar() {
        let bar_chart = BarChart::new(
            "Task Deadlines".to_string(),
            100,
            100,
            100,
            100,
            10,
            10,
            vec![
                ("T1".into(), (50, "#800000".to_string())),
                ("T2".into(), (100, "#008000".to_string())),
            ],
        );

        let io = bar_chart.draw();
        io.save(&PathBuf::from_str("xml/bars.drawio").unwrap())
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
