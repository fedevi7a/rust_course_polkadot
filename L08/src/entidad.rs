use std::{
    collections::HashMap,
    fs::{self},
    path::Path,
};

use serde::{Deserialize, Serialize};

pub trait ScreenOutput {
    fn to_screen(&self) -> String;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Persona {
    pub identificacion: String,
    pub apellidos: String,
    pub nombres: String,
}

impl ScreenOutput for Persona {
    fn to_screen(&self) -> String {
        format!(
            "{:?},{:?},{:?}",
            self.identificacion, self.nombres, self.apellidos
        )
    }
}

pub struct PersonaDAO {
    indice: HashMap<String, Persona>,
}

impl ScreenOutput for PersonaDAO {
    fn to_screen(&self) -> String {
        format!("{:?}", self.indice)
    }
}

impl PersonaDAO {
    pub fn new() -> PersonaDAO {
        let mut p = PersonaDAO {
            indice: HashMap::new(),
        };
        p.refresh();
        p
    }

    pub fn refresh(&mut self) {
        let path_json = Path::new("./L08/src/json/personas.json");
        let data_str = fs::read_to_string(path_json).expect("Unable to read file");
        let personas: Vec<Persona> =
            serde_json::from_str(&data_str).expect("JSON does not have correct format.");
        self.indice.clear();
        for p in personas {
            self.indice.insert(p.clone().identificacion, p);
        }
    }

    #[allow(dead_code)]
    pub fn save_state(&self) {
        let datos = self.indice.values().cloned().collect::<Vec<Persona>>();
        self.save(&datos);
    }

    pub fn save(&self, datos: &Vec<Persona>) {
        let path_json = Path::new("./L08/src/json/personas.json");
        std::fs::write(path_json, serde_json::to_string_pretty(&datos).unwrap()).unwrap();
    }

    pub fn save_and_refresh(&mut self, datos: &Vec<Persona>) {
        self.save(datos);
        self.refresh();
    }

    pub fn as_vector(&self) -> Vec<Persona> {
        let datos = self.indice.values().cloned().collect::<Vec<Persona>>();
        datos
    }

    #[allow(dead_code)]
    pub fn add(&mut self, p: Persona) {
        if !self.indice.contains_key(&p.identificacion) {
            self.indice.insert(p.clone().identificacion, p);
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self, p: Persona) {
        if self.indice.contains_key(&p.identificacion) {
            self.indice.insert(p.clone().identificacion, p);
        }
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, key: &String) -> Option<Persona> {
        self.indice.remove(key)
    }
}
