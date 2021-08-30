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
        Ausblick
        {
            scrollable_state: scrollable::State::new(),
            subject: String::from("Subject"),
            body: String::from("Body hello world lorem ipcontentsuasdasssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss
            ssssssssdalksjdölaskjdöalkjföalskdfj aslkdjfölas kddjfölaskdjföla jflöasjdflökajsd lföjasdlökfjlödskjföal sdjfölasd flaskdjflö asjdflkasjd f
            asdlfkjasdölfjaölsdjf asldkfj a
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
           
            


            
            
            
            
            
            
            
            
            
            ölsdjföalskdjgöasfdjgölaskjdfglöasdjf lökasjdf  
            asdlfkjasdölfj aölksdjf ölaksdjfölajsdöflkja öldjf ölaksdjjf m mother fuckor dolor
            
            -- FIN --"),


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
                .padding(10)
                .push(Text::new(self.subject.clone()))
                .push(Text::new(self.body.clone()))
                .width(Length::Fill)
                .height(Length::Fill))
            .into()

    }

}

