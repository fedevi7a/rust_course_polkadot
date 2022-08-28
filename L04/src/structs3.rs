#[derive(Debug)]
#[allow(dead_code)]
struct Persona<'a> {
    apellidos: &'a mut String,
    nombres: &'a mut String,
    edad: &'a mut u16,
}

pub(crate) fn struct3() {
    let mut p = Persona {
        apellidos: &mut String::from("VALERA"),
        nombres: &mut String::from("RAMON"),
        edad: &mut 30_u16,
    };
    p.apellidos.push_str(" ARANGUREN");
    let _codigo_postal = String::from("18015");

    {
        let mut val_apellidos = String::from("VALERA ARANGUREN");
        p.apellidos = &mut val_apellidos;
        println!("{:?}", p);
    }
}
