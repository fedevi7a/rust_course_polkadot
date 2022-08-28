mod my {
    #[derive(Debug)]
    pub struct Persona {
        pub apellidos: String,
        pub nombres: String,
        pub edad: u32,
    }

    #[allow(dead_code)]
    impl Persona {
        fn imprimir(&self) {
            println!("{:?}", self);
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Persona {
    apellidos: String,
    nombres: String,
    edad: u32,
}

pub(crate) fn struct5() {
    let persona = Persona {
        apellidos: String::from("VALERA"),
        nombres: String::from("RAMON"),
        edad: 30_u32,
    };
    //    persona.imprimir();
    println!("{:?}", persona);
}
