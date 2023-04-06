use std::env;
use iced::{alignment, Element, Sandbox, Settings};
use iced::widget::{button, scrollable, text, Column};
use native_dialog::FileDialog;

use crate::app::parse_mailformats;

struct Ausblick {
    subject: String,
    body: String,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    OpenFileDialog,
}

pub fn run_iced() -> iced::Result {

    Ausblick::run(Settings::default())
}

impl Sandbox for Ausblick {
    type Message = Message;

    fn new() -> Self {
        let argc = env::args().count();

        if argc >= 2 {
            let args: Vec<String> = env::args().collect();
            let file_path: String = args[1].to_string();
            let f = parse_mailformats(&file_path);
            Ausblick {
                subject: f.subject,
                body: f.body,
            }
        } else {
            Ausblick {
                subject: "".to_string(),
                body: "".to_string(),
            }
        }
    }

    fn title(&self) -> String {
        String::from("Ausblick")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::OpenFileDialog => {
                let path = FileDialog::new()
                    .add_filter("", &["msg", "eml"])
                    .add_filter("All", &["*"])
                    .set_location("~")
                    .show_open_single_file()
                    .unwrap();

                match path {
                    Some(path) => {
                        let p = &path.into_os_string().into_string().unwrap();
                        println!("P {}", &p);
                        let f = parse_mailformats(&p);
                        self.subject = f.subject;
                        self.body = f.body;
                    }
                    None => return,
                };
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let scrollable_content: Element<Message> = Element::from(scrollable(
            Column::new()
                .push(text("Subject:").size(28))
                .push(text(&self.subject))
                .push(text("Messge:").size(28))
                .push(text(&self.body)),
        ));

        Column::new()
            .push(
                button(text("open").horizontal_alignment(alignment::Horizontal::Center))
                    .on_press(Message::OpenFileDialog),
            )
            .push(scrollable_content).into()
    }
}
