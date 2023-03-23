use native_dialog::FileDialog;

use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

use iced::{alignment, Element, Sandbox, Settings};

use iced::widget::{button, scrollable, text, Column};

use msg_parser::Outlook;

fn main() -> iced::Result {
    Ausblick::run(Settings::default())
}

struct Ausblick {
    subject: String,
    body: String,
}

fn parse_mailformats(path: &str) -> Ausblick {
    let file = Path::new(path);
    let ext = file.extension().unwrap();

    if ext == PathBuf::from("msg") {
        return Ausblick {
            subject: "MSG".to_string(),
            body: "MSG".to_string(),
        };
    }
    if ext == PathBuf::from("EML") {
        return Ausblick {
            subject: "EML".to_string(),
            body: "EML".to_string(),
        };
    } else {
        Ausblick {
            subject: "Foo".to_string(),
            body: "Bar".to_string(),
        }
    }
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
                    .add_filter("Outlook", &["msg"])
                    .add_filter("Mailbox", &["eml"])
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
                .push(
                    button(text("open").horizontal_alignment(alignment::Horizontal::Center))
                        .on_press(Message::OpenFileDialog),
                )
                .push(text("Subject:").size(28))
                .push(text(&self.subject))
                .push(text("Messge:").size(28))
                .push(text(&self.body)),
        ));
        scrollable_content
    }
}
