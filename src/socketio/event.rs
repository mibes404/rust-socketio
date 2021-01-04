/// An Event in the socket.io. Could either be one of the common (`Message`, `Error`)
///  or a custom one.
pub enum Event {
    Message,
    Error,
    Custom(String),
    Open,
    Close,
}

impl From<String> for Event {
    fn from(string: String) -> Self {
        match &string.to_lowercase()[..] {
            "message" => Event::Message,
            "error" => Event::Error,
            "open" => Event::Open,
            "close" => Event::Close,
            custom => Event::Custom(custom.to_owned()),
        }
    }
}

impl From<&str> for Event {
    fn from(string: &str) -> Self {
        Event::from(String::from(string))
    }
}

impl From<Event> for String {
    fn from(event: Event) -> Self {
        match event {
            Event::Message => String::from("message"),
            Event::Open => String::from("open"),
            Event::Close => String::from("close"),
            Event::Error => String::from("error"),
            Event::Custom(string) => string,
        }
    }
}