use iced::{
    Element, Task, Theme, Renderer, window::{Id, Settings}, Program,
};
use crate::types::app::App;
use crate::types::message::Message;

impl Program for App {
    type State = App;
    type Message = Message;
    type Theme = Theme;
    type Renderer = Renderer;
    type Executor = iced::executor::Default;

    fn name() -> &'static str {
        "Iced Video Player"
    }

    fn settings(&self) -> iced::Settings {
        iced::Settings::default()
    }

    fn window(&self) -> Option<Settings> {
        Some(Settings {
            decorations: false,
            ..Default::default()
        })
    }

    fn boot(&self) -> (Self::State, Task<Self::Message>) {
        (App::default(), Task::none())
    }

    fn update(
        &self,
        state: &mut Self::State,
        message: Self::Message,
    ) -> Task<Self::Message> {
        state.update(message)
    }

    fn view<'a>(
        &self,
        state: &'a Self::State,
        _window: Id,
    ) -> Element<'a, Self::Message, Self::Theme, Self::Renderer> {
        state.view()
    }
}
