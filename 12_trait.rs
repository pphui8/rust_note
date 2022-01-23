pub mod aggregator {
    use std::fmt::{Display, Debug};

    pub trait Summary {
        // fn summarize(&self) -> String;
        // give a default performence
        fn summarize(&self) -> String {
            format!("Read more from {} ....", self.summarize_auther())
            // Default implementations can call other 
            // methods in the same trait, even if those other methods don’t have a default implementation
        }
        fn summarize_auther(&self) -> String;
    }
    
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }

        fn summarize_auther(&self) -> String {
            format!("@{}", self.author)
        }
    }
    
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
   
        fn summarize_auther(&self) -> String {
            format!("@{}", self.username)
        }
    }

    /*
        pub fn notify<T: Summary>(item: &T) {
            println!("Breaking news! {}", item.summarize());
        }
    */
    // syntax sugar version
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    /*
        // item must implement both Summary and Display
        pub fn display_notify<T: Summary + Display>(item: &T) {
            println!("Breaking news! {}", item.summarize())
        }
    */
    // syntax sugar version
    pub fn display_notify(item: &(impl Summary + Display)) {
        println!("Breaking news! {}", item.summarize())
    }

    // 'where' to avoid too many trait
    // instead of this
    /*
        fn where_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    */
    #[allow(unused)]
    fn where_function<T, U>(t: T, u: U) -> u32
        where T: Display + Summary,
              U: Clone + Summary,
    {
        println!("{} {}", t.summarize(), u.summarize());
        0
    }

    // traits as return type
    #[allow(unused)]
    fn return_trait(choice: i8) -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}

use aggregator::{Summary, Tweet, NewsArticle};
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    aggregator::notify(&article);
}