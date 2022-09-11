#[derive(Debug)]
struct Rectangle {
    _width: u32,
    _height: u32,
}

impl Rectangle {
    fn _can_hold(&self, other: &Rectangle) -> bool {
        self._width > other._width && self._height > other._height
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    pub fn larger_can_hold_smaller() {
        let larger = Rectangle {
            _width: 8,
            _height: 7,
        };
        let smaller = Rectangle {
            _width: 5,
            _height: 1,
        };

        assert!(larger._can_hold(&smaller));
    }
}
