// Your task is to replace the '??' sections so the code compiles.
// Don't change any line other than the marked one.

pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {
    _name: String,
}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn some_func(item: (impl SomeTrait + OtherTrait)) -> bool {
    item.some_function() && item.other_function()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_func() {
        some_func(SomeStruct {
            _name: String::from("test"),
        });
    }
}
