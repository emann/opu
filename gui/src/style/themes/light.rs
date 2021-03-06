use iced::{container, radio, rule, scrollable, Color};

use crate::style::colors;

pub const BACKGROUND: Color = colors::hardware::LIGHT_GRAY;

const SURFACE: Color = Color::from_rgb(
    0x40 as f32 / 255.0,
    0x44 as f32 / 255.0,
    0x4B as f32 / 255.0,
);

const ACCENT: Color = Color::from_rgb(
    0x6F as f32 / 255.0,
    0xFF as f32 / 255.0,
    0xE9 as f32 / 255.0,
);

pub const ACTIVE: Color = colors::hardware::DARK_GRAY;

const SCROLLBAR: Color = Color::from_rgb(
    0x2E as f32 / 255.0,
    0x33 as f32 / 255.0,
    0x38 as f32 / 255.0,
);

const SCROLLER: Color = Color::from_rgb(
    0x20 as f32 / 255.0,
    0x22 as f32 / 255.0,
    0x25 as f32 / 255.0,
);

pub struct Container;

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color {
                a: 0.99,
                ..BACKGROUND
            }
            .into(),
            text_color: Color::WHITE.into(),
            ..container::Style::default()
        }
    }
}

pub struct Radio;

impl radio::StyleSheet for Radio {
    fn active(&self) -> radio::Style {
        radio::Style {
            background: SURFACE.into(),
            dot_color: ACTIVE,
            border_width: 1.0,
            border_color: ACTIVE,
        }
    }

    fn hovered(&self) -> radio::Style {
        radio::Style {
            background: Color { a: 0.5, ..SURFACE }.into(),
            ..self.active()
        }
    }
}

pub struct Scrollable;

impl scrollable::StyleSheet for Scrollable {
    fn active(&self) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Color {
                a: 0.8,
                ..SCROLLBAR
            }
            .into(),
            border_radius: 2.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: Color { a: 0.7, ..SCROLLER },
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self) -> scrollable::Scrollbar {
        let active = self.active();

        scrollable::Scrollbar {
            background: SCROLLBAR.into(),
            scroller: scrollable::Scroller {
                color: SCROLLER,
                ..active.scroller
            },
            ..active
        }
    }

    fn dragging(&self) -> scrollable::Scrollbar {
        let hovered = self.hovered();

        scrollable::Scrollbar {
            scroller: scrollable::Scroller {
                color: ACCENT,
                ..hovered.scroller
            },
            ..hovered
        }
    }
}

pub struct Rule;

impl rule::StyleSheet for Rule {
    fn style(&self) -> rule::Style {
        rule::Style {
            color: SURFACE,
            width: 2,
            radius: 1.0,
            fill_mode: rule::FillMode::Percent(30.0),
        }
    }
}
