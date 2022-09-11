use fltk::{
    app::App,
    button::Button,
    prelude::{GroupExt, WidgetExt},
    window::DoubleWindow,
};

use fltk::{app::*, browser::*, enums::*, input::*, prelude::*, window::*};

const WIDGET_WIDTH: i32 = 70;
const WIDGET_HEIGHT: i32 = 25;
const WIDGET_PADDING: i32 = 10;

#[derive(Clone, Copy)]
enum Message {
    Create,
    Update,
    Delete,
    Select,
    Filter,
    Save,
}

use crate::entidad::PersonaDAO;
use crate::entidad::{Persona, ScreenOutput};

pub struct Gui {
    app: App,
    wind: DoubleWindow,
    sender: Sender<Message>,
    receiver: Receiver<Message>,
    model: Vec<Persona>,
    persona_dao: PersonaDAO,
    filter_input: Input,
    list_browser: HoldBrowser,
    ident_input: Input,
    name_input: Input,
    surname_input: Input,
    create_button: Button,
    update_button: Button,
    delete_button: Button,
    save_button: Button,
}

impl Gui {
    pub fn new() -> Gui {
        let app = App::default().with_scheme(Scheme::Gtk);
        let wind = Window::default().with_label("CRUD");
        let (sender, receiver) = channel::<Message>();

        let filter_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .with_pos(WIDGET_PADDING + WIDGET_WIDTH * 2, WIDGET_PADDING)
            .with_label("Filter prefix:");

        let list_browser = HoldBrowser::default()
            .with_pos(
                WIDGET_PADDING,
                filter_input.y() + filter_input.height() + WIDGET_PADDING,
            )
            .with_size(WIDGET_WIDTH * 3, WIDGET_HEIGHT * 4);

        let ident_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .with_pos(
                list_browser.x() + list_browser.width() + WIDGET_PADDING + WIDGET_WIDTH,
                list_browser.y(),
            )
            .with_label("Id:");

        let name_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&ident_input, WIDGET_PADDING)
            .with_label("Nombres:");

        let surname_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&name_input, WIDGET_PADDING)
            .with_label("Apellidos:");

        let create_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .with_pos(
                WIDGET_PADDING,
                list_browser.y() + list_browser.height() + WIDGET_PADDING,
            )
            .with_label("Crear");

        let update_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .right_of(&create_button, WIDGET_PADDING)
            .with_label("Modificar");

        let delete_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .right_of(&update_button, WIDGET_PADDING)
            .with_label("Borrar");

        let save_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .right_of(&delete_button, WIDGET_PADDING)
            .with_label("Guardar");

        let persona_dao = PersonaDAO::new();
        let model = persona_dao.as_vector();

        Gui {
            app,
            wind,
            sender,
            receiver,
            filter_input,
            list_browser,
            persona_dao,
            model,
            ident_input,
            name_input,
            surname_input,
            create_button,
            update_button,
            delete_button,
            save_button,
        }
    }

    pub fn build(&mut self) {
        self.filter_input.set_trigger(CallbackTrigger::Changed);
        self.filter_input.emit(self.sender, Message::Filter);

        self.list_browser.emit(self.sender, Message::Select);

        //self.sender.send(Message::Filter);

        self.create_button.emit(self.sender, Message::Create);

        self.update_button.emit(self.sender, Message::Update);
        self.update_button.deactivate();

        self.delete_button.emit(self.sender, Message::Delete);
        self.delete_button.deactivate();

        self.save_button.emit(self.sender, Message::Save);

        self.wind.set_size(
            self.name_input.x() + self.name_input.width() + WIDGET_PADDING,
            self.create_button.y() + self.create_button.height() + WIDGET_PADDING,
        );

        self.sender.send(Message::Filter);
    }

    fn clear_edit(&mut self) {
        self.ident_input.set_value("");
        self.name_input.set_value("");
        self.surname_input.set_value("");
    }

    pub fn show(&mut self) {
        self.wind.end();
        self.wind.show();
        while self.app.wait() {
            match self.receiver.recv() {
                Some(Message::Create) => {
                    self.model.push(Persona {
                        identificacion: self.ident_input.value(),
                        apellidos: self.surname_input.value(),
                        nombres: self.name_input.value(),
                    });
                    self.clear_edit();
                    self.sender.send(Message::Filter);
                }
                Some(Message::Update) => {
                    if self.list_browser.value() > 0 {
                        let text_selection =
                            self.list_browser.text(self.list_browser.value()).unwrap();
                        let search_result = self
                            .model
                            .iter_mut()
                            .find(|e| e.to_screen().eq_ignore_ascii_case(&text_selection));
                        match search_result {
                            Some(persona) => {
                                persona.nombres = self.name_input.value();
                                persona.apellidos = self.surname_input.value();
                                self.clear_edit();
                                self.sender.send(Message::Filter);
                                self.sender.send(Message::Select);
                            }
                            _ => {
                                println!("ELEMENTO NO ENCONTRADO!!!");
                            }
                        }
                    } else {
                        println!("NO HAY ELEMENTO PARA MODIFICAR!!!");
                    }
                }
                Some(Message::Delete) => {
                    if self.list_browser.value() > 0 {
                        let text_selection =
                            self.list_browser.text(self.list_browser.value()).unwrap();
                        let search_result = self
                            .model
                            .iter()
                            .enumerate()
                            .find(|e| e.1.to_screen().eq_ignore_ascii_case(&text_selection));
                        match search_result {
                            Some((index, _persona)) => {
                                self.model.remove(index);
                                self.clear_edit();
                                self.sender.send(Message::Filter);
                                self.sender.send(Message::Select);
                            }
                            _ => {
                                println!("ELEMENTO NO ENCONTRADO!!!");
                            }
                        }
                    } else {
                        println!("NO HAY ELEMENTO PARA ELIMINAR!!!");
                    }
                }
                Some(Message::Save) => {
                    self.persona_dao.save_and_refresh(&self.model);
                    self.model = self.persona_dao.as_vector();
                    self.clear_edit();
                    self.sender.send(Message::Filter);
                    self.sender.send(Message::Select);
                }
                Some(Message::Select) => {
                    if self.list_browser.value() == 0 {
                        self.update_button.deactivate();
                        self.delete_button.deactivate();
                    } else {
                        let text_selection =
                            self.list_browser.text(self.list_browser.value()).unwrap();
                        let search_result = self
                            .model
                            .iter()
                            .find(|e| e.to_screen().eq_ignore_ascii_case(&text_selection));

                        match search_result {
                            Some(persona) => {
                                self.ident_input.set_value(&persona.identificacion);
                                self.name_input.set_value(&persona.nombres);
                                self.surname_input.set_value(&persona.apellidos);
                                self.update_button.activate();
                                self.delete_button.activate();
                            }
                            _ => {
                                println!("ELEMENTO NO ENCONTRADO!!!");
                            }
                        }
                    }
                }
                Some(Message::Filter) => {
                    let prefix: String = self.filter_input.value().to_lowercase();
                    let filter_empty: bool = prefix.trim().eq_ignore_ascii_case("");
                    self.list_browser.clear();
                    for (_i, p) in self.model.iter().enumerate() {
                        if (p.identificacion.eq_ignore_ascii_case(prefix.as_str())) || filter_empty
                        {
                            let item = p.to_screen();
                            self.list_browser.add(&item);
                        }
                    }
                    self.sender.send(Message::Select);
                }
                None => {}
            }
        }
    }

    #[allow(dead_code)]
    pub fn refresh(&mut self, data: Vec<Persona>) {
        for (i, p) in data.iter().enumerate() {
            println!("{} {:?} ", i, p);
        }
    }
}
