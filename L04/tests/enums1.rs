// I AM NOT DONE

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
}

#[cfg(test)]
mod tests {
    #[test]
    fn call_enum() {
        println!("{:?}", Message::Quit);
        println!("{:?}", Message::Echo);
        println!("{:?}", Message::Move);
        println!("{:?}", Message::ChangeColor);
    }
}
