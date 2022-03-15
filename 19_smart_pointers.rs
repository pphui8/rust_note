//! In Rust, the different smart pointers defined in the standard library provide functionality beyond that provided by references
//! an additional difference between references and smart pointers is that references are pointers that only borrow data; in contrast, in many cases, smart pointers own the data they point to
//! 
//! The characteristic that distinguishes a smart pointer from an ordinary struct is that smart pointers implement the Deref and Drop traits
//! The Deref trait allows an instance of the smart pointer struct to behave like a reference so you can write code that works with either references or smart pointers.
//! The Drop trait allows you to customize the code that is run when an instance of the smart pointer goes out of scope
//! 
//! common smart pointer
//! - Box<T> for allocating values on the heap
//! - Rc<T>, a reference counting type that enables multiple ownership
//! - Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

#[allow(unused)]
/// Boxes allow you to store data on the heap rather than the stack
fn box_t() {
    // use box when: 
    // - When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
    // - When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
    // - When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

    // in this case, data are similar to how we would if this data were on the stack
    // auto deallocation and it happens for the box (stored on the stack) and the data it points to (stored on the heap)
    let b = Box::new(5);
    println!("b = {}", b);
}

#[allow(unused)]
/// what can box do but common type cannot
fn recursive_type() {
    // At compile time, Rust needs to know how much space a type takes up
    // error because List has infinite size
    /*
    enum List {
        Cons(i32, List),
        Nil,
    }
    */
}

#[allow(unused)]
/// Each item in a cons list contains two elements: the value of the current item and the next item. The last item in the list contains only a value called Nil without a next item
mod cons_list {
    enum List {
        // Because a Box<T> is a pointer, witch size doesn’t change based on the amount of data it’s pointing to
        // he next List value that will be on the heap rather than inside the Cons variant
        Cons(i32, Box<List>),
        Nil,
    }

    use crate::cons_list::List::{Cons, Nil};
    fn usage() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }
}

#[allow(unused)]
fn deref() {
    // Implementing the Deref trait allows you to customize the behavior of the dereference operator, * 

    // Comparing a number and a reference to a number isn’t allowed because they’re different types
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    // assert_eq!(5, y);    // wrong

    // using Box<T> like a reference
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // The only difference between above and under is that here we set y to be an instance of a box pointing to a copied value of x 
    // rather than a reference pointing to the value of x

    mod example {
        struct MyBox<T>(T);

        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }
        /// Without the Deref trait, the compiler can only dereference & references
        /// When we entered *y in Listing 15-9, behind the scenes Rust actually ran this code:  *(y.deref())
        use std::ops::Deref;
        impl<T> Deref for MyBox<T> {
            // defines an associated type for the Deref trait to use
            type Target = T;
            // deref is a func that borrows self and returns a reference to the inner data
            fn deref(&self) -> &Self::Target {
                // If the deref method returned the value directly instead of a reference to the value
                // the value would be moved out of self
                &self.0
            }
        }
    }

    /// Deref coercion is a convenience that Rust performs on arguments to functions and methods
    /// For example, deref coercion can convert &String to &str because String implements the Deref trait such that it returns &str
    /// Deref coercion happens automatically
    mod deref_coercion {
        fn hello(name: &str) {
            println!("Hello, {}!", name);
        }
        fn usage() {
            hello("Rust");
            // Rust can turn &MyBox<String> into &String by calling deref
            let m = crate::MyBox::new(String::from("Rust"));
            // Rust can turn &MyBox<String> into &String by calling deref
            hello(&m);
            // call without deref
            // the & and [..] take a string slice of the String
            let m = MyBoxx:new(String::from("Rust"));
            hello(&(*m)[..]);
        }
        fn mut_deref() {
            // you can use the DerefMut trait to override the * operator on mutable references
            // - From &T to &U when T: Deref<Target=U>
            // - From &mut T to &mut U when T: DerefMut<Target=U>
            // - From &mut T to &U when T: Deref<Target=U>
            // immutable references will never coerce to mutable references
        }
    }
}

/// Specify the deallocate code to run when a value goes out of scope by implementing the Drop trait
mod drop {
    struct CustomSmartPointer {
        data: String,
    }
    
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }
    
    fn main() {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        // d was dropped before c
        println!("CustomSmartPointers created.");
    }

    /// Rust doesn’t let you call the Drop trait’s drop method manually; 
    /// instead you have to call the std::mem::drop function
    fn dropping_early() {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created.");
        // explicit destructor calls not allowed
        // Rust doesn’t let us call drop explicitly because Rust would still 
        // automatically call drop on the value at the end of main.
        // This would be a double free error
        // c.drop();
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }
}

/// there are cases when a single value might have multiple owners
/// A node shouldn’t be cleaned up unless it doesn’t have any edges pointing to it
/// Rc: reference counting
/// keeps track of the number of references to a value to determine whether or not the value is still in use
/// 
/// We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our program to read and 
/// we can’t determine at compile time which part will finish using the data last.
mod Rc_T {
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use crate::Rc_T::List::{Cons, Nil};
    
    fn usage() {
        let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
        // b and c are referce to a concurrently
        let b = Cons(3, Box::new(a));
        // not allowed to because a has been moved.
        let c = Cons(4, Box::new(a));
    }
    /// solve:
    /// 1. We could change the definition of Cons to hold references instead, but then we would have to specify lifetime parameters
    /// 2. change our definition of List to use Rc<T>
    /// 
    /// Using Rc<T> allows a single value to have multiple owners, and the count ensures that the value remains valid as long as any of the owners still exist
    /// 
    /// ### inmutable ref because mutiple mutable ref violate rules
    mod boost_by_Rc {
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }
        
        use super::boost_by_Rc::List::{Cons, Nil};
        use std::rc::Rc;
        
        fn usage() {
            let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
            // we call the Rc::clone function and pass a reference to the Rc<List> in a as an argument
            // in this case. The implementation of Rc::clone doesn’t make a deep copy of all the data like most types’ implementations of clone do
            let b = Cons(3, Rc::clone(&a));
            // Rc::clone() has a butter performance
            let c = Cons(4, a.clone());
            // We can see that the Rc<List> in a has an initial reference count of 1; then each time we call clone, the count goes up by 1
            // call Rc::clone() or x.clone() will increase the count
            // the Drop trait decreases the reference count automatically
            println!("count of a: {}", Rc::strong_count(&a));
        }
    }
}

/// Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data, 
/// the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing
/// 
/// RefCell<T> type represents single ownership over the data it holds
/// 
/// The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that
/// 
/// summary
/// - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
/// - Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
/// - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
mod RefCell {
    // there are situations in which it would be useful for a value to mutate itself in its methods but appear immutable to other code
    // the borrowing rules are checked at runtime instead
    
    // example is 19.5_example.rs

    // use borrow_mut() to get the mutable borrow
    // use borrow() to get the immutable borrow

    // how this works:
    // borrow() method returns the smart pointer type Ref<T>
    // borrow_mut() returns the smart pointer type RefMut<T>

    // The RefCell<T> keeps track of how many Ref<T> and RefMut<T> smart pointers are currently active. Every time we call borrow,
    // the RefCell<T> increases its count of how many immutable borrows are active. When a Ref<T> value goes out of scope,
    // the count of immutable borrows goes down by one. Just like the compile-time borrowing rules,
    // RefCell<T> lets us have many immutable borrows or one mutable borrow at any point in time
}

/// A common way to use RefCell<T> is in combination with Rc<T>.
/// Recall that Rc<T> lets you have multiple owners of some data, but it only gives immutable access to that data.
/// If you have an Rc<T> that holds a RefCell<T>, you can get a value that can have multiple owners and that you can mutate!
mod mutiple_owners {
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use super::mutiple_owners::List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;

    fn usage() {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
}

/// We can see that Rust allows memory leaks by using Rc<T> and RefCell<T>: it’s possible to create references where items refer to each other in a cycle. 
/// This creates memory leaks because the reference count of each item in the cycle will never reach 0, and the values will never be dropped.
mod memory_leaking {
    // example: 19.7_example.rs

    /// create a weak reference to the value within an Rc<T> instance by calling Rc::downgrade and passing a reference to the Rc<T>
    /// he difference is the weak_count doesn’t need to be 0 for the Rc<T> instance to be cleaned up
    /// Weak references don’t express an ownership relationship.
    /// to do anything with the value that a Weak<T> is pointing to, you must make sure the value still exists
    /// Do this by calling the upgrade method on a Weak<T> instance, which will return an Option<Rc<T>>
    fn prevent_ref_cycle() {

    }
}

mod tree_data_structure {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
    }

    fn main() {
        let leaf = Rc::new(Node {
            value: 3,
            children: RefCell::new(vec![]),
        });

        let branch = Rc::new(Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
    }

    /// if we nned to make child aware of its parent
    /// add parent field but using Weak<T> to provent cycle
    mod improve {
        use std::cell::RefCell;
        use std::rc::{Rc, Weak};

        #[derive(Debug)]
        struct Node {
            value: i32,
            parent: RefCell<Weak<Node>>,
            children: RefCell<Vec<Rc<Node>>>,
        }

        fn main() {
            let leaf = Rc::new(Node {
                value: 3,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![]),
            });

            println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        }
    }
}

fn main() {

}