fn function() {
    println!("called `function()`");
}

mod my2;

pub(crate) fn external_mod() {
    my2::function();
    function();
    my2::indirect_access();
    my2::nested::function();
}
