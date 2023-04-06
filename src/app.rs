
use std::path::Path;
use std::path::PathBuf;

use msg_parser::Outlook;
use eml_parser::{eml::Eml, EmlParser};

pub struct Mail {
    pub subject: String,
    pub body: String,
}

pub fn parse_mailformats(path: &str) -> Mail {
    let file = Path::new(path);
    let ext = file.extension().unwrap();

    if ext == PathBuf::from("msg") {
        let msg = Outlook::from_path(file).unwrap();
        return Mail {
            subject: msg.subject,
            body: msg.body,
        };
    }
    if ext == PathBuf::from("eml") {
        let eml: Eml = EmlParser::from_file(file).unwrap()
            .with_body()
            .parse().unwrap();
        return Mail {
            subject: eml.subject.to_owned().unwrap(),
            body: eml.body.to_owned().unwrap(),
        };
    } else {
        Mail {
            subject: "Ausblick error".to_string(),
            body: "unable to show contents".to_string(),
        }
    }
}
