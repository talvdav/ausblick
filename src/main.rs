use std::env;

use iced::
{
    // button, Align, Button, ProgressBar, Radio, Row, Container, Space,
    scrollable, Column, Scrollable, Element,
    Rule, Sandbox, Settings, Text, Length, HorizontalAlignment,
     
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
    scrollable_state: scrollable::State,
    subject: String,
    body: String,
}

#[derive(Debug, Clone, Copy)]
enum Message {

}

impl Sandbox for Ausblick {
    type Message = Message;

    fn new() -> Self {

        let args: Vec<String> = env::args().collect();
        let path: String = args[1].to_string();
        let outlook = Outlook::from_path(path).unwrap();
        Ausblick
        {
            scrollable_state: scrollable::State::new(),
            subject: outlook.subject,
            body: outlook.body,


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


        Column::new()
            .push(
                Scrollable::new(&mut self.scrollable_state)
                .padding(20)
                .push(Text::new("Subject: ").size(30))
                .push(Rule::horizontal(15))
                .push(Text::new(self.subject.clone()))
                .push(Rule::horizontal(15))
                .push(Text::new("Message: ").size(30))
                .push(Rule::horizontal(15))
                .push(Text::new(self.body.clone()))
                .push(Rule::horizontal(15))
                .width(Length::Fill)
                .height(Length::Fill))
            .into()

    }

}

