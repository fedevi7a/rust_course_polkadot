fn una_funcion() {
    println!("NO PASA NADA!")
}

fn una_funcion_con_retorno() -> u64 {
    20_u64
}

fn funcion_con_parametro(p: u64) -> u64 {
    p * 2
}

#[allow(dead_code)]
fn funcion_con_parametro_string(p: &str) -> String {
    let nuevo_string = String::from(p);
    nuevo_string.to_uppercase()
}

#[allow(dead_code)]
fn funcion_con_parametro_string2(p: &mut String) -> String {
    p.push_str(" dentro del procedimiento ");
    let nuevo_string = String::from(p.as_str());
    nuevo_string.to_uppercase()
}

#[allow(dead_code)]
fn funcion_con_parametro_box(p: &mut Box<u64>) -> u64 {
    *p.as_ref() * 2_u64
}

pub(crate) fn functions() {
    una_funcion();
    let valor_retorno = una_funcion_con_retorno();
    println!("VALOR DE RETORNO -> {}", valor_retorno);

    let valor_solicitado = 15_u64;
    let valor_retorno_parametros = funcion_con_parametro(valor_solicitado);
    println!("VALOR DE RETORNO -> {}", valor_solicitado);
    println!("VALOR DE RETORNO -> {}", valor_retorno_parametros);

    let valor_string = String::from("demostracion");
    let nuevo_valor = funcion_con_parametro_string(&valor_string);
    println!("VALOR DE RETORNO -> {}", valor_string);
    println!("VALOR DE RETORNO -> {}", nuevo_valor);

    let mut valor_string2 = String::from("demostracion2");
    let nuevo_valor2 = funcion_con_parametro_string2(&mut valor_string2);
    println!("VALOR DE RETORNO -> {}", valor_string2);
    println!("VALOR DE RETORNO -> {}", nuevo_valor2);

    let mut valor_solicitado_box = Box::<u64>::new(35_u64);
    let valor_retorno_parametros_box = funcion_con_parametro_box(&mut valor_solicitado_box);
    println!("VALOR DE RETORNO -> {}", valor_solicitado_box);
    println!("VALOR DE RETORNO -> {}", valor_retorno_parametros_box);
}
