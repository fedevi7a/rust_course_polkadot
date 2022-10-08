use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum TipoVivienda {
    Casa,
    Apartamento,
}

impl TipoVivienda {
    pub fn from(name: &str) -> Self {
        match name {
            "Casa" => Self::Casa,
            "Apartamento" => Self::Apartamento,
            _ => panic!("{} es un valor invalido", name),
        }
    }

    pub fn get_values() -> Vec<Self> {
        vec![Self::Casa, Self::Apartamento]
    }

    pub fn get_position(&self) -> usize {
        for (i, tipo_vivienda) in Self::get_values().iter().enumerate() {
            if self == tipo_vivienda {
                return i;
            }
        }
        panic!("{:?} es un valor invalido", self);
    }
}
