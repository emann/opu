use crate::Config;
use iced::{button, Align, Button, Column, Command, Element, Row, Text};
use opu_core::op1::OP1;
use triax_ui::{NewStage, Stage, StageMessage};

#[derive(Clone, Debug, StageMessage)]
#[triax_ui(stage(SelectOperation))]
#[allow(clippy::large_enum_variant)]
pub enum Message {
    #[triax_ui(next_stage(crate::stages::Load))]
    SavePressed,
    #[triax_ui(next_stage(crate::stages::Load))]
    LoadPressed,
    #[triax_ui(next_stage(crate::stages::Load))]
    PatchManagerPressed,
    #[triax_ui(next_stage(crate::stages::Load))]
    SettingsPressed,
}

pub struct SelectOperation {
    pub(crate) config: Config,
    pub(crate) op1: OP1,
    save_button: button::State,
    load_button: button::State,
    patch_manager_button: button::State,
    settings_button: button::State,
}

impl SelectOperation {
    fn new(config: Config) -> Self {
        SelectOperation {
            config,
            op1: OP1::get_connected_op1_blocking(), // TODO: Just get it, error if none found
            save_button: button::State::default(),
            load_button: button::State::default(),
            patch_manager_button: button::State::default(),
            settings_button: button::State::default(),
        }
    }
}

impl TryFrom<&mut super::WaitForOP1ToBeConnected> for NewStage<SelectOperation> {
    type Error = String;

    fn try_from(prev: &mut super::WaitForOP1ToBeConnected) -> Result<Self, Self::Error> {
        Ok((SelectOperation::new(prev.config.clone()), Command::none()))
    }
}

impl TryFrom<&mut super::Load> for NewStage<SelectOperation> {
    type Error = String;

    fn try_from(prev: &mut super::Load) -> Result<Self, Self::Error> {
        Ok((SelectOperation::new(prev.config.clone()), Command::none()))
    }
}

impl TryFrom<&mut super::Save> for NewStage<SelectOperation> {
    type Error = String;

    fn try_from(prev: &mut super::Save) -> Result<Self, Self::Error> {
        Ok((SelectOperation::new(prev.config.clone()), Command::none()))
    }
}

impl Stage for SelectOperation {
    type Application = crate::OPU;
    type Message = Message;

    fn new(
        _config: &<<Self as Stage>::Application as iced::Application>::Flags,
    ) -> (Self, Command<Message>) {
        unimplemented!("use TryFrom<WaitForOP1ToBeConnected> instead");
    }

    fn title(&self) -> String {
        String::from("Select Operation")
    }

    fn update(
        &mut self,
        _message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> Command<Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Row::new()
            .push(
                Column::new()
                    .padding(20)
                    .align_items(Align::Center)
                    .push(
                        Button::new(&mut self.save_button, Text::new("Save"))
                            .on_press(Message::LoadPressed),
                    )
                    .push(
                        Button::new(&mut self.load_button, Text::new("Load"))
                            .on_press(Message::LoadPressed),
                    ),
            )
            .push(
                Column::new()
                    .padding(20)
                    .align_items(Align::Center)
                    .push(
                        Button::new(&mut self.patch_manager_button, Text::new("Patches"))
                            .on_press(Message::LoadPressed),
                    )
                    .push(
                        Button::new(&mut self.settings_button, Text::new("Settings"))
                            .on_press(Message::LoadPressed),
                    ),
            )
            .into()
    }
}
