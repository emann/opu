use iced::pure::widget::Element;
use iced::pure::{button, column, container, text};
use iced::{Alignment, Canvas, Command, Length, Text};
use iced_lazy::pure::{self, Component};
use iced_native::text;

use opu_core::op1::OP1;

use crate::loading::Loading;
use crate::{style, Config};

pub struct Context<Message> {
    pub config: Config,
    pub on_op1_found: Box<dyn Fn(OP1) -> Message>,
}

pub struct WaitForOP1<'a, Message> {
    context: &'a Context<Message>,
}

pub fn wait_for_op1<'a, Message>(context: &'a Context<Message>) -> WaitForOP1<'a, Message> {
    WaitForOP1::new(context)
}

#[derive(Debug, Clone)]
pub enum Event {
    Tick,
}

impl<'a, Message> WaitForOP1<'a, Message> {
    pub fn new(context: &'a Context<Message>) -> Self {
        Self { context }
    }
}

impl<'a, Message, Renderer> Component<Message, Renderer> for WaitForOP1<'a, Message>
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
                    .color(self.context.config.theme().text_color()),
            )
            .push(button(text("button")).on_press(Event::Tick))
            // .push(loading)
            .into()
    }
}

impl<'a, Message, Renderer> From<WaitForOP1<'a, Message>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'static + iced_native::text::Renderer,
{
    fn from(numeric_input: WaitForOP1<'a, Message>) -> Self {
        pure::component(numeric_input)
    }
}
