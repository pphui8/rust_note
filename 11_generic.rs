#[allow(unused)]
fn in_struct() {
    struct Point<T> {
        x: T,
        y: T,
    }

    // wrong when x, y have different types
    let p = Point { x: 5, y: 10 };

    struct Pointt<T, U> {
        x: T,
        y: U,
    }

    let p = Pointt  { x: 5, y: 10.01 };
}

#[allow(unused)]
fn in_enum() {
    enum Option<T> {
        Some(T),
        None,
    }
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}

#[allow(unused)]
fn in_method() {
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    // implement methods only on Point<f32>
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    let p = Point { x: 3, y: 4 };
    println!("{}", p.x());
    // no method fond for p
    // println!("{}", p.distance_from_origin());
    let p = Point { x: 2.0f32, y: 4.0f32 };
    println!("{}", p.distance_from_origin());
}

#[allow(unused)]
// not to use syntaic sugar because T also is a return value
// T needs to implement Copy to get the first value
// so noly data implemented Copy could use this function (i32, char...)
fn in_function<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
    /*
        we could specify that T has the trait bound Clone instead of Copy
        but that will potentially use heap
        heap allocations can be slow if we’re working with large amounts of data

        Another way we could implement largest is for 
        the function to return a reference to a T value 
        in the slice.
    */
}

fn main() {

}