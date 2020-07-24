// State pattern: a value has some internal state, represented by a set of state
// objects. Value's behaviour changes based on the internal state.
// Each state object is responsible for its own behaviour and when it should change
// into another state. The value holding a state object knows nothing about the
// states and their transitions.

// 1. A blog post start as an empty draft.
// 2. Then the draft is done, a review is requested.
// 3. When the post is approved, it gets published.
// 4. Only published posts return content to print.
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text)
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
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

// Behaviour shared by different post states
trait State {
    // Box<Self> means this method is valid only when called on a Box holding the type
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

struct PendingReview {}

impl State for PendingReview {
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

pub fn run() {
    // New draft
    let mut post = Post::new();

    // Text is added to the draft
    post.add_text("I wrote a Rust library today");
    // Not approved yet
    assert_eq!("", post.content());

    // We request for a review
    post.request_review();
    // Not approved yet
    assert_eq!("", post.content());

    // Post gets approved
    post.approve();
    assert_eq!("I wrote a Rust library today", post.content());
}
