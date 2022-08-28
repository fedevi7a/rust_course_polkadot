#[derive(Debug)]
#[allow(dead_code)]
struct Carro {
    marca: String,
    modelo: String,
    placa: str,
}

struct Valores(f32, i64);

#[derive(Debug)]
#[allow(dead_code)]
struct Direccion {
    calle: String,
    numero: u64,
    provincia: String,
    codigo_postal: String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Persona {
    apellidos: String,
    nombres: String,
    edad: u16,
    direccion: Direccion,
}

pub(crate) fn struct2() {
    //////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////

    let p = Persona {
        apellidos: String::from("VALERA"),
        nombres: String::from("RAMON"),
        edad: 30_u16,
        direccion: Direccion {
            calle: String::from(""),
            numero: 1,
            provincia: String::from(""),
            codigo_postal: String::from(""),
        },
    };

    println!("{:?}", p);

    let v = Valores(10.5_f32, 18_i64);
    println!("{} {}", v.0, v.1);
}
