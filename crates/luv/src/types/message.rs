use iced::Event;

#[derive(Clone, Debug)]
pub enum Message {
    TogglePause,
    ToggleLoop,
    Seek(f64),
    SeekRelease,
    EndOfStream,
    NewFrame,
    EventOccurred(Event),
}
