//! what`s more about cargo
//! - Customize your build through release profiles
//! - Publish libraries on crates.io
//! - Organize large projects with workspaces
//! - Install binaries from crates.io
//! - Extend Cargo using custom commands


/// Cargo has two main profiles: the dev profile Cargo uses when you run cargo build
/// and the release profile Cargo uses when you run cargo build --release
/// set in Cargo.toml, default is 0
/// ```toml
/// [profile.dev]
/// opt-level = 0
/// [profile.release]
/// opt-level = 3
/// ```
fn build_release() {
    // run cargo build
    // run cargo build --release
}

/// Documentation comments use three slashes, ///  
/// support Markdown notation for formatting the text  
/// 
/// # Examples
/// 
/// ```
/// let a = 6;
/// let res = comment(a);
/// 
/// assert_eq(6, res);
/// ```
fn comment(arg: i32) -> i32 {
    arg
}
// We can generate the HTML documentation from this documentation comment by running cargo doc
// This command runs the rustdoc tool distributed with Rust and puts the generated HTML documentation in the target/doc directory

/// more sections of doc
/// # Examples
/// ```
/// //code
/// ```
/// 
/// # Panics
/// don`t call this function in thiese situations
/// 
/// # Error
/// if the function returns a ```Result```, describe the kinds of errors that might occur
/// 
/// # Safety
/// if the function is ```unsafe``` to call
fn documentation() {
}
// //! 
// describe the whole create
// adds documentation to the item that contains the comments rather than adding documentation to the items following the comments. 
// We typically use these doc comments inside the crate root file (src/lib.rs by convention) or inside a module to document the crate or the module as a whole


/// Re-exporting takes a public item in one location and makes it public in another location
/// rather than annoyed deeply to find out useful API
pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        SecondaryColor::Green
    }
}

mod test_1 {
    // had to figure out that PrimaryColor is in the kinds module and mix is in the utils module
    //  the structure is inconvenient because developers must specify the module names in the use statements
    use super::kinds::PrimaryColor;
    use super::utils::mix;

    fn main() {
        let red = PrimaryColor::Red;
        let yellow = PrimaryColor::Yellow;
        mix(red, yellow);
    }
}

/// In cases where there are many nested modules, re-exporting the types at the top level 
/// with pub use can make a significant difference in the experience of people who use the crate
mod improve {
    // re-export useful API
    pub use self::kinds::PrimaryColor;
    pub use self::kinds::SecondaryColor;
    pub use self::utils::mix;

    pub mod kinds {
        // --snip--
    }

    pub mod utils {
        // --snip--
    }
}

mod test_2 {
    use super::improve::mix;
    use super::improve::PrimaryColor;

    fn main() {
        // --snip--
        let red = PrimaryColor::Red;
        let yellow = PrimaryColor::Yellow;
        mix(red, yellow);
    }
}

/// Be careful when publishing a crate because a publish is permanent. The version can never be overwritten
/// and the code cannot be deleted
fn publish_create() {
    // 1. make a useful documentation comment
    // 2. exporting a convenient public API with ```pub use```
    // 3. setting up a creates.io acount
    // 4. add the private token by ```cargo login [token]```
    // 5. add some metadata in Cargo.toml
    // ```toml
    // [package]
    // name = "pphui8_guessing_game"
    // version = "0.1.0"
    // edition = "2021"
    // description = "A fun game where you guess what number the computer has chosen."
    // license = "MIT OR Apache-2.0"
    //
    // [dependencies]
    // ```
    // 6. cargo publish

    // update
    // 1. change the version
    // 2. cargo publish

    // yank: version revocation
    // A yank does not delete any code
    // Yanking a version prevents new projects from starting to depend on that version
    // while allowing all existing projects that depend on it to continue to download and depend on that version
    // yank: ```cargo yank --vers 1.0.1```
    // or undo: ```cargo yank --vers 1.0.1 --undo```
}

fn main() {
    build_release();
    comment(6);
}