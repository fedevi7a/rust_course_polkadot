use std::cmp::Ordering;

pub(crate) fn control_flow() {
    if_example();
    loop_example();
}

fn if_example() {
    // Ejemplo if en una sentencia let
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // Ejemplo if anidado
    let n = 5;
    let big_n = if n < 10 && n > -10 {
        println!("increase ten-fold");
        // This expression returns an `i32`.
        10 * n
    } else {
        println!("halve the number");
        // This expression must return an `i32` as well.
        n / 2
        // ^ Try suppressing this expression with a semicolon.
    };
    // ^ Don't forget to put a semicolon here! All `let` bindings need it.
    println!("{} -> {}", n, big_n);

    // Ejemplo de match
    let n = 5;
    match n.cmp(&0) {
        Ordering::Less => print!("{} is negative", n),
        Ordering::Equal => print!("{} is zero", n),
        Ordering::Greater => print!("{} is positive", n),
    }
    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        // This expression returns an `i32`.
        10 * n
    } else {
        println!(", and is a big number, halve the number");
        // This expression must return an `i32` as well.
        n / 2
        // ^ Try suppressing this expression with a semicolon.
    };
    // ^ Don't forget to put a semicolon here! All `let` bindings need it.
    println!("{} -> {}", n, big_n);
}

fn loop_example() {
    // Infinite loop
    let mut count = 0u32;
    println!("Let's count until infinity!");
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            // Skip the rest of this iteration
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            // Exit this loop
            break;
        }
    }

    // Loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    // Reverse
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // fizz buzz
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
