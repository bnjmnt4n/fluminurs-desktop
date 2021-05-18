use iced::{executor, Application, Clipboard, Command, Element, Settings, Text};

pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}

struct Hello;

impl Application for Hello {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (Hello, Command<Self::Message>) {
        (Hello, Command::none())
    }

    fn title(&self) -> String {
        String::from("fluminurs-desktop")
    }

    fn update(&mut self, _message: Self::Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hello world from fluminurs-desktop!").into()
    }
}
