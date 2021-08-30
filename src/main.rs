use std::env;

use iced::
{
    // button, Align, Button, ProgressBar, Radio,
    scrollable, Column, Scrollable, Element,
    Sandbox, Settings, Text, Container,  Length,
    Row, Rule, Space,
};

use msg_parser::
{
    Outlook,
 // Person, Attachment, TransportHeaders
};

fn main() -> iced::Result {
    Ausblick::run(Settings::default())
}

struct Ausblick {
    subject: String,
    body: String,
}

#[derive(Debug, Clone, Copy)]
enum Message {

}

impl Sandbox for Ausblick {
    type Message = Message;

    fn new() -> Self {
        Ausblick
        {
            subject: String::from("Subject"),
            body: String::from("Body"),
        }
    }

    fn title (&self) -> String {
        String::from("Ausblick - Simple .msg Viewer")
    }

    fn update (&mut self, message: Message) {
        match message {

        }
    }

    fn view (&mut self) -> Element<Message> {
        Column::new().push(Text::new("Hello World!")).into()
    }




}

