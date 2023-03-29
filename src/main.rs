use native_dialog::FileDialog;

use std::env;
use std::path::Path;
use std::path::PathBuf;

use iced::{alignment, Element, Sandbox, Settings};

use iced::widget::{button, scrollable, text, Column};

use msg_parser::Outlook;
use eml_parser::{eml::Eml, EmlParser};

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
        let msg = Outlook::from_path(file).unwrap();
        return Ausblick {
            subject: msg.subject,
            body: msg.body,
        };
    }
    if ext == PathBuf::from("eml") {
        let eml: Eml = EmlParser::from_file(file).unwrap()
            .with_body()
            .parse().unwrap();
        return Ausblick {
            subject: eml.subject.to_owned().unwrap(),
            body: eml.body.to_owned().unwrap(),
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
