// In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up
//     let v1 = vec![1, 2, 3];
//     let v1_iter = v1.iter(); // useless
// All iterators implement a trait named Iterator that is defined in the standard library


trait Iterator {
    //  the Item type will be the type returned from the iterator
    type Item;
    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

fn iterator_basic() {
    let v1 = vec![1, 2, 3];

    // imutable references
    let mut v1_iter = v1.iter();
    // mutable references
    // let mut v1_iter_mut = v1.iter_mut();
    // takes the onwership
    // let mut v1_iter_taken = v1.into_iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}


fn iterator_consume() {
    // Methods that call next are called consuming adaptors(消费适配器)
    let mut v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    let total: i32 = v1_iter.sum(); // take onwership
    // We aren’t allowed to use v1_iter after the call to sum
    // because sum takes ownership of the iterator we call it on.

    // iterator adaptors allow you to change iterators into different kinds of iterators
    // all iterators are lazy, you have to call one of the consuming adaptor methods to get results from calls to iterator adaptors
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    // collet consumes the iterator and collects the resulting values into a collection data type
}

/*
 If the closure returns true, the value will be included in the iterator produced by filter. 
If the closure returns false, the value won’t be included in the resulting iterator
*/
fn closure_cap_env() {
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }
    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        // shoe_size comes from envoriment
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);
    }
}

// 1. implement trait iterator
// 2. provide a definition for is the next method (at least)
fn create_iterator() {
    struct Counter {
        // The count field is private because we want the implementation of Counter to manage its value
        count: u32,
    }
    
    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        // meaning the iterator will return u32 values
        type Item = u32;
    
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    
    fn other_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}

fn main() {

}