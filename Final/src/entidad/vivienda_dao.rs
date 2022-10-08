use std::{collections::HashMap, path::Path};

use super::{vivienda::Vivienda, ScreenOutput};

const PATH_CSV: &str = "./Final/csv/viviendas.csv";

pub struct ViviendaDAO {
    indice: HashMap<usize, Vivienda>,
    pub last_index: usize,
}

impl ScreenOutput for ViviendaDAO {
    fn to_screen(&self) -> String {
        format!("{:?}", self.indice)
    }
}

impl ViviendaDAO {
    pub fn new() -> ViviendaDAO {
        let mut p = ViviendaDAO {
            indice: HashMap::new(),
            last_index: 0,
        };
        p.refresh();
        p
    }

    pub fn refresh(&mut self) {
        let path_csv = Path::new(PATH_CSV);
        let mut rdr = csv::Reader::from_path(path_csv).unwrap();

        self.indice.clear();
        for result in rdr.deserialize() {
            let record: Vivienda = result.unwrap();
            self.indice.insert(record.clone().identificacion, record);
        }
        if let Some((_, vivienda)) = self.indice.iter().last() {
            self.last_index = vivienda.identificacion;
        }
    }

    pub fn save(&self, datos: &Vec<Vivienda>) {
        let path_csv = Path::new(PATH_CSV);
        let mut wtr = csv::Writer::from_path(path_csv).unwrap();

        for vivienda in datos {
            wtr.serialize(vivienda)
                .expect(&format!("Error guardando vivienda: {:?}", vivienda));
        }
    }

    pub fn save_and_refresh(&mut self, datos: &Vec<Vivienda>) {
        self.save(datos);
        self.refresh();
    }

    pub fn as_vector(&self) -> Vec<Vivienda> {
        let mut datos = self.indice.values().cloned().collect::<Vec<Vivienda>>();
        datos.sort_by_key(|vivienda| vivienda.identificacion);
        datos
    }
}
