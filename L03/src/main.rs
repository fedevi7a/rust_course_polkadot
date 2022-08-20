mod functions;
mod scope;
mod variable_rules;

fn main() {
    scope::scope();
    functions::functions();
    variable_rules::variable_rules();
}
