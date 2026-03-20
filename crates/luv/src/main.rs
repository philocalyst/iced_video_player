mod types;

use types::app::App;
use iced::Task;

fn main() -> iced::Result {
    iced::application(
        || (App::default(), Task::none()),
        App::update,
        App::view,
    )
    .window(iced::window::Settings {
        decorations: false,
        ..Default::default()
    })
    .run()
}
