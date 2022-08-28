// `allow` required to silence warnings because only
// one variant is used.
#[allow(dead_code)]
enum Color {
    // These 3 are specified solely by their name.
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models.
    Rgb(u32, u32, u32),
    Hsv(u32, u32, u32),
    Hsl(u32, u32, u32),
    Cmy(u32, u32, u32),
    Cmyk(u32, u32, u32, u32),
}

fn color_to_string(color: Color) -> String {
    let mut texto_color = String::from("");
    let c: &str = match color {
        Color::Rgb(20, y, z) => {
            let concatenated = format!("r = {x}, g = {y}, b = {z} ", x = 20, y = y, z = z);
            return concatenated;
        }
        Color::Rgb(x, y, z) => {
            return String::from(
                format!("r = {x}, g = {y}, b = {z} ", x = x, y = y, z = z).as_str(),
            )
        }
        Color::Blue => "AZUL",
        _ => ".",
    };
    texto_color.push_str(c);
    texto_color
}

struct Foo {
    x: (u32, u32),
    y: u32,
}

pub(crate) fn match_instruction() {
    let un_color = Color::Blue;
    let texto_color = color_to_string(un_color);
    println!("{}", texto_color);

    let otro_color = Color::Rgb(20, 10, 255);
    let otro_texto_color = color_to_string(otro_color);
    println!("{}", otro_texto_color);

    println!(
        " -- CONCATENACION --> {x} ",
        x = texto_color + " " + otro_texto_color.as_str()
    );

    //////////////////////////////////////////////////////////////////////////////////////

    let array = [1, -2, 6];

    match array {
        // Binds the second and the third elements to the respective variables
        [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        // Single values can be ignored with _
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),

        // You can also bind some and ignore the rest
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        // The code below would not compile
        // [-1, second] => ...

        // Or store them in another array/slice (the type depends on
        // that of the value that i2s being matched against)
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // Combining these patterns, we can, for example, bind the first and
        // last values, and store the rest of them in a single array
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }

    //////////////////////////////////////////////////////////////////////////////////////
    let _foo = Foo { x: (1, 2), y: 3 };

    match _foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
    }
}

// {
// //////////////////////////////////////////////////////////////////////////////////////

// let array = [1, -2, 6];

// match array {
//     // Binds the second and the third elements to the respective variables
//     [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

//     // Single values can be ignored with _
//     [1, _, third] => println!(
//         "array[0] = 1, array[2] = {} and array[1] was ignored",
//         third
//     ),

//     // You can also bind some and ignore the rest
//     [-1, second, ..] => println!(
//         "array[0] = -1, array[1] = {} and all the other ones were ignored",
//         second
//     ),
//     // The code below would not compile
//     // [-1, second] => ...

//     // Or store them in another array/slice (the type depends on
//     // that of the value that i2s being matched against)
//     [3, second, tail @ ..] => println!(
//         "array[0] = 3, array[1] = {} and the other elements were {:?}",
//         second, tail
//     ),

//     // Combining these patterns, we can, for example, bind the first and
//     // last values, and store the rest of them in a single array
//     [first, middle @ .., last] => println!(
//         "array[0] = {}, middle = {:?}, array[2] = {}",
//         first, middle, last
//     ),
// }

//     //////////////////////////////////////////////////////////////////////////////////////

//     let color = Color::RGB(122, 17, 40);
//     // ^ Try different variants for `color`

//     println!("What color is it?");
//     // An `enum` can be destructured using a `match`.
//     match color {
//         Color::Red   => println!("The color is Red!"),
//         Color::Blue  => println!("The color is Blue!"),
//         Color::Green => println!("The color is Green!"),
//         Color::RGB(r, g, b) =>
//             println!("Red: {}, green: {}, and blue: {}!", r, g, b),
//         Color::HSV(h, s, v) =>
//             println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
//         Color::HSL(h, s, l) =>
//             println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
//         Color::CMY(c, m, y) =>
//             println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
//         Color::CMYK(c, m, y, k) =>
//             println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
//                 c, m, y, k),
//         // Don't need another arm because all variants have been examined
//     }

//     //////////////////////////////////////////////////////////////////////////////////////

// let foo = Foo { x: (1, 2), y: 3 };

// match foo {
//     Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

//     // you can destructure structs and rename the variables,
//     // the order is not important
//     Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

//     // and you can also ignore some variables:
//     Foo { y, .. } => println!("y = {}, we don't care about x", y),
//     // this will give an error: pattern does not mention field `x`
//     //Foo { y } => println!("y = {}", y),
// }

//     //////////////////////////////////////////////////////////////////////////////////////

//       // Assign a reference of type `i32`. The `&` signifies there
//     // is a reference being assigned.
//     let reference = &4;

//     match reference {
//         // If `reference` is pattern matched against `&val`, it results
//         // in a comparison like:
//         // `&i32`
//         // `&val`
//         // ^ We see that if the matching `&`s are dropped, then the `i32`
//         // should be assigned to `val`.
//         &val => println!("Got a value via destructuring: {:?}", val),
//     }

//     // To avoid the `&`, you dereference before matching.
//     match *reference {
//         val => println!("Got a value via dereferencing: {:?}", val),
//     }

//     // What if you don't start with a reference? `reference` was a `&`
//     // because the right side was already a reference. This is not
//     // a reference because the right side is not one.
//     let _not_a_reference = 3;

//     // Rust provides `ref` for exactly this purpose. It modifies the
//     // assignment so that a reference is created for the element; this
//     // reference is assigned.
//     let ref _is_a_reference = 3;

//     // Accordingly, by defining 2 values without references, references
//     // can be retrieved via `ref` and `ref mut`.
//     let value = 5;
//     let mut mut_value = 6;

//     // Use `ref` keyword to create a reference.
//     match value {
//         ref r => println!("Got a reference to a value: {:?}", r),
//     }

//     // Use `ref mut` similarly.
//     match mut_value {
//         ref mut m => {
//             // Got a reference. Gotta dereference it before we can
//             // add anything to it.
//             *m += 10;
//             println!("We added 10. `mut_value`: {:?}", m);
//         },
//     }

//     enum Temperature {
//         Celsius(i32),
//         Farenheit(i32),
//     }

//     let temperature = Temperature::Celsius(35);
//     // ^ try different values for `temperature`

//     match temperature {
//         Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
//         // The `if condition` part ^ is a guard
//         Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),

//         Temperature::Farenheit(t) if t > 86 => println!("{}F is above 86 Farenheit", t),
//         Temperature::Farenheit(t) => println!("{}F is below 86 Farenheit", t),
//     }

// }
