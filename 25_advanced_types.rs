//! The newtype pattern is a lightweight way to achieve encapsulation to hide implementation details

#[allow(unused)]
mod create {
    fn create() {
        // the alias Kilometers is a synonym for i32
        // Kilometers will be treated the same as values of type i32:
        // However, using this method, we don’t get the type checking benefits
        type Kilometers = i32;
        let x: i32 = 5;
        let y: Kilometers = 5;
        println!("x + y = {}", x + y);
    }

    /// The main use case for type synonyms is to reduce repetition
    /// Choosing a meaningful name for a type alias can help communicate your intent as well
    fn main_use() {
        let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
        fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
            // --snip--
        }
        // we could use type alias
        type Thunk = Box<dyn Fn() + Send + 'static>;
        let f: Thunk = Box::new(|| println!("hi"));
        fn takes_long_type_shorten(f: Thunk) {
            // --snip--
        }
    }

    /// Type aliases are also commonly used with the Result<T, E> type for reducing repetition
    fn type_aliases_as_return() {
        use std::fmt;
        use std::io::Error;
        type Result<T> = std::result::Result<T, std::io::Error>;
        pub trait Write {
            // fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
            fn write(&mut self, buf: &[u8]) -> Result<usize>;
            // fn flush(&mut self) -> Result<(), Error>;
            fn flush(&mut self) -> Result<()>;
            // fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
            fn write_all(&mut self, buf: &[u8]) -> Result<()>;
            // fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
            fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
        }
    }
}

/// Rust has a special type named ! that’s known in type theory lingo as the empty type because it has no values.
/// it stands in the place of the return type when a function will never return
/// The formal way of describing this behavior is that expressions of type ! can be coerced into any other type
mod never_type {
    /// Functions that return never are called diverging functions
    fn bar() -> ! {
        // panic! stands for a ! type
        panic!("error")
    }

    fn example() {
        let guess = String::from("123");
        loop {
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                // continue stands for a !
                // so guess would be covered by u32 witch the first pattern decided
                Err(_) => continue,
            };
        }
    }
}

/// dynamically sized types, these types let us write code using values whose size we can know only at runtime.
mod dynamically_sized_type {
    /// we make the types of s1 and s2 a &str rather than a str
    /// the slice data structure stores the starting position and the length of the slice.
    /// so &str`s size is twice of usize
    /// The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind
    fn str() {
        let s1: &str = "Hello there!";
    }

    /// To work with DSTs, Rust has a particular trait called the Sized trait
    fn create_sized_type() {
        fn generic<T: Sized>(t: T) {
            // --snip--
        }
        /// A trait bound on ?Sized means “T may or may not be Sized”
        /// this is only available for Sized, not any other traits.
        /// we use &T because the type might not be Sized, we need to use it behind some kind of pointer
        fn generic_two<T: ?Sized>(t: &T) {
            // --snip--
        }
    }
}

/// function pointers will allow you to use functions as arguments to other functions
/// Unlike closures, fn is a type rather than a trait,
/// so we specify fn as the parameter type directly rather than declaring a generic type parameter with one of the Fn traits as a trait bound.
/// Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce),
mod function_pointers {
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    fn main() {
        let answer = do_twice(add_one, 5);
        println!("The answer is: {}", answer);
    }

    /// return closures
    /// Closures are represented by traits, which means you can’t return closures directly
    fn return_clo() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}

/// three kinds of procedural macros
/// - Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
/// - Attribute-like macros that define custom attributes usable on any item
/// - Function-like macros that look like function calls but operate on the tokens specified as their argument
/// 
/// macros have some additional powers that functions don’t.
/// macros can produce rust code automatically
/// we can call marcos with various variable
mod macros {
    #[macro_export]
    /// body is similar to the structure of a match expression
    /// this is the first form of macros: declaratice macros
    macro_rules! vec {
        // a literal comma separator character could optionally appear after the code that matches the code in $()
        // The * specifies that the pattern matches zero or more of whatever precedes the *.
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }

    /// The second form of macros is procedural macros
    /// Procedural macros accept some code as an input, operate on that code, and produce some code as an output rather than
    /// matching against patterns and replacing the code with other code as declarative macros do
    /// use
    // #[some_attribute]
    // pub fn some_name(input: TokenStream) -> TokenStream {
    // }

    /// To write a custom derive Macro
    use super::HelloMacro;
    fn use_marcos() {
        
    }
}

/// procedural macros need to be in their own crate.
pub mod hello_macro_derive {

}
pub mod hello_macro {
    pub trait HelloMacro {
        fn hello_macro();
    }
    struct Pancakes;

    impl HelloMacro for Pancakes {
        fn hello_macro() {
            println!("Hello, Macro! My name is Pancakes!");
        }
    }
}

fn main() {
    let mut res: Vec<(String, String)> = Vec::new();
    res.push((String::from("abc"), String::from("def")));

}