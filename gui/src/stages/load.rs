use crate::Config;
use iced::{
    button, pick_list, Align, Application, Button, Column, Command, Container, Element, Length,
    PickList, Row, Sandbox, Settings, Space, Text,
};
use iced_aw::selection_list::StyleSheet;
use iced_aw::{selection_list, SelectionList};
use opu_core::metadata::Metadata;
use opu_core::op1::OP1;
use opu_core::project::Project;
use opu_core::OPUConfig;
use triax_ui::{
    loading::Logo,
    widgets::{column, text},
    NewStage, Stage, StageMessage,
};

#[derive(Clone, Copy, Debug)]
pub struct CustomStyle;

impl StyleSheet for CustomStyle {
    fn style() -> selection_list::Style {
        selection_list::Style {
            width: Length::Fill,
            height: Length::Fill,
            selected_background: crate::style::colors::hardware::GREEN.into(),
            text_size: 36,
            background: crate::style::colors::hardware::LIGHT_GRAY.into(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, StageMessage)]
#[triax_ui(stage(Load))]
#[allow(clippy::large_enum_variant)]
pub enum Message {
    #[triax_ui(next_stage(crate::stages::SelectOperation))]
    Done,
    ProjectSelected(Project),
}

pub struct Load {
    pub(crate) config: Config,
    pub(crate) op1: OP1,
    projects: Vec<Project>,
    selection_list: selection_list::State<Project>,
    selected_project: Project,
}

impl TryFrom<&mut super::SelectOperation> for NewStage<Load> {
    type Error = String;

    fn try_from(prev: &mut super::SelectOperation) -> Result<Self, Self::Error> {
        let config = prev.config.clone();
        let projects = Project::get_all_projects_in_dir(config.project_library());
        // TODO: Handle no projects available case
        let selected_project = projects.clone().into_iter().next().expect(
            "For now we expect there to be a project available to use as the first selected",
        );
        Ok((
            Load {
                config,
                projects,
                op1: prev.op1.op1_dirs.clone().into(),
                selection_list: selection_list::State::default(),
                selected_project,
            },
            Command::none(),
        ))
    }
}

impl Stage for Load {
    type Application = crate::OPU;
    type Message = Message;

    fn new(
        _config: &<<Self as Stage>::Application as Application>::Flags,
    ) -> (Self, Command<<Self as Stage>::Message>) {
        unimplemented!("use TryFrom<Start> instead");
    }

    fn title(&self) -> String {
        String::from("Load Project")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> Command<Message> {
        match message {
            Message::ProjectSelected(project) => {
                self.selected_project = project;
                Command::none()
            }
            _ => Command::none(),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let pick_list = SelectionList::new(
            &mut self.selection_list,
            &self.projects,
            &Some(self.selected_project.clone()),
            Message::ProjectSelected,
            CustomStyle::style(),
        );

        let row = Row::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .spacing(10)
            .push(
                Container::new(pick_list)
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .push(metadata_display_container(
                &self.config,
                &self.selected_project.metadata,
            ));

        let content = Column::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .spacing(10)
            .push(
                Text::new("Select a Project to Load")
                    .color(self.config.theme().text_color())
                    .size(24),
            )
            .push(row);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn metadata_display_container<'a, Message: 'a>(
    config: &'a Config,
    metadata: &'a Metadata,
) -> Element<'a, Message> {
    // Root data
    let project_name = Text::new(metadata.project_name.clone()).color(config.theme().text_color());
    let created = Text::new(format!(
        "Created: {}",
        metadata.created().format("%I:%M:%S%P %x")
    ))
    .color(config.theme().text_color());
    let last_saved = Text::new(format!(
        "Last Saved: {}",
        metadata.last_saved.format("%I:%M:%S%P %x")
    ))
    .color(config.theme().text_color());

    // Tempo Settings
    let bpm = Text::new(format!("BPM: {}", metadata.tempo_settings.bpm))
        .color(config.theme().text_color());
    let tape_speed = Text::new(format!(
        "Tape Speed: {}",
        metadata.tempo_settings.tape_speed
    ))
    .color(config.theme().text_color());

    // Mixer Settings
    // --- Per Channel Mix Settings
    let track_1_mix_settings = Text::new(format!(
        "Track 1: Level: {}, Pan: {}",
        metadata.mixer_settings.per_channel_mix_settings[0].level,
        metadata.mixer_settings.per_channel_mix_settings[0].pan
    ))
    .color(config.theme().text_color());
    let track_2_mix_settings = Text::new(format!(
        "Track 2: Level: {}, Pan: {}",
        metadata.mixer_settings.per_channel_mix_settings[1].level,
        metadata.mixer_settings.per_channel_mix_settings[1].pan
    ))
    .color(config.theme().text_color());
    let track_3_mix_settings = Text::new(format!(
        "Track 3: Level: {}, Pan: {}",
        metadata.mixer_settings.per_channel_mix_settings[2].level,
        metadata.mixer_settings.per_channel_mix_settings[2].pan
    ))
    .color(config.theme().text_color());
    let track_4_mix_settings = Text::new(format!(
        "Track 4: Level: {}, Pan: {}",
        metadata.mixer_settings.per_channel_mix_settings[3].level,
        metadata.mixer_settings.per_channel_mix_settings[3].pan
    ))
    .color(config.theme().text_color());
    // --- EQ Settings
    let low_balance = Text::new(format!(
        "Low: {}",
        metadata.mixer_settings.per_channel_mix_settings[3].level
    ))
    .color(config.theme().text_color());
    let mid_balance = Text::new(format!(
        "Mid: {}",
        metadata.mixer_settings.per_channel_mix_settings[3].level
    ))
    .color(config.theme().text_color());
    let high_balance = Text::new(format!(
        "High: {}",
        metadata.mixer_settings.per_channel_mix_settings[3].level
    ))
    .color(config.theme().text_color());
    // --- Master Effect Settings
    // let effect_name = Text::new(format!(
    //     "Effect: {}",
    //     metadata.mixer_settings.master_effect_settings.effect
    // ))
    // .color(config.theme().text_color());
    let effect_blue = Text::new(format!(
        "Blue Knob: {}",
        metadata.mixer_settings.master_effect_settings.blue
    ))
    .color(config.theme().text_color());
    let effect_green = Text::new(format!(
        "Green Knob: {}",
        metadata.mixer_settings.master_effect_settings.blue
    ))
    .color(config.theme().text_color());
    let effect_white = Text::new(format!(
        "White Knob: {}",
        metadata.mixer_settings.master_effect_settings.blue
    ))
    .color(config.theme().text_color());
    let effect_orange = Text::new(format!(
        "Orange Knob: {}",
        metadata.mixer_settings.master_effect_settings.blue
    ))
    .color(config.theme().text_color());
    // --- Master Out Mix Settings
    let left_balance = Text::new(format!(
        "Left Balance: {}",
        metadata.mixer_settings.master_out_settings.left_balance
    ))
    .color(config.theme().text_color());
    let right_balance = Text::new(format!(
        "Right Balance: {}",
        metadata.mixer_settings.master_out_settings.right_balance
    ))
    .color(config.theme().text_color());
    let drive = Text::new(format!(
        "Drive: {}",
        metadata.mixer_settings.master_out_settings.drive
    ))
    .color(config.theme().text_color());
    let release = Text::new(format!(
        "Release: {}",
        metadata.mixer_settings.master_out_settings.release
    ))
    .color(config.theme().text_color());

    let content = Column::with_children(vec![
        project_name.into(),
        created.into(),
        last_saved.into(),
        bpm.into(),
        tape_speed.into(),
        track_1_mix_settings.into(),
        track_2_mix_settings.into(),
        track_3_mix_settings.into(),
        track_4_mix_settings.into(),
        low_balance.into(),
        mid_balance.into(),
        high_balance.into(),
        // effect_name.into(),
        effect_blue.into(),
        effect_green.into(),
        effect_white.into(),
        effect_orange.into(),
        left_balance.into(),
        right_balance.into(),
        drive.into(),
        release.into(),
    ])
    .width(Length::Fill)
    .align_items(Align::Center)
    .spacing(10);

    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}
