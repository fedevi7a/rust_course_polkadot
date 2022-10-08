use serde::{Deserialize, Serialize};

use super::{tipo_vivienda::TipoVivienda, ScreenOutput};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Vivienda {
    pub identificacion: usize,
    pub calle: String,
    pub numero: usize,
    pub piso: usize,
    pub cp: String,
    pub m2: usize,
    pub baños: usize,
    pub habitaciones: usize,
    pub tipo_vivienda: TipoVivienda,
}

impl ScreenOutput for Vivienda {
    fn to_screen(&self) -> String {
        format!(
            "{})  {} {}, {} piso - C.P: {} - {:#?}",
            self.identificacion, self.calle, self.numero, self.piso, self.cp, self.tipo_vivienda,
        )
    }
}
