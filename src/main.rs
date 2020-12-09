use iced::{executor, Application, Command, Element, Settings, Text};

struct GUI;

impl Application for GUI {
    type Executor = executor::Null;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (GUI, Command::none())
    }

    fn title(&self) -> String {
        String::from("DEMO")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Pomodoro Timer").into()
    }
}

fn main() {
    GUI::run(Settings::default())
}
