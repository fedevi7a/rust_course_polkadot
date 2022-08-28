#[derive(Debug)]
#[allow(dead_code)]
struct Direccion<'b> {
    calle: &'b mut String,
    numero: &'b mut u64,
    provincia: &'b mut String,
    codigo_postal: &'b mut String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Persona<'a> {
    apellidos: &'a mut String,
    nombres: &'a mut String,
    edad: &'a mut u16,
    direccion: &'a mut Direccion<'a>,
}

pub(crate) fn struct4() {
    let p = Persona {
        apellidos: &mut String::from("VALERA"),
        nombres: &mut String::from("RAMON"),
        edad: &mut 30_u16,
        direccion: &mut Direccion {
            calle: &mut String::from(""),
            numero: &mut 1,
            provincia: &mut String::from(""),
            codigo_postal: &mut String::from(""),
        },
    };

    {
        p.apellidos.push_str(" ARANGUREN");
        let mut codigo_postal = String::from("18015");
        p.direccion.codigo_postal = &mut codigo_postal;
        println!("{:?}", p);

        p.direccion.codigo_postal.clear();
        p.direccion.codigo_postal.push_str("18014");
        println!("{:?}", p);
    }
}
