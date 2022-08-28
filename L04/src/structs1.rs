#[derive(Debug)]
#[allow(dead_code)]
pub struct Persona {
    apellidos: String,
    nombres: String,
    edad: u32,
}

pub(crate) fn struct1() {
    let persona = Persona {
        apellidos: String::from("VALERA"),
        nombres: String::from("RAMON"),
        edad: 30_u32,
    };
    println!("{:?}", persona);
}
