mod types;

use iced::Task;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use types::app::App;

fn main() -> iced::Result {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    iced::application(|| (App::default(), Task::none()), App::update, App::view)
        .window(iced::window::Settings {
            decorations: false,
            ..Default::default()
        })
        .run()
}
