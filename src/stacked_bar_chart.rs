// stacked_bar_chart

use crate::xml::*;
use std::default::Default;

#[derive(Debug)]
pub struct StackedBarChart {
    title: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    label_margin: u32,
    bar_margin: u32,
    //  label, color
    stack_labels: Vec<(String, String)>,
    //  label, (bound, vector of values)
    data: Vec<(String, (u32, Vec<u32>))>,
}

impl Default for StackedBarChart {
    fn default() -> Self {
        StackedBarChart {
            title: "Stacked Bar Chart".to_string(),
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            label_margin: 10,
            bar_margin: 10,
            stack_labels: vec![],
            data: vec![],
        }
    }
}

impl StackedBarChart {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        title: String,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        label_margin: u32,
        bar_margin: u32,
        stack_labels: Vec<(String, String)>,
        data: Vec<(String, (u32, Vec<u32>))>,
    ) -> Self {
        StackedBarChart {
            title,
            x,
            y,
            width,
            height,
            bar_margin,
            label_margin,
            stack_labels,
            data,
        }
    }

    pub fn draw(self) -> Tag {
        // we reserve space at top and bottom
        let bars_height = self.height - (self.label_margin * 2);

        let x_scale = self.width as f32 / (self.data.len() + 1) as f32;
        let (id_vec, y_vec): (Vec<_>, Vec<_>) = self.data.into_iter().unzip();

        let y_vec_sum: Vec<u32> = y_vec.iter().map(|(_, bar)| bar.iter().sum()).collect(); // sum each stack
        let y_vec_top: Vec<u32> = y_vec.iter().map(|(top, _)| *top).collect(); // sum each stack

        let y_max = (*y_vec_sum.iter().max().unwrap()).max(*y_vec_top.iter().max().unwrap()); // overall max stack

        let y_scale = bars_height as f32 / y_max as f32;
        println!("y_scale {}", y_scale);

        // create a vector of stacks
        let stacks: Vec<(_, Vec<_>)> = y_vec
            .iter()
            .enumerate()
            .map(|(x, (top, stack))| {
                let (acc, mut stack) = stack.iter().enumerate().fold(
                    (0u32, vec![]),
                    |(acc_value, mut acc_stack), (i, y)| {
                        let y_scaled = (y_scale * *y as f32) as u32;
                        acc_stack.push(
                            Tag::rect(
                                self.x + self.bar_margin / 2 + ((x as f32 + 0.5) * x_scale) as u32, // offset by half bar_margin
                                self.y + self.label_margin + (bars_height - y_scaled) - acc_value, // offset by label_margin from top
                                x_scale as u32 - self.bar_margin,
                                y_scaled,
                            )
                            .style("fillColor", &self.stack_labels.get(i).unwrap().1),
                        );

                        (acc_value + y_scaled, acc_stack)
                    },
                );
                let y_scaled = (y_scale * *top as f32) as u32;
                println!("y_scale {}, top {}, y_scaled {}", y_scale, top, y_scaled);
                stack.push(Tag::line(
                    self.x + self.bar_margin / 2 + ((x as f32 + 0.5) * x_scale) as u32, // offset by half bar_margin
                    self.y + self.label_margin + bars_height - y_scaled, // offset by label_margin from top
                    self.x
                        + self.bar_margin / 2
                        + ((x as f32 + 0.5) * x_scale) as u32
                        + x_scale as u32
                        - self.bar_margin, // offset by half bar_margin
                    self.y + self.label_margin + bars_height - y_scaled, // offset by label_margin from top
                ));
                (acc, stack)
            })
            .collect();

        // flatten into a vector of rectangles, skip the accumulated value
        let mut bars: Vec<_> = stacks.into_iter().flat_map(|(_, vec)| vec).collect();

        let mut texts: Vec<_> = id_vec
            .iter()
            .enumerate()
            .map(|(x, id)| {
                Tag::text(
                    id,
                    self.x + ((x as f32 + 0.5) * x_scale) as u32,
                    self.y + self.height - self.label_margin,
                    x_scale as u32,
                    self.label_margin,
                )
            })
            .collect();

        bars.append(&mut texts);
        bars.push(Tag::text(self.title, 0, 0, self.width, self.label_margin));

        // legend
        let legend_width = 100;

        bars.push(Tag::text(
            "Legend",
            self.width,
            0,
            legend_width,
            self.label_margin,
        ));

        for (i, (label, color)) in self.stack_labels.iter().rev().enumerate() {
            bars.push(Tag::text(
                label,
                self.width,
                (i as u32 + 2) * self.label_margin,
                legend_width / 2,
                self.label_margin,
            ));

            bars.push(
                Tag::rect(
                    self.width + (legend_width as f32 * (0.5 + 0.125)) as u32,
                    ((i as f32 + 2.25) * self.label_margin as f32) as u32,
                    legend_width / 4,
                    self.label_margin / 2,
                )
                .style("fillColor", color),
            );
        }

        Tag::draw(bars)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use srp::common::*;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_stacked_bars() {
        let bar_chart = StackedBarChart::new(
            "Task Response Times".to_string(),
            0,
            0,
            200,
            200,
            20,
            10,
            vec![
                ("WCET".to_string(), "#008000".to_string()),
                ("Blocking".to_string(), "#000080".to_string()),
                ("Preemption".to_string(), "#800000".to_string()),
            ],
            vec![
                ("T1".into(), (320, vec![50, 100])),
                ("T2".into(), (120, vec![25, 75])),
                ("T3".into(), (140, vec![10, 50, 75])),
            ],
        );

        let io = bar_chart.draw();
        io.save(&PathBuf::from_str("xml/stacked_bars.drawio").unwrap())
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
