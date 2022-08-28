use std::io::stdin;

pub(crate) fn parse_error() {
    // Declare a mutable input string.
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).ok();
    println!("{}", input_string);

    let valor_numerico = input_string.trim().parse::<u64>().unwrap_or(0_u64);
    println!("{}", valor_numerico);

    let mut otro_input_string = String::new();
    stdin().read_line(&mut otro_input_string).ok();
    let otro_valor_numerico = otro_input_string.trim().parse::<u64>();
    match otro_valor_numerico {
        Ok(valor_entero) => println!("{}", valor_entero),
        Err(_) => println!("Error en Conversion"),
    };
}
