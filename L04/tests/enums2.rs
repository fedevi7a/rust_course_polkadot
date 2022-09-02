#[derive(Debug)]
enum Message {
    Move { _x: u8, _y: u8 },
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

mod tests {
    use super::*;

    #[test]
    fn main() {
        let messages = [
            Message::Move { _x: 10, _y: 30 },
            Message::Echo(String::from("hello world")),
            Message::ChangeColor(200, 255, 255),
            Message::Quit,
        ];

        for message in &messages {
            message.call();
        }
    }
}
