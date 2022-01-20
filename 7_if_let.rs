// syntax sugar for a match that runs code 
// when the value matches one pattern and then ignores 
// all other values.

enum Num {
    One,
    Two,
    Three,
    Other(i32),
}

impl Num {
    fn get_num(&self) -> i32 {
        // same code
        match self {
            Num::Other(x) => *x,
            _ => 0,
        };
        // same code
        if let Num::Other(x) = self {
            *x
        } else {
            0
        }
    }
}

fn main() {
    let x = Some(-100i32);
    // same code
    match x {
        Some(value) => println!("{}", value),
        _ => (),
    }
    // same code
    if let Some(value) = x {
        println!("{}", value);
    }

    let x = Num::Other(32i32);
    println!("{}", x.get_num())
}