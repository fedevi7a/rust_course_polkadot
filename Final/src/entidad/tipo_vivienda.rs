use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum TipoVivienda {
    Casa,
    Apartamento,
}

impl TipoVivienda {
    pub fn from(name: &str) -> Option<Self> {
        match name {
            "Casa" => Some(Self::Casa),
            "Apartamento" => Some(Self::Apartamento),
            _ => None,
        }
    }

    pub fn as_vector() -> Vec<Self> {
        vec![Self::Casa, Self::Apartamento]
    }

    pub fn get_position(&self) -> Result<usize, String> {
        for (index, tipo_vivienda) in Self::as_vector().iter().enumerate() {
            if self == tipo_vivienda {
                return Ok(index);
            }
        }
        Err(format!("{:?} es un valor invalido", self))
    }
}
