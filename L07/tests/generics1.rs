// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

#[cfg(test)]
mod tests {

    #[test]
    fn test_generic() {
        let _shopping_list: Vec<&str> = vec!["milk"];
    }
}
