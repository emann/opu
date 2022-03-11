use crate::Config;

pub mod load;

#[derive(Debug, Clone)]
pub enum Operation {
    Save,
    Load,
}

pub struct Context<Message> {
    pub config: Config,
    pub on_back_button_press: Box<dyn Fn() -> Message>,
}
