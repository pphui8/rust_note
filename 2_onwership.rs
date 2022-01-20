#[allow(unused, dead_code)]
fn main() {
    #[allow(dead_code)]
    fn take_onwership(str: String) {
        println!("{}", str);
    }

    fn not_take_onwership(str: &String) {
        println!("{}", *str);
    }

    {
        let s = "hello world!";
        // call s.drop() automaticlly

        // success because integer has known and fixed size
        // variable with known size stroed in stack
        let x = 5;
        let y = x;
        println!("{} {}", x, y);

        // false because string is a pointer
        let s1 = String::from("hello");
        //s1 was moved here
        // let s2 = s1;
        // take_onwership(s1);
        // s1 wasn`t moved here
        not_take_onwership(&s1);
        let s2 = s1.clone();
        println!("{}, world!", s1);
    }
}

fn borrow() {
    let mut a = String::from("hello");
    // We also cannot have a mutable 
    // reference while we have an immutable one
    // let c = &a;

    let b = &mut a;

    // can`t use c as mutable borrow unless c nenver used
    // let c = &mut a;
    // println!("{} ", c);
}

// slice is a kind of reference
fn slice_type() {
    let a = String::from("hello world!");
    // equal
    let b1 = &a[0..12];
    let b2 = &a[..];
    assert_eq!(b1, b2);

    println!("{}", first_word(&a));

    // use &str rather than &String in first_word()
    // &str accept both String and str
    let c = "abc";
    first_word(&a);
    first_word(&c);
}

// use slice as return value
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}