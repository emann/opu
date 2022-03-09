use iced::Length;
use std::fmt::{Display, Formatter};

use iced::pure::widget::Element;
use iced::pure::{container, Container};
use iced_lazy::pure::{self, Component};
use iced_native::text;

pub use select_operation::Context as SelectOperationContext;
pub use wait_for_op1::Context as WaitForOP1Context;

use select_operation::select_operation;
use wait_for_op1::wait_for_op1;

mod operations;
pub mod select_operation;
pub mod wait_for_op1;

pub use operations::Operation;

pub enum Page<Message> {
    WaitForOP1(WaitForOP1Context<Message>),
    SelectOperation(SelectOperationContext<Message>),
}

impl<Message> Page<Message> {
    pub(crate) fn component<'a, Renderer>(&'a self) -> Element<'a, Message, Renderer>
    where
        Renderer: 'static + iced_native::text::Renderer,
    {
        match self {
            Page::WaitForOP1(context) => wait_for_op1(&context).into(),
            Page::SelectOperation(context) => select_operation(&context).into(),
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
                Page::SelectOperation(_) => "Select Operation",
                // Page::Load => "Load Project",
                // Page::Save => "Save Project",
            }
        )
    }
}
