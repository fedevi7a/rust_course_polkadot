// Don't mind this for now :)
// I AM NOT DONE

fn call_me() {
    println!("called");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_function() {
        call_me();
    }
}
