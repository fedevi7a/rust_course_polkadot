use fltk::{
    app::{channel, App, Receiver, Scheme, Sender},
    browser::HoldBrowser,
    button::Button,
    enums::CallbackTrigger,
    input::{Input, IntInput},
    menu::Choice,
    prelude::{BrowserExt, GroupExt, InputExt, MenuExt, WidgetExt},
    window::{DoubleWindow, Window},
};

use crate::entidad::{tipo_vivienda::TipoVivienda, vivienda::Vivienda, vivienda_dao::ViviendaDAO};

const WIDGET_WIDTH: i32 = 130;
const WIDGET_HEIGHT: i32 = 25;
const WIDGET_PADDING: i32 = 10;
const CHAR_WIDTH: i32 = 9;

#[derive(Clone, Copy)]
enum Message {
    Create,
    Update,
    Delete,
    Select,
    Filter,
    Save,
}

pub struct Gui {
    app: App,
    pub wind: DoubleWindow,
    sender: Sender<Message>,
    receiver: Receiver<Message>,
    model: Vec<Vivienda>,
    vivienda_dao: ViviendaDAO,
    filter_input: Input,
    list_browser: HoldBrowser,
    calle_input: Input,
    numero_input: IntInput,
    piso_input: IntInput,
    cp_input: Input,
    m2_input: IntInput,
    baños_input: IntInput,
    habitaciones_input: IntInput,
    tipo_vivienda_choice: Choice,
    create_button: Button,
    update_button: Button,
    delete_button: Button,
    save_button: Button,
}

impl Gui {
    pub fn new() -> Gui {
        let app = App::default().with_scheme(Scheme::Gtk);
        let wind = Window::default().with_label("Gestion de Viviendas");
        let (sender, receiver) = channel::<Message>();

        let label = "Busqueda:";
        let label_len = label.len() as i32 * CHAR_WIDTH;
        let filter_input = Input::default()
            .with_size(
                WIDGET_WIDTH * 4 + WIDGET_PADDING * 3 - label_len,
                WIDGET_HEIGHT,
            )
            .with_pos(WIDGET_PADDING + label_len, WIDGET_PADDING)
            .with_label(label);

        let list_browser = HoldBrowser::default()
            .with_pos(
                WIDGET_PADDING,
                filter_input.y() + filter_input.height() + WIDGET_PADDING,
            )
            .with_size(
                WIDGET_WIDTH * 4 + WIDGET_PADDING * 3,
                WIDGET_HEIGHT * 8 + WIDGET_PADDING * 8,
            );

        let largest_label_len = "Cantidad de Habitaciones:".len() as i32 * CHAR_WIDTH;
        let calle_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .with_pos(
                list_browser.x() + list_browser.width() + largest_label_len,
                list_browser.y(),
            )
            .with_label("Calle:");

        let numero_input = IntInput::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&calle_input, WIDGET_PADDING)
            .with_label("Numero:");

        let piso_input = IntInput::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&numero_input, WIDGET_PADDING)
            .with_label("Piso:");

        let cp_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&piso_input, WIDGET_PADDING)
            .with_label("Codigo Postal:");

        let m2_input = IntInput::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&cp_input, WIDGET_PADDING)
            .with_label("Metros Cuadrados:");

        let baños_input = IntInput::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&m2_input, WIDGET_PADDING)
            .with_label("Cantidad de Baños:");

        let habitaciones_input = IntInput::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&baños_input, WIDGET_PADDING)
            .with_label("Cantidad de Habitaciones:");

        let mut tipo_vivienda_choice = Choice::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&habitaciones_input, WIDGET_PADDING)
            .with_label("Tipo de Vivienda:");

        TipoVivienda::as_vector().iter().for_each(|tipo_vivienda| {
            tipo_vivienda_choice.add_choice(&format!("{:?}", tipo_vivienda))
        });

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

        let vivienda_dao = ViviendaDAO::new();
        let model = vivienda_dao.as_vector();

        Gui {
            app,
            wind,
            sender,
            receiver,
            filter_input,
            list_browser,
            vivienda_dao,
            model,
            calle_input,
            numero_input,
            piso_input,
            cp_input,
            m2_input,
            baños_input,
            habitaciones_input,
            tipo_vivienda_choice,
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

        self.create_button.emit(self.sender, Message::Create);

        self.update_button.emit(self.sender, Message::Update);
        self.update_button.deactivate();

        self.delete_button.emit(self.sender, Message::Delete);
        self.delete_button.deactivate();

        self.save_button.emit(self.sender, Message::Save);

        self.wind.set_size(
            self.calle_input.x() + self.calle_input.width() + WIDGET_PADDING,
            self.create_button.y() + self.create_button.height() + WIDGET_PADDING,
        );

        self.sender.send(Message::Filter);
    }

    fn clear_edit(&mut self) {
        self.calle_input.set_value("");
        self.numero_input.set_value("");
        self.piso_input.set_value("");
        self.cp_input.set_value("");
        self.m2_input.set_value("");
        self.baños_input.set_value("");
        self.habitaciones_input.set_value("");
        self.tipo_vivienda_choice.clear_changed();
    }

    pub fn show(&mut self) {
        self.wind.end();
        self.wind.show();
        while self.app.wait() {
            match self.receiver.recv() {
                Some(Message::Create) => {
                    self.create();
                }
                Some(Message::Update) => {
                    self.update();
                }
                Some(Message::Delete) => {
                    self.delete();
                }
                Some(Message::Save) => {
                    self.save();
                }
                Some(Message::Select) => {
                    self.select();
                }
                Some(Message::Filter) => {
                    self.filter();
                }
                None => {}
            }
        }
    }

    fn create(&mut self) {
        let identificacion = self.vivienda_dao.last_index + 1;
        self.vivienda_dao.last_index += 1;
        self.model.push(Vivienda {
            identificacion,
            calle: self.calle_input.value(),
            numero: self.numero_input.value().parse().unwrap(),
            piso: self.piso_input.value().parse().unwrap(),
            cp: self.cp_input.value(),
            m2: self.m2_input.value().parse().unwrap(),
            baños: self.baños_input.value().parse().unwrap(),
            habitaciones: self.habitaciones_input.value().parse().unwrap(),
            tipo_vivienda: TipoVivienda::from(
                &self
                    .tipo_vivienda_choice
                    .choice()
                    .expect("Tipo de vivienda no seleccionado"),
            )
            .expect("Tipo de vivienda invalido"),
        });
        self.clear_edit();
        self.sender.send(Message::Filter);
    }

    fn update(&mut self) {
        if self.list_browser.value() > 0 {
            let text_selection = self.list_browser.text(self.list_browser.value()).unwrap();
            let search_result = self
                .model
                .iter_mut()
                .find(|e| e.to_string().eq_ignore_ascii_case(&text_selection));
            match search_result {
                Some(vivienda) => {
                    vivienda.calle = self.calle_input.value();
                    vivienda.numero = self.numero_input.value().parse().unwrap();
                    vivienda.piso = self.piso_input.value().parse().unwrap();
                    vivienda.cp = self.cp_input.value();
                    vivienda.m2 = self.m2_input.value().parse().unwrap();
                    vivienda.baños = self.baños_input.value().parse().unwrap();
                    vivienda.habitaciones = self.habitaciones_input.value().parse().unwrap();
                    vivienda.tipo_vivienda = TipoVivienda::from(
                        &self
                            .tipo_vivienda_choice
                            .choice()
                            .expect("Tipo de vivienda no seleccionado"),
                    )
                    .expect("Tipo de vivienda invalido");

                    self.clear_edit();
                    self.sender.send(Message::Filter);
                    self.sender.send(Message::Select);
                }
                _ => {
                    println!("Vivienda no encontrada");
                }
            }
        } else {
            println!("No hay viviendas");
        }
    }

    fn delete(&mut self) {
        if self.list_browser.value() > 0 {
            let text_selection = self.list_browser.text(self.list_browser.value()).unwrap();
            let search_result = self
                .model
                .iter()
                .enumerate()
                .find(|e| e.1.to_string().eq_ignore_ascii_case(&text_selection));
            match search_result {
                Some((index, _vivienda)) => {
                    self.model.remove(index);
                    self.clear_edit();
                    self.sender.send(Message::Filter);
                    self.sender.send(Message::Select);
                }
                _ => {
                    println!("Vivienda no encontrada");
                }
            }
        } else {
            println!("No hay viviendas");
        }
    }

    fn save(&mut self) {
        self.vivienda_dao.save_and_refresh(&self.model);
        self.model = self.vivienda_dao.as_vector();
        self.clear_edit();
        self.sender.send(Message::Filter);
        self.sender.send(Message::Select);
    }

    fn select(&mut self) {
        if self.list_browser.value() == 0 {
            self.update_button.deactivate();
            self.delete_button.deactivate();
        } else {
            let text_selection = self.list_browser.text(self.list_browser.value()).unwrap();
            let search_result = self
                .model
                .iter()
                .find(|e| e.to_string().eq_ignore_ascii_case(&text_selection));

            match search_result {
                Some(vivienda) => {
                    self.calle_input.set_value(&vivienda.calle);
                    self.numero_input
                        .set_value(&format!("{}", &vivienda.numero));
                    self.piso_input.set_value(&format!("{}", &vivienda.piso));
                    self.cp_input.set_value(&format!("{}", &vivienda.cp));
                    self.m2_input.set_value(&format!("{}", &vivienda.m2));
                    self.baños_input.set_value(&format!("{}", &vivienda.baños));
                    self.habitaciones_input
                        .set_value(&format!("{}", &vivienda.habitaciones));
                    self.tipo_vivienda_choice.set_value(
                        vivienda
                            .tipo_vivienda
                            .get_position()
                            .unwrap_or_else(|error_msg| panic!("{}", error_msg))
                            as i32,
                    );

                    self.update_button.activate();
                    self.delete_button.activate();
                }
                _ => {
                    println!("No hay viviendas");
                }
            }
        }
    }

    fn filter(&mut self) {
        let prefix: String = self.filter_input.value().to_lowercase();
        let filter_empty: bool = prefix.trim().eq_ignore_ascii_case("");
        self.list_browser.clear();
        for (_i, p) in self.model.iter().enumerate() {
            if (p.calle.eq_ignore_ascii_case(prefix.as_str())) || filter_empty {
                let item = p.to_string();
                self.list_browser.add(&item);
            }
        }
        self.sender.send(Message::Select);
    }
}
