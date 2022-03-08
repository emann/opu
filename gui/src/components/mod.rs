pub mod select_operation;
pub mod wait_for_op1;

use crate::components::wait_for_op1::Event;
use iced::pure::widget::Element;
use iced_lazy::pure::{self, Component};
use iced_native::text;
pub use select_operation::SelectOperation;
use std::fmt::{Display, Formatter};
pub use wait_for_op1::WaitForOP1;

pub enum Page<Message> {
    WaitForOP1(WaitForOP1<Message>),
    SelectOperation(SelectOperation<Message>),
    Load,
    Save,
}

// impl<Message> Page<Message> {
//     fn view<Renderer>(&self) -> Element<'_, Message, Renderer>
//     where
//         Renderer: 'static + iced_native::text::Renderer,
//     {
//         match self {
//             Page::WaitForOP1(op1) => op1.view(&()),
//             _ => panic!("panik"),
//         }
//     }
// }

// impl<'a, Message, Renderer> From<Page<Message>> for Element<'a, Message, Renderer>
// where
//     Renderer: 'static + iced_native::text::Renderer,
//     Message: 'a,
// {
//     fn from(page: Page<Message>) -> Element<'a, Message, Renderer> {
//         match page {
//             Page::WaitForOP1(op1) => pure::component(op1),
//             _ => panic!("panik"),
//         }
//     }
// }

impl<Message> Display for Page<Message> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Page::WaitForOP1(_) => "Waiting For OP1 To Be Connected",
                Page::SelectOperation(_) => "Select Operation",
                Page::Load => "Load Project",
                Page::Save => "Save Project",
            }
        )
    }
}
