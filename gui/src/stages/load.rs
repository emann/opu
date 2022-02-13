use crate::Config;
use iced::{
    button, pick_list, Align, Application, Button, Column, Command, Container, Element, Length,
    PickList, Row, Sandbox, Settings, Space, Text,
};
use iced_aw::selection_list::StyleSheet;
use iced_aw::{selection_list, SelectionList};
use opu_core::dirs::get_dirs;
use opu_core::op1::OP1;
use opu_core::project::{Project, ProjectLibrary};
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
        Ok((
            Load {
                config: prev.config.clone(),
                op1: prev.op1.op1_dirs.clone().into(),
                projects: Project::get_all_projects_in_dir(get_dirs().projects),
                selection_list: selection_list::State::default(),
                selected_project: Project::default(),
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

        let left_column = Column::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .spacing(10)
            .push(Text::new("Pick a Project to Load"))
            .push(
                Container::new(pick_list)
                    .width(Length::Units(200))
                    .height(Length::Units(200)),
            );

        let content = Row::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .spacing(10)
            .push(left_column)
            .push(Text::new(format!("{}", self.selected_project)));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
