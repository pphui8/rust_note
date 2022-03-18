/// state pattern
/// the pattern is that a value has some internal state, which is represented by a set of state objects
/// The way we organized the code, we have to look in only one place to know the different ways a published post can behave:
/// the implementation of the State trait on the Published struct.
mod state_patten {
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }
    
    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }
        /// we still want the content method to return an empty string slice because the post is still in the draft state
        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }
        /// change its state from Draft to PendingReview.
        pub fn request_review(&mut self) {
            // we call the take method to take the Some value out of the state field and leave a None in its place,
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }
        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
    }
    
    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
    }
    
    struct Draft {}
    
    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }
    
    /// represents the state when a post is waiting for a review
    struct PendingReview {}
    
    /// when we request a review on a post already in the PendingReview state, it should stay in the PendingReview state.
    impl State for PendingReview {
        /// This syntax means the method is only valid when called on a Box holding the type. 
        /// To consume the old state, the request_review method needs to take ownership of the state value. 
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
    }
    
    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }

    /// example: blog
    /// - A blog post starts as an empty draft.
    /// - When the draft is done, a review of the post is requested.
    /// - When the post is approved, it gets published.
    /// - Only published blog posts return content to print, so unapproved posts can’t accidentally be published.
    fn main() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}

/// implementing the state pattern exactly isn`t taking s full advantage of Rust
/// encoding states and behavior as types
mod improve {
    // instead of having a content method on a draft post that returns an empty string,
    // we’ll make it so draft posts don’t have the content method at all
    pub struct Post {
        content: String,
    }
    
    pub struct DraftPost {
        content: String,
    }
    
    // Any attempt to get around these constraints will result in a compiler error.
    impl Post {
        /// content is private and there aren’t any functions that return Post,
        /// it’s not possible to create an instance of Post right now.
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }
        pub fn content(&self) -> &str {
            &self.content
        }
    }
    
    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }
        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }

    pub struct PendingReviewPost {
        content: String,
    }
    
    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
    }

    /// this implementation doesn’t quite follow the object-oriented state pattern anymore:
    /// the transformations between the states are no longer encapsulated entirely within the Post implementation.
    /// our gain is that invalid states are now impossible because of the type system and the type checking that happens at compile time
    fn main() {
        let mut post = Post::new();
    
        post.add_text("I ate a salad for lunch today");
    
        let post = post.request_review();
    
        let post = post.approve();
    
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}

fn main() {

}