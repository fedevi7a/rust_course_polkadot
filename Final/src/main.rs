mod entidad;
mod presentacion;

fn main() {
    let mut gui = presentacion::Gui::new();
    gui.build();
    gui.show();
}
