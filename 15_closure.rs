// closures is anonymous functions that can capture their environment
// Closures don’t require you to annotate the types of the parameters or the return value like fn functions do
// Closure definitions will have one concrete type inferred for each of their parameters and for their return value
fn closures() {
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u8|             { x + 1 };
    let add_one_v4 = |x: u8|               x + 1  ;
} 

// to aviod unnessasery call and call this repeatly
// We can create a struct that will hold the closure and the resulting value of calling the closure
// this called  memoization or lazy evaluation.
fn closure_with_stored_value() {
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }
    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }
    // usage
    let mut expensive_res = Cacher::new(|num| {
        println!("calculating...");
        num
    });
    // call when nessesary and recall doesn`t matter
    let res = expensive_res.value(32);
}
/*
problem one: recall this instance by expensive_res.value(32); in different paramiters always get the same value
problem two: current Cacher implementation is that it only accepts closures that take one parameter of type u32 and return a u32
*/ 


// closure can also capture their environment and access variables from the scope in which they’re defined
// witch function couldn`t
fn closure_capture_env() {
    let x = 4;
    // borrow x so closure implement Fn trait because the body of the closure only needs to read the value in x
    let equal_to_x = |z| z == x;
    // wrong!!
    // fn equal_to_x(z: i32) -> bool {
    //     z == x
    // }
    let y = 4;
    assert!(equal_to_x(y));
}
/*
When a closure captures a value from its environment, it uses memory to store the values for use in the closure body
Closures can capture values from their environment in three ways
1. FnOnce: taking onwership (the closure can’t take ownership of the same variables more than once)
2. FnMut: can change the environment
3. Fn

All closures implement FnOnce because they can all be called at least once. Closures that don’t move the captured variables also implement FnMut
and closures that don’t need mutable access to the captured variables also implement Fn
If you want to force the closure to take ownership of the values it uses in the environment, you can use the move keyword before the parameter list
let equal_to_x = move |z| z == x;   // take onwership
*/


fn main() {

}