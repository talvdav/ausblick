use native_dialog::FileDialog;
use std::env;

use iced::{
    Element, Sandbox,  Settings, alignment,
};

use iced::widget::{
    button, Column, text, scrollable
};

use msg_parser::Outlook;

fn main() -> iced::Result {
    Ausblick::run(Settings::default())
}

struct Ausblick {
    subject: String,
    body: String,
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
                subject: outlook.subject,
                body: outlook.body,
            }
        } else {
            Ausblick {
                subject: "".to_string(),
                body: "".to_string(),
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

    fn view(&self) -> Element<Message> {
        let scrollable_content: Element<Message> =
            Element::from(
                scrollable(
                    Column::new()
                        .push(button(
                            text("open").horizontal_alignment(alignment::Horizontal::Center))
                              .on_press(Message::OpenFileDialog))
                        .push(text("Subject:"))
                        .push(text(&self.subject))
                        .push(text("Messge:"))
                        .push(text(&self.body))
                ));
        scrollable_content
    }

}
