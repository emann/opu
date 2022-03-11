use iced::pure::widget::Element;
use iced::pure::{button, column, container, pick_list, row, text};
use iced::{Alignment, Canvas, Command, Length, Text};
use iced_lazy::pure::{self, Component};
use iced_native::text;

use opu_core::op1::OP1;
use opu_core::project::Project;
use opu_core::OPUConfig;

use crate::components::operations::Context;
use crate::components::Operation;
use crate::loading::Loading;
use crate::{style, Config};

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

pub enum Event {
    BackButtonPressed,
    ProjectSelected(Project),
}

impl<'a, Message, Renderer> Component<Message, Renderer> for SelectOperation<'a, Message>
where
    Renderer: text::Renderer + 'static,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::BackButtonPressed => Some(self.context.on_back_button_press.as_ref()()),
            Event::ProjectSelected(project) => {
                println!("Project Selected: {}", project);
                None
            }
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
        let pick_list = pick_list(
            Project::get_all_projects_in_dir(self.context.config.project_library()),
            Some(
                Project::get_all_projects_in_dir(self.context.config.project_library())
                    .into_iter()
                    .next()
                    .unwrap(),
            ),
            Event::ProjectSelected,
        );

        let left_column = column()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(10)
            .push(text("Pick a Project to Load"))
            .push(
                container(pick_list)
                    .width(Length::Units(200))
                    .height(Length::Units(200)),
            );

        let content = row()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(10)
            .push(left_column)
            .push(text(format!("{}", "placeholder")));

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
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
