// pub mod select_operation;
pub mod wait_for_op1;

use crate::components::wait_for_op1::Event;
use iced::pure::widget::Element;
use iced_lazy::pure::Component;
use std::fmt::{Display, Formatter};
pub use wait_for_op1::WaitForOP1;

pub enum Page<Message> {
    WaitForOP1(WaitForOP1<Message>),
    Load,
    Save,
}

impl<'a, Message> Page<Message> {
    fn view(&self) -> Element<'a, Message, _> {
        match self {
            Page::WaitForOP1(p) => p.view(),
            _ => panic!("AH"),
        }
    }
}

impl<Message> Display for Page<Message> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Page::WaitForOP1(_) => "Waiting For OP1 To Be Connected",
                Page::Load => "Load Project",
                Page::Save => "Save Project",
            }
        )
    }
}
