use native_dialog::FileDialog;
use std::env;

use iced::{
    button,
    scrollable,
    Button, //Align,  ProgressBar, Radio, Row, Container, Space, HorizontalAlignment,
    Column,
    Element,
    Length,
    Sandbox,
    Scrollable,
    Settings,
    Text,
};

use msg_parser::Outlook;

fn main() -> iced::Result {
    Ausblick::run(Settings::default())
}

struct Ausblick {
    scrollable_state: scrollable::State,
    subject: String,
    body: String,
    open_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    OpenFileDialog,
}

impl Sandbox for Ausblick {
    type Message = Message;

    fn new() -> Self {
        let argc = env::args().count();

        if argc >= 2 {
            let args: Vec<String> = env::args().collect();
            let path: String = args[1].to_string();
            let outlook = Outlook::from_path(path).unwrap();
            Ausblick {
                scrollable_state: scrollable::State::new(),
                subject: outlook.subject,
                body: outlook.body,
                open_button: button::State::new(),
            }
        } else {
            Ausblick {
                scrollable_state: scrollable::State::new(),
                subject: "".to_string(),
                body: "".to_string(),
                open_button: button::State::new(),
            }
        }
    }

    fn title(&self) -> String {
        String::from("Ausblick - Simple .msg Viewer")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::OpenFileDialog => {
                let path = FileDialog::new()
                    .add_filter("Outlook", &["msg"])
                    .set_location("~")
                    .show_open_single_file()
                    .unwrap();

                match path {
                    Some(path) => {
                        let outlook = Outlook::from_path(path).unwrap();
                        self.subject = outlook.subject;
                        self.body = outlook.body;
                    }
                    None => return,
                };
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(
                Scrollable::new(&mut self.scrollable_state)
                    .padding(20)
                    .push(Text::new("Subject: ").size(30))
                    .push(Text::new(self.subject.clone()))
                    .push(Text::new("Message: ").size(30))
                    .push(Text::new(self.body.clone()))
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .push(
                Button::new(&mut self.open_button, Text::new("Open"))
                    .width(Length::Fill)
                    .padding(20)
                    .on_press(Message::OpenFileDialog),
            )
            .into()
    }
}
