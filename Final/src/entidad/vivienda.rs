use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::tipo_vivienda::TipoVivienda;

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

impl Display for Vivienda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{})  {} {}, {} piso - C.P: {} - {:#?}",
            self.identificacion, self.calle, self.numero, self.piso, self.cp, self.tipo_vivienda,
        ))
    }
}
