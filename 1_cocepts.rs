fn shadowing() {
    let x = 5;
    let x = x + 10;
    {
        let x = 6;
        println!("x is {}", x);
    }
    println!("x is {}", x);
}

fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x is {}, y is {}, z is {}", tup.0, , tup.1, tup.2)
    println!("The value of y is: {}", y);
    println!("It is possible to return multiple values using a tuple");
}

fn sta_and_exp() {
    println!("\nstatements don`t return value");
    println!("1. assignment(let x = 6)");

    println!("1. scope with no ending semicolons {}");
    println!("control flow (if x { true } else { false }")
}

fn main() {
    // 1. shadowing part
    shadowing();

    // 2. tuple types
    tuple();

    // 3. statements and expressions
    sta_and_exp();
}