#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_enum() {
        println!("{:?}", Message::Quit);
        println!("{:?}", Message::Echo);
        println!("{:?}", Message::Move);
        println!("{:?}", Message::ChangeColor);
    }
}
