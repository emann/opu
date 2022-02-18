use crate::loading::Loading;
use crate::{style, Config};
use iced::{Align, Canvas, Column, Command, Element, Length, Text};
use opu_core::op1::OP1;
use triax_ui::{Stage, StageMessage};

#[derive(Clone, Debug, StageMessage)]
#[triax_ui(stage(WaitForOP1ToBeConnected))]
#[allow(clippy::large_enum_variant)]
pub enum Message {
    #[triax_ui(next_stage(crate::stages::SelectOperation))]
    OP1Found,
    Tick,
}

pub struct WaitForOP1ToBeConnected {
    pub config: Config,
    pub loading: Loading,
}

impl Stage for WaitForOP1ToBeConnected {
    type Application = crate::OPU;
    type Message = Message;

    fn new(
        config: &<<Self as Stage>::Application as iced::Application>::Flags,
    ) -> (Self, Command<Message>) {
        let wait_for_op1 = Self {
            config: config.clone(),
            loading: Loading::default(),
        };

        (
            wait_for_op1,
            match OP1::find_connected() {
                Some(_) => triax_ui::message_command::<Self>(Message::OP1Found),
                None => Command::perform(OP1::get_connected_op1(), move |_| Message::OP1Found),
            },
        )
    }

    fn title(&self) -> String {
        String::from("Finding OP1")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> Command<Message> {
        match message {
            Message::Tick => {
                self.loading.tick();
                Command::none()
            }
            Message::OP1Found => Command::none(),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let loading = Canvas::new(&mut self.loading)
            .width(Length::Units(150))
            .height(Length::Units(50));

        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Text::new(String::from("Waiting for OP-1 to be connected"))
                    .size(50)
                    .color(self.config.theme().text_color()),
            )
            .push(loading)
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        self.loading.subscription().map(move |_| Message::Tick)
    }
}
