use iced::{
    canvas::{self, Cursor, Geometry, Path, Program},
    time, Color, Point, Rectangle, Subscription,
};
use std::time::Instant;

use crate::style::colors;

pub struct Loading {
    current_interval: u64,
    canvas: canvas::Cache,
    colors: Vec<Color>,
    n_circles: u64,
    tick_ms: u64,
    n_intervals: u64,
}

impl Loading {
    pub fn new(colors: Vec<Color>, tick_ms: u64) -> Self {
        let n_circles = colors.len() as u64;
        // + 2 to allow one interval with none showing and one intervals with all showing
        let n_intervals = n_circles + 2;
        Self {
            current_interval: 0,
            canvas: canvas::Cache::new(),
            colors,
            n_circles,
            tick_ms,
            n_intervals,
        }
    }

    pub fn tick(&mut self) {
        self.current_interval = (self.current_interval + 1) % self.n_intervals;
        self.canvas.clear();
    }

    pub fn subscription(&self) -> Subscription<Instant> {
        time::every(std::time::Duration::from_millis(self.tick_ms))
    }

    pub fn num_circles_to_display(&self) -> u64 {
        self.current_interval.min(self.n_circles)
    }
}

impl Default for Loading {
    fn default() -> Self {
        Loading::new(
            vec![
                colors::hardware::BLUE,
                colors::hardware::GREEN,
                colors::hardware::WHITE,
                colors::hardware::ORANGE,
            ],
            200,
        )
    }
}

impl<Message: Clone> Program<Message> for Loading {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let loading = self.canvas.draw(bounds.size(), |frame| {
            let center_spacing = frame.width() / ((self.n_circles * 2) as f32);
            let radius = center_spacing / 2.0;

            for circle_num in 0..self.num_circles_to_display() {
                let offset: f32 = ((circle_num * 2) + 1) as f32;
                let center = Point {
                    x: offset * center_spacing,
                    y: radius,
                };
                let background = Path::circle(center, radius);
                frame.fill(
                    &background,
                    self.colors
                        .get(circle_num as usize)
                        .expect("Current tick should never exceed the number of colors")
                        .to_owned(),
                );
            }
        });

        vec![loading]
    }
}
