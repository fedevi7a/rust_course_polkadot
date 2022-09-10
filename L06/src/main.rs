mod app_with_custom_lib;
mod external_mod;
mod gui_test;
mod modules;
mod super_and_self;
mod use_clause;

fn main() {
    app_with_custom_lib::app_with_custom_lib();
    modules::modules();
    external_mod::external_mod();
    use_clause::use_clause();
    super_and_self::super_and_self();
    gui_test::gui_test();
}
