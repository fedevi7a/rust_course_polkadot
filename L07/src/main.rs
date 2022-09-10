mod generic_bound;
mod generic_functions;
mod generic_simple;
mod trait_example;
mod traits;

fn main() {
    generic_simple::generic_simple();
    generic_functions::generic_functions();
    generic_bound::generic_bound();
    traits::traits();
    trait_example::traits_example();
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        // panic!("Make this test fail");
    }
}
