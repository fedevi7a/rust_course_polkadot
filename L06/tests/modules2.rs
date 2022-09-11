// You can bring module paths into scopes and provide new names for them with the
// 'use' and 'as' keywords. Fix these 'use' statements to make the code compile.

mod delicious_snacks {
    pub(crate) use self::fruits::PEAR as fruit;
    pub(crate) use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const _APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const _CARROT: &str = "Carrot";
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_modules() {
        println!(
            "favorite snacks: {} and {}",
            delicious_snacks::fruit,
            delicious_snacks::veggie
        );
    }
}
