use iced::pure::widget::Element;
use iced::pure::{column, container, text};
use iced::{Alignment, Canvas, Command, Length, Text};
use iced_lazy::pure::{self, Component};
use iced_native::text;

use opu_core::op1::OP1;

use crate::loading::Loading;
use crate::{style, Config};

pub struct SelectOperation<Message> {
    config: Config,
    op1: OP1,
    // loading: Loading,
    on_op1_found: Box<dyn Fn(OP1) -> Message>,
}

pub fn select_operation<Message>(
    config: Config,
    op1: OP1,
    on_op1_found: impl Fn(OP1) -> Message + 'static,
) -> SelectOperation<Message> {
    SelectOperation::new(config, op1, on_op1_found)
}

#[derive(Debug, Clone)]
pub enum Event {
    Tick,
}

impl<Message> SelectOperation<Message> {
    pub fn new(config: Config, op1: OP1, on_op1_found: impl Fn(OP1) -> Message + 'static) -> Self {
        Self {
            config,
            op1,
            // loading: Loading::default(),
            on_op1_found: Box::new(on_op1_found),
        }
    }
}

impl<Message, Renderer> Component<Message, Renderer> for SelectOperation<Message>
where
    Renderer: text::Renderer + 'static,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Message> {
        match event {
            Event::Tick => {
                // self.loading.tick();
                None
            }
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Event, Renderer> {
        // let loading = Canvas::new(&mut self.loading)
        //     .width(Length::Units(150))
        //     .height(Length::Units(50));

        column()
            .padding(20)
            .align_items(Alignment::Center)
            .push(
                text(String::from("Waiting for OP-1 to be connected"))
                    .size(50)
                    .color(self.config.theme().text_color()),
            )
            // .push(loading)
            .into()
    }
}

impl<'a, Message, Renderer> From<SelectOperation<Message>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'static + iced_native::text::Renderer,
{
    fn from(numeric_input: SelectOperation<Message>) -> Self {
        pure::component(numeric_input)
    }
}
