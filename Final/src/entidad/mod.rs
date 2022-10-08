pub mod tipo_vivienda;
pub mod vivienda;
pub mod vivienda_dao;

pub trait ScreenOutput {
    fn to_screen(&self) -> String;
}
