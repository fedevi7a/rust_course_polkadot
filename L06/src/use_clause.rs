// Bind the `deeply::nested::function` path to `other_function`.
use crate::use_clause::deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {

    #[derive(Debug)]
    #[allow(dead_code)]
    pub struct Persona {
        pub apellidos: String,
        pub nombres: String,
    }

    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

pub(crate) fn use_clause() {
    // Easier access to `deeply::nested::function`
    other_function();
    let persona = deeply::Persona {
        apellidos: String::from("VALERA"),
        nombres: String::from("RAMON"),
    };
    println!("{:?}", persona);

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        use crate::use_clause::deeply::nested::function;

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function()` is only in this block.
        function();

        println!("Leaving block");
    }

    function();
}
