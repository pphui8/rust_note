// lifetime main aim to prevent dangling refernce

fn main() {
    lifetime_test();
}

#[allow(unused)]
fn the_problem() {
    {
        let r;
        {
            let x = 5;
            r = &x;
        }   // wrong because x ended here
        // println!("r: {}", r);
    }
}

// a function return the longer of string slices
// wrong because don`t know which slice is returned
// we don`t know the refrence`s lifetime since they are borrowed
// so we can`t assume thoese value always benn valid
// add generic lifetime parameters that define the relationship between 
// the references so the borrow checker can perform its analysis
// fn longest(x: &str, y: &str) -> &str
#[allow(unused)]
// add (') let make the function accept reference with any lifetime
// the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 'a get the lifetime equals to the small one of x and y
    if x.len() > y.len() {
        x
    } else {
        y
    }
/*
    The constraint we want to express in this signature is that the lifetimes of 
    both of the parameters and the lifetime of the returned reference are 
    related such that the returned reference will be valid as long as both the parameters are.
*/
}

#[allow(unused)]
fn lifetime_test() {
    let string1 = String::from("the longer string (very long)");
    let result;
    {
        let string2 = String::from("the shorter string");
        result = longest(string1.as_str(), string2.as_str());
        println!("the longer one is {}", result);
    }
    // wrong because result indicate a value witch doesn`t live long enough
    // even the return value refers to string1
    // result`s lifetime equals to the shorter one: string2
    // println!("the longer one is {}", result);
}

// struct holds references
#[allow(unused)]
struct ImportantExcerpt<'a> {
    // This annotation means an instance of ImportantExcerpt can’t 
    // outlive the reference it holds in its part field.
    part: &'a str,
}

// implement holds references
#[allow(unused)]
impl<'a> ImportantExcerpt<'a> {
    // the lifetime elision rules often make it so 
    // that lifetime annotations aren’t necessary in method signatures
    /*
        There are two input lifetimes, so Rust applies the first lifetime elision rule 
        and gives both &self and announcement their own lifetimes. 
        Then, because one of the parameters is &self, the return type gets the lifetime of &self,
        and all lifetimes have been accounted for.
    */
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

#[allow(unused)]
fn use_struct() {
    let s = String::from("hello world!");
    let instance = ImportantExcerpt {
        part: s.as_str(),
    };
}

// lifetime elision rules:  if your code fits these particular cases, you don’t need to write the lifetimes explicitly.
// fn first_word<'a>(s: &'a str) -> &'a str was required in ealry vsion
#[allow(unused)]
fn first_word(s: &str) -> &str {
    // compiled without error because return value`s lifetime is deterministic
    // error if there`s ambiguity
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
    /*
        three rules:
            1. each parameter that is a reference gets its own lifetime parameter
            2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
            3. multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime 
            of self is assigned to all output lifetime parameters
    */
}

#[allow(unused)]
fn static_lifetime() {
    // static means that this reference can live for the entire duration of the program
    let s: &'static str = "static lifetime";

}

use std::fmt::Display;
/*
    Because lifetimes are a type of generic, the declarations of the lifetime parameter 'a 
    and the generic type parameter T go in the same list inside the angle brackets after 
    the function name
*/
#[allow(unused)]
fn generic_type_and_trait() {
    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}