use iced::widget::MouseArea;
use iced::{
    Background, Border, Color, Element, Event, Length, event, keyboard, mouse,
    widget::{Button, Column, Container, Row, Slider, Stack, Text, button, container, mouse_area},
};
use iced_video_player::{Video, VideoPlayer};
use std::time::{Duration, Instant};

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}

#[derive(Clone, Debug)]
enum Message {
    TogglePause,
    ToggleLoop,
    Seek(f64),
    SeekRelease,
    EndOfStream,
    NewFrame,
    EventOccurred(Event),
}

struct App {
    video: Video,
    position: f64,
    dragging: bool,
    controls_visible: bool,
    last_interaction: Instant,
}

impl Default for App {
    fn default() -> Self {
        App {
            video: Video::new(
                &url::Url::from_file_path(
                    std::path::PathBuf::from(file!())
                        .parent()
                        .unwrap()
                        .join("../.media/test.mp4")
                        .canonicalize()
                        .unwrap(),
                )
                .unwrap(),
            )
            .unwrap(),
            position: 0.0,
            dragging: false,
            controls_visible: true,
            last_interaction: Instant::now(),
        }
    }
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::TogglePause => {
                self.video.set_paused(!self.video.paused());
                self.last_interaction = Instant::now();
                self.controls_visible = true;
            }
            Message::ToggleLoop => {
                self.video.set_looping(!self.video.looping());
                self.last_interaction = Instant::now();
                self.controls_visible = true;
            }
            Message::Seek(secs) => {
                self.dragging = true;
                self.video.set_paused(true);
                self.position = secs;
                self.last_interaction = Instant::now();
                self.controls_visible = true;
            }
            Message::SeekRelease => {
                self.dragging = false;
                self.video
                    .seek(Duration::from_secs_f64(self.position), false)
                    .expect("seek");
                self.video.set_paused(false);
                self.last_interaction = Instant::now();
            }
            Message::EndOfStream => {
                println!("end of stream");
            }
            Message::NewFrame => {
                if !self.dragging {
                    self.position = self.video.position().as_secs_f64();
                }

                // Auto-hide controls after 3 seconds of inactivity
                if self.last_interaction.elapsed() > Duration::from_secs(3) && !self.dragging {
                    self.controls_visible = false;
                }
            }
            Message::EventOccurred(event) => match event {
                Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                    self.last_interaction = Instant::now();
                    self.controls_visible = true;
                }
                Event::Keyboard(_) => {
                    self.last_interaction = Instant::now();
                    self.controls_visible = true;
                }
                _ => {}
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let video_player = VideoPlayer::new(&self.video)
            .width(Length::Fill)
            .height(Length::Fill)
            .content_fit(iced::ContentFit::Contain)
            .on_end_of_stream(Message::EndOfStream)
            .on_new_frame(Message::NewFrame);

        let controls_opacity = if self.controls_visible { 1.0 } else { 0.0 };

        let slider = Container::new(
            Slider::new(
                0.0..=self.video.duration().as_secs_f64(),
                self.position,
                Message::Seek,
            )
            .step(0.1)
            .on_release(Message::SeekRelease),
        )
        .padding(iced::Padding::new(8.0).left(20.0).right(20.0))
        .style(move |_theme| container::Style {
            background: Some(Background::Color(Color::from_rgba(
                0.0,
                0.0,
                0.0,
                0.7 * controls_opacity,
            ))),
            border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..Default::default()
        });

        let play_button =
            Button::new(Text::new(if self.video.paused() { "Play" } else { "Pause" }).size(14))
                .style(|_theme, _status| button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.2, 0.4, 0.8))),
                    text_color: Color::WHITE,
                    border: Border {
                        radius: 8.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .padding(iced::Padding::new(10.0).left(20.0).right(20.0))
                .on_press(Message::TogglePause);

        let loop_button = Button::new(
            Text::new(if self.video.looping() {
                "Loop: On"
            } else {
                "Loop: Off"
            })
            .size(14),
        )
        .style(|_theme, _status| button::Style {
            background: Some(Background::Color(Color::from_rgb(0.15, 0.15, 0.2))),
            text_color: Color::WHITE,
            border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .padding(iced::Padding::new(10.0).left(20.0).right(20.0))
        .on_press(Message::ToggleLoop);

        let time_display = Text::new(format!(
            "{}:{:02} / {}:{:02}",
            self.position as u64 / 60,
            self.position as u64 % 60,
            self.video.duration().as_secs() / 60,
            self.video.duration().as_secs() % 60,
        ))
        .size(14);

        let controls = Container::new(
            Row::new()
                .spacing(12)
                .align_y(iced::alignment::Vertical::Center)
                .push(play_button)
                .push(loop_button)
                .push(
                    Container::new(time_display)
                        .width(Length::Fill)
                        .align_x(iced::alignment::Horizontal::Right),
                ),
        )
        .style(move |_theme| container::Style {
            background: Some(Background::Color(Color::from_rgba(
                0.05,
                0.05,
                0.08,
                0.9 * controls_opacity,
            ))),
            border: Border {
                radius: 10.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .padding(iced::Padding::new(14.0).left(20.0).right(20.0));

        let controls_overlay = Container::new(
            Column::new()
                .push(Container::new(Text::new("")).height(Length::Fill))
                .push(slider)
                .push(controls)
                .spacing(8)
                .padding(20),
        )
        .width(Length::Fill)
        .height(Length::Fill);

        MouseArea::new(Stack::new().push(video_player).push(controls_overlay))
            .on_move(|_| {
                Message::EventOccurred(Event::Mouse(mouse::Event::CursorMoved {
                    position: iced::Point::ORIGIN,
                }))
            })
            .into()
    }
}
