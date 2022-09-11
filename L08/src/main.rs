use crate::entidad::PersonaDAO;

mod entidad;
mod presentacion;

fn main() {
    let mut _persona_dao = PersonaDAO::new();
    let mut gui = presentacion::Gui::new();
    gui.build();
    gui.show();
}
