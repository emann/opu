use iced::pure::widget::Element;
use iced::pure::{button, column, container, row, text};
use iced::{Alignment, Canvas, Command, Length, Text};
use iced_lazy::pure::{self, Component};
use iced_native::text;

use opu_core::op1::OP1;

use crate::components::Operation;
use crate::loading::Loading;
use crate::{style, Config};

pub struct Context<Message> {
    pub config: Config,
    pub on_operation_selected: Box<dyn Fn(Operation) -> Message>,
}

pub struct SelectOperation<'a, Message> {
    context: &'a Context<Message>,
}

pub fn select_operation<'a, Message>(
    context: &'a Context<Message>,
) -> SelectOperation<'a, Message> {
    SelectOperation::new(context)
}

impl<'a, Message> SelectOperation<'a, Message> {
    pub fn new(context: &'a Context<Message>) -> Self {
        Self { context }
    }
}

impl<'a, Message, Renderer> Component<Message, Renderer> for SelectOperation<'a, Message>
where
    Renderer: text::Renderer + 'static,
{
    type State = ();
    type Event = Operation;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        Some(self.context.on_operation_selected.as_ref()(event))
    }

    fn view(&self, _state: &Self::State) -> Element<Operation, Renderer> {
        row()
            .push(
                column()
                    .padding(20)
                    .align_items(Alignment::Center)
                    .push(button(text("Save")).on_press(Operation::Save))
                    .push(button(text("Load")).on_press(Operation::Load)),
            )
            .push(
                column()
                    .padding(20)
                    .align_items(Alignment::Center)
                    .push(button(text("Patches")).on_press(Operation::Load))
                    .push(button(text("Settings")).on_press(Operation::Load)),
            )
            .into()
    }
}

impl<'a, Message, Renderer> From<SelectOperation<'a, Message>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'static + iced_native::text::Renderer,
{
    fn from(numeric_input: SelectOperation<'a, Message>) -> Self {
        pure::component(numeric_input)
    }
}
