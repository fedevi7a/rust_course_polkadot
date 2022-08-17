pub(crate) fn data_types() {
    mutable();
    scopes();
    primitives();
    tuples();
    arrays();
    type_conversion();
    type_alias();
}

fn mutable() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn scopes() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}

fn primitives() {
    // Enteros con Signo y Sin Signo
    let _x = 42; // `x` has type `i32`
    let _y: i32 = -30;

    // Punto Flotante
    let _xf = 2.0; // f64
    let _yf: f32 = 3.0; // f32

    // Boolean
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // Char
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';
}

fn tuples() {
    // Tuplas
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);

    let xtuple: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = xtuple.0;
    let _six_point_four = xtuple.1;
    let _one = xtuple.2;
}

fn arrays() {
    let _a = [1, 2, 3]; // a: [i32; 3]
    let mut _m = [1, 2, 3]; // m: [i32; 3]

    // Inicialización de todos los elementos de un Arreglo
    let _a = [0; 20]; // a: [i32; 20]

    // Longitud de un Arreglo
    let a = [1, 2, 3];
    println!("a has {} elements", a.len());

    // Acceder a un determinado elemento de un arreglo
    let names = ["Graydon", "Brian", "Niko"];
    println!("The second name is: {}", names[1]);
}

fn type_conversion() {
    // Explicit conversion
    let decimal = 65.4321_f32;

    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // Ejemplos de Conversion sobre Tipos

    println!("1000 as a u16 is: {}", 1000_u16);
    // println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.

    // Unless it already fits, of course.
    println!(" 128 as a i16 is: {}", 128_i16);

    // 128 as u8 -> -128, whose two's complement in eight bits is:
    // println!(" 128 as a i8 is : {}", 128 as i8);

    // repeating the example above
    // 1000 as u8 -> 232
    // println!("1000 as a u8 is : {}", 1000 as u8);
    // and the two's complement of 232 is -24
    // println!(" 232 as a i8 is : {}", 232 as i8);

    // Since Rust 1.45, the `as` keyword performs a *saturating cast*
    // when casting from float to int. If the floating point value exceeds
    // the upper bound or is less than the lower bound, the returned value
    // will be equal to the bound crossed.

    // 300.0 is 255
    println!("300.0 is {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("nan as u8 is {}", f32::NAN as u8);
}

fn type_alias() {
    // Alias para Tipos de Datos
    type NanoSecond = u64;
    type Inch = u64;
    type U64 = u64;

    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
