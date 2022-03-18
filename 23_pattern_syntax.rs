//! patterns contains
//! - Literals (字面值)
//! - Destructured arrays, enums, structs, or tuples (解构的数组、枚举、结构体或者元组)
//! - Variables (变量)
//! - Wildcards (通配符)
//! - Placeholders (占位符)

mod where_places_patterns_can_be_used {
    // 1. march arms
    // 2. if let expressions
    // it`s also possible to mix the if let
    fn if__let() {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();
    
        if let Some(color) = favorite_color {
            println!("Using your favorite color, {}, as the background", color);
        } else if is_tuesday {
            println!("Tuesday is green day!");
        // if let Ok(age) = age && age > 30 is not allowed
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }    
    // 3. while let conditional loops
    fn if_while() {
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }
    // 4. for loops
    fn for_loop() {
        let v = vec!['a', 'b', 'c'];
        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }
    // 5. let statements
    fn let_statements() {
        // let is also a pattern
        // means bind what matches here to the variable
        let PATTERN = "EXPRESSION";
        let (x, y, z) = (1, 2, 3);
    }
    // 6. function parameters (including closures)
    fn foo(x: i32) {
        // x is a pattern
        // fn cur_location(&(x, y): &(i32, i32)) {
    }
}

/// whether a pattern might fall to match
/// Patterns that will match for any possible value passed are irrefutable
/// example: ```let x = 5;```
/// Patterns that can fail to match for some possible value are refutable.
/// example: ```if let Some(X) = value```
mod refutability {
    // Function parameters, let statements, and for loops can only accept irrefutable patterns
    // The if let and while let expressions accept refutable and irrefutable patterns
    
    fn cover_refutable_to_irrefutable() {
        // when we try to use a refutable pattern where Rust requires an irrefutable pattern and vice versa. 
        // we will get an error
        // let Some(x) = some_option_value;
        // To fix the problem where we have a refutable pattern where an irrefutable pattern is needed
        let some_option_value: Option<i32> = None;
        if let Some(x) = some_option_value {
            println!("{}", x);
        }
    }
}

#[allow(unused)]
mod pattern_syntax {
    fn match_named_var() {
        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            // we are in a new scope so y is a new var
            Some(y) => println!("Matched, y = {:?}", y),
            _ => println!("Default case, x = {:?}", x),
        }
        println!("at the end: x = {:?}, y = {:?}", x, y);
    }

    fn multiple_patterns() {
        let x = 1;
        // use | syntax
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("others"),
        }
        // use ..=
        // we could use char or number
        match x {
            1..=5 => println!("one to five"),
            _ => println!("others"),
        }
    }

    fn break_apart_values() {
        // struct
        struct Point {
            x: i32,
            y: i32,
        };
        let p = Point { x: 0, y: 7 };
        let Point { x: a, y: b} = p;
        let Point { x, y } = p; // shorthand
        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }

        // enum
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        
        fn main() {
            let msg = Message::ChangeColor(0, 160, 255);
        
            match msg {
                Message::Quit => {
                    println!("The Quit variant has no data to destructure.")
                }
                Message::Move { x, y } => {
                    println!(
                        "Move in the x direction {} and in the y direction {}",
                        x, y
                    );
                }
                Message::Write(text) => println!("Text message: {}", text),
                Message::ChangeColor(r, g, b) => println!(
                    "Change the color to red {}, green {}, and blue {}",
                    r, g, b
                ),
            }
        }

        // nested items
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }
        
        enum Message2 {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }
        let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));
    
        match msg {
            Message2::ChangeColor(Color::Rgb(r, g, b)) => println!(
                "Change the color to red {}, green {}, and blue {}",
                r, g, b
            ),
            Message2::ChangeColor(Color::Hsv(h, s, v)) => println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            ),
            _ => (),
        }

        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    }
}

mod ignoring_values {
    // 1. use _ to ignore entire value
    // 2. use _ inside another pattern to ignore just part of a valu
    // 3. If creating a variable but don’t use it anywhere, staring the var name with _
    // 4. ingnore remaining parts of value with ..


    /// subsle difference The syntax _x still binds the value to the variable, whereas _ doesn’t bind at all
    fn error_example() {
        let s = Some(String::from("Hello!"));
        if let Some(_s) = s {
            println!("found a string");
        }
        // println!("{:?}", s);   //error because s was moved
        // to fix this bug, replace _s to _
    }

    /// ignore the remaining parts
    fn ignore_remain() {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }
        let origin = Point { x: 0, y: 0, z: 0 };
        match origin {
            Point { x, .. } => println!("x is {}", x),
        }

        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, .., last) => {
                println!("Some numbers: {}, {}", first, last);
            }
        }
    }
}

mod extra_conditional {
    // A match guard is an additional if condition specified after the pattern in a match arm that must also match
    fn example() {
        let num = Some(4);
        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("{}", x),
            None => (),
        }

        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            // this doesn`t introduce a new variable y
            Some(n) if n == y => println!("Matched, n = {}", n),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {}", x, y);

        // use or operator |
        let x = 4;
        let y = false;
        match x {
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
    }
}

/// The at operator (@) lets us create a variable that holds a value at
/// the same time we’re testing that value to see whether it matches a pattern.
mod at_operator {
    fn  usage() {
        enum Message {
            Hello { id: i32 },
        }
    
        let msg = Message::Hello { id: 5 };
    
        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => println!("Found an id in range: {}", id_variable),
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => println!("Found some other id: {}", id),
        }
    }
}

fn main() {

}