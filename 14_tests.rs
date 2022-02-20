// put unit tests om src directory and annotate the module with cfg(test)
// integration tests are entirely external to your library, To create integration tests, you first need a tests diectory
// add a file ended with .rs, write tests and $ cargo test --test filename.rs or just cargo test

// Rust’s privacy rules do allow you to test private functions

// Each test is run in a new thread
// to avoid problems coused by prallel, use: $ cargo test -- --test-threads=1
// tests won`t print output, use: $ cargo test -- --show-output


// assert() to test wether value is true
// assert_eq() to test wether two values is equal
// assert_ne() to test wether two values is not equal


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // Tests fail when something in the test function panics
    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    #[ignore]   //this will be ignored most tests, use -- --ignored
    fn it_works_2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

// It's an attribute that provides a basic implementation of the Debug trait for the following struct.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests_2 {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    #[should_panic(expected = "specified the entire panic message")]
    fn smaller_cannot_hold_lager() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(smaller.can_hold(&larger));
    }
}