//! rust don`t has inheritance (继承)
//! instead we could use trait instead of inheritance

/// we could use trait to define common behavior  
/// a GUI example
/// 
/// dynamic dispatch like generics
/// ust won’t compile our code if the values don’t implement the traits that the trait objects need.
#[allow(unused)]
mod inheritace {
    /// we can define a vector that takes a trait object
    /// We can use trait objects in place of a generic or concrete type.
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        // a stand-in for any type inside a Box that implements the Draw trait.
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            // This works differently from defining a struct that uses a generic type parameter with trait bounds.
            // generic type parameter can only be substituted with one concrete type at a time,
            // whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime. 
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
    /*
    * different
    pub struct Screen<T: Draw> {
        pub components: Vec<T>,
    }
    
    impl<T> Screen<T>
    where
        T: Draw,
    {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
    */
    // if you’ll only ever have homogeneous collections,
    // using generics and trait bounds is preferable because the definitions will be monomorphized at compile time to use the concrete types.

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }
    
    impl Draw for Button {
        fn draw(&self) {
            // code to actually draw a button
        }
    }

    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }
    
    impl Draw for SelectBox {
        fn draw(&self) {
            // code to actually draw a select box
        }
    }

    /// concerned only with the messages a value responds to rather than the value’s concrete type
    /// Rust won’t compile our code if the values don’t implement the traits that the trait objects need.
    fn usage() {
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ],
        };
        screen.run();
    }
}

// object safety
// we must use object safety object, a safe object requires:
// - The return type isn’t Self.
// - There are no generic type parameters.
// 
// why:
// 1.  trait object forgets the exact type that Self is, there is no way the method can use the original concrete type. 
// 

/// not a object safe feature
pub trait Clone {
    fn clone(&self) -> Self;
}

/// so you cannot use this as a trait 
pub struct Screen {
    pub components: Vec<Box<dyn Clone>>,
}

fn main() {

}