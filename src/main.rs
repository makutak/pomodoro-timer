use std::time::{Duration, Instant};

use iced::{
    button, executor, Align, Application, Button, Column, Command, Element, Font,
    HorizontalAlignment, Length, Row, Settings, Subscription, Text,
};

use iced_futures::{self, futures};

const FONT: Font = Font::External {
    name: "PixelMplus12-Regular",
    bytes: include_bytes!("../rsc/PixelMplus12-Regular.ttf"),
};

const FPS: u64 = 30;
const MILLISEC: u64 = 30;
const MINUTE: u64 = 60;
const HOUR: u64 = 60 * MINUTE;

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Stop,
    Reset,
    Update,
}

pub enum TickState {
    Stopped,
    Ticking,
}

struct GUI {
    last_update: Instant,
    total_duration: Duration,
    tick_state: TickState,
    start_stop_button_state: button::State,
    reset_button_state: button::State,
}

impl Application for GUI {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (
            GUI {
                last_update: Instant::now(),
                total_duration: Duration::new(25 * MINUTE, 0),
                tick_state: TickState::Stopped,
                start_stop_button_state: button::State::new(),
                reset_button_state: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Pomodoro Timer")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Start => {
                self.tick_state = TickState::Ticking;
                self.last_update = Instant::now();
            }
            Message::Stop => {
                self.tick_state = TickState::Stopped;
                self.total_duration += Instant::now() - self.last_update;
            }
            Message::Reset => {
                self.last_update = Instant::now();
                self.total_duration = Duration::new(25 * MINUTE, 0);
                //self.total_duration = Duration::default()
            }
            Message::Update => match self.tick_state {
                TickState::Ticking => {
                    let now_update = Instant::now();
                    self.total_duration += now_update - self.last_update;
                    self.last_update = now_update;
                }
                _ => {}
            },
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let timer = Timer::new(Duration::from_millis(MILLISEC / FPS));
        iced::Subscription::from_recipe(timer).map(|_| Message::Update)
    }

    fn view(&mut self) -> Element<Self::Message> {
        // prepare duration text
        let seconds = self.total_duration.as_secs();
        let duration_time = format!(
            "{:0>2}:{:0>2}.{:0>2}",
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
            self.total_duration.subsec_millis() / 10,
        );

        // prepare start/stop text
        let start_stop_text = match self.tick_state {
            TickState::Stopped => Text::new("Start")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
            TickState::Ticking => Text::new("Stop")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        };

        // prepare start/stop message on button press
        let start_stop_message = match self.tick_state {
            TickState::Stopped => Message::Start,
            TickState::Ticking => Message::Stop,
        };

        // init widgets
        let tick_text = Text::new(duration_time).font(FONT).size(60);
        let start_stop_button = Button::new(&mut self.start_stop_button_state, start_stop_text)
            .min_width(80)
            .on_press(start_stop_message);

        let reset_button = Button::new(
            &mut self.reset_button_state,
            Text::new("Reset")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        )
        .min_width(80)
        .on_press(Message::Reset);

        // prepare column
        Column::new()
            .push(tick_text)
            .push(
                Row::new()
                    .push(start_stop_button)
                    .push(reset_button)
                    .spacing(10),
            )
            .spacing(10)
            .spacing(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::Center)
            .into()
    }
}

fn main() {
    GUI::run(Settings::default())
}

pub struct Timer {
    duration: Duration,
}

impl Timer {
    fn new(duration: Duration) -> Timer {
        Timer { duration: duration }
    }
}

impl<H, E> iced_native::subscription::Recipe<H, E> for Timer
where
    H: std::hash::Hasher,
{
    type Output = Instant;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
        self.duration.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, E>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        use futures::stream::StreamExt;
        async_std::stream::interval(self.duration)
            .map(|_| Instant::now())
            .boxed()
    }
}
