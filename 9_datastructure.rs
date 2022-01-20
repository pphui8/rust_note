#[allow(unused, dead_code)]

fn main() {
    
}

fn vector() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    let v2 = vec![1, 2, 3, 4, 5];

    // third os a point
    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    if let Some(i) = v2.get(2) {
        println!("The third element is {}", i);
    }

    {
        let mut v = vec![1, 2, 3, 4, 5];
    
        let first = &v[0];
    
        // cannot borrow `v` as mutable because it is 
        // also borrowed as immutable
        // v.push(6);
    
        println!("The first element is: {:?}", first);
    }
    
    {
        // iterrator
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
    }

    {
        // use enum and vector store multiple types
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
    
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }

    {
        let mut v: Vec<i32> = Vec::new();
        v.push(122);
    }
}

fn string() {
    // initiallize a string, UTF-8 encoded
    let s = String::from("hello world!");
    let mut s = "hello world".to_string();


    let mut s1 = String::from("foo");
    let s2 = "bar";
    // push_str() doesnt take the ownership
    // push() only push a single character
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s1 was borrowed by s3
    // &string was changed into &str automatically
    let s3 = s1 + &s2; 
    println!("{}", /*s1,*/ s2);

    // indexing not allowed because
    // not safe and protencial bugs
    // let str1  = s3[0];

    // slicing
    // contains the first 4 bytes of the string
    let s4 = &s3[0..4];
    let hello = "Здравствуйте";
    // panics at runtime because
    // З contains 2 bytes so [0..1] indecates nothing
    let s = &hello[0..1];

    // interat a string
    for char in hello.chars() {
        // format!() doesn`t take ownership
        format!("{}", char);
    }

    for char in hello.bytes() {
        format!("{}", char);
    }
}

use std::collections::HashMap;

fn hash_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // using iterators and the collect method on a vector of tuples
    // zip method to create an iterator of tuples 
    // collect method to turn that iterator of tuples into a hash map
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // HashMap<_, _> is needed here because it’s possible to collect 
    // into many different data structures 
    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // ownership: owned value(string) will be moved
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // We aren’t able to use the variables field_name and field_value 
    // after they’ve been moved into the hash map
    /*
    If we insert references to values into the hash map, the values 
    won’t be moved into the hash map. The values that the references 
    point to must be valid for at least as long as the hash map is 
    valid.
    */

    // accessing value
    if let Some(value) = scores.get(&String::from("Blue")) {
        println!("{}", value)
    }
    for (key, value) in scores {
        println!("{} {}", key, value)
    }

    // updating
    // overwriting
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    // insert a key without value
    /*
    The return value of the entry method is an enum called 
    Entry that represents a value that might or might not exist
    
    we want to check whether the key for the Yellow team has
    a value associated with it. If it doesn’t,
    we want to insert the value 50

    The or_insert method on Entry is defined to return   
    a mutable reference to the value for the corresponding
    Entry key if that key exists
    */
    scores.entry(String::from("Yellow")).or_insert(50);

    // update a value based on the old vlaue
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // return a mutable reference of the vlaue
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    // {"world": 2, "hello": 1, "wonderful": 1}
}