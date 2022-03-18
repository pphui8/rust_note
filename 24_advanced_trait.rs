//! advanced features
//! 
//! - Unsafe Rust: how to opt out of some of Rust’s guarantees and take responsibility for manually upholding those guarantees
//! - Advanced traits: associated types, default type parameters, fully qualified syntax, supertraits, and the newtype pattern in relation to traits
//! - Advanced types: more about the newtype pattern, type aliases, the never type, and dynamically sized types
//! - Advanced functions and closures: function pointers and returning closures
//! - Macros: ways to define code that defines more code at compile time

/// f you use unsafe code incorrectly, problems due to memory unsafety, such as null pointer dereferencing, can occur.
/// To switch to unsafe Rust, use the unsafe keyword and then start a new block that holds the unsafe code.
/// 
/// You can take five actions in unsafe Rust, called unsafe superpowers, that you can’t in safe Rust. Those superpowers include the ability to:
/// - Dereference a raw pointer
/// - Call an unsafe function or method
/// - Access or modify a mutable static variable
/// - Implement an unsafe trait
/// - Access fields of unions
/// 
/// unsafe doesn’t turn off the borrow checker or disable any other of Rust’s safety checks
#[allow(unused)]
mod unsafe_rust {
    // Unsafe Rust has two new types called raw pointers that are similar to references
    // raw pointers can be immutable or mutable and are written as *const T and *mut T
    // !The asterisk isn’t the dereference operator; it’s part of the type name. 
    
    /// raw pointers:
    /// - Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
    /// - Aren’t guaranteed to point to valid memory
    /// - Are allowed to be null
    /// - Don’t implement any automatic cleanup
    fn raw_pointer() {
        let mut num = 5;
        let r1 = &num as *const i32;
        // allowed
        let r2 = &mut num as *mut i32;
        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }

    /// calls to unsafe functions.
    unsafe fn danferous() {}
    fn call_unsafe_func() {
        unsafe {
            danferous();
        }
    }
    // Just because a function contains unsafe code doesn’t mean we need to mark the entire function as unsafe. 
    // we could create a safe abstraction over unsafe code
}

mod ffi {
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    
    fn main() {
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }

    // or a API allow other lang to use
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
}

/// In Rust, global variables are called static variables. 
/// Static variables are similar to constants
/// Static variables can only store references with the 'static lifetime
/// means the Rust compiler can figure out the lifetime and we aren’t required to annotate it explicitly.
/// 
/// Accessing an immutable static variable is safe.
/// Accessing and modifying mutable static variables is unsafe.
mod global_variables {
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    fn main() {
        add_to_count(3);

        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }
}

mod implement_unsafe_trait {
    unsafe trait Foo {
        // methods go here
    }
    
    unsafe impl Foo for i32 {
        // method implementations go here
    }
}

mod advanced_trait {
    /// Associated types connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures.
    /// 
    /// what different between placeholder and generics
    /// when using generics, when a trait has a generic parameter, it can be implemented for a type multiple times
    /// when a trait has a generic parameter,  it can be implemented for a type multiple times, changing the concrete types of the generic type parameters each time
    /// suummary: generics needs to type annoncement everytime we use
    fn associated_types() {
        pub trait Iterator {
            // stands in for the type of the values the type implementing the Iterator trait is iterating over
            // The type Item is a placeholder type, and the next method’s definition shows that it will return values of type Option<Self::Item>
            type Item;
            // we could specified Item u32
            // type Item = u32;
            fn next(&mut self) -> Option<Self::Item>;
        }
        /* example using generics
        * pub trait Iterator<T> {
        *     fn next(&mut self) -> Option<T>;
        * }
        */
    }

    /// Rust doesn’t allow you to create your own operators or overload arbitrary operators.
    /// But you can overload the operations and corresponding traits listed in std::ops by implementing the traits associated with the operator
    use std::ops::Add;
    // The default generic type in this code is within the Add trait.
    // this syntax is called default type parameters.
    // If we don’t specify a concrete type for Rhs when we implement the Add trait, the type of Rhs will default to Self
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    impl Add for Point {
        type Output = Point;
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    fn run() {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
    }

    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
        /// To add Millimeters and Meters, we specify impl Add<Meters> to set the value of the Rhs type parameter instead of using the default of Self.
        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    /// It’s possible to implement a method directly on the type with the same name as methods from traits.
    /// you’ll need to tell Rust which one you want to use when we call them
    fn disambiguation() {
        // Running this code will print *waving arms furiously*,
        // showing that Rust called the fly method implemented on Human directly.
        trait Pilot {
            fn fly(&self);
        }
        trait Wizard {
            fn fly(&self);
        }
        struct Human;
        impl Pilot for Human {
            fn fly(&self) {
                println!("This is your captain speaking.");
            }
        }
        impl Wizard for Human {
            fn fly(&self) {
                println!("Up!");
            }
        }
        impl Human {
            fn fly(&self) {
                println!("*waving arms furiously*");
            }
        }
        /// Specifying the trait name before the method name clarifies to Rust which implementation of fly we want to call
        /// We could also write Human::fly(&person)
        /// 
        /// if the function has no self parameter, we needs to use fully qualified syntax
        /// we could use Dog::baby_name() of <Dog as Animal>::baby_name()
        /// 
        /// In general, fully qualified syntax is defined as follows:
        /// <Type as Trait>::function(receiver_if_method, next_arg, ...);
        fn run() {
            let person = Human;
            Pilot::fly(&person);
            Wizard::fly(&person);
            person.fly();
        }
    }

    /// Sometimes, you might need one trait to use another trait’s functionality
    fn require_others_func() {
        use std::fmt;
        // we’ve specified that OutlinePrint requires the Display trait, we can use the to_string function that is automatically implemented for any type that implements Display
        trait OutlinePrint: fmt::Display {
            fn outline_print(&self) {
                let output = self.to_string();
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {} *", output);
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
            }
        }
        struct Point {
            x: i32,
            y: i32,
        }
        impl OutlinePrint for Point {}
        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }
        fn main() {
            let p = Point { x: 1, y: 3 };
            p.outline_print();
        }
    }
}

/// to get around the orphan rule, we could use Newtype pattern
/// newtype pattern involves creating a new type in a tuple struct
/// The tuple struct will have one field and be a thin wrapper around the type we want to implement a trait for
mod newtype {
    use std::fmt;
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // uses self.0 to access the inner Vec<T>
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    fn main() {
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }
}

fn main() {

}