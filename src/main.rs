use iced::{
    button, executor, Align, Application, Button, Column, Command, Element, Font,
    HorizontalAlignment, Length, Row, Settings, Subscription, Text,
};

const FONT: Font = Font::External {
    name: "PixelMplus12-Regular",
    bytes: include_bytes!("../rsc/PixelMplus12-Regular.ttf"),
};

#[derive(Debug, Clone)]
pub enum Messag {
    Start,
    Stop,
    Reset,
}

pub enum TickState {
    Stopped,
    Ticking,
}

struct GUI {
    tick_state: TickState,
    start_stop_button_state: button::State,
    reset_button_state: button::State,
}

impl Application for GUI {
    type Executor = executor::Null;
    type Message = Messag;
    type Flags = ();

    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (
            GUI {
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
            Messag::Start => {
                self.tick_state = TickState::Ticking;
            }
            Messag::Stop => {
                self.tick_state = TickState::Stopped;
            }
            Messag::Reset => {}
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        // prepare duration text
        let duration_time = "25:00:00.00";

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
            TickState::Stopped => Messag::Start,
            TickState::Ticking => Messag::Stop,
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
        .on_press(Messag::Reset);

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
