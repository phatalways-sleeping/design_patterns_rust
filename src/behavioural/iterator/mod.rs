use std::slice::Iter;

pub struct Feed {
    pub id: String,
    pub content: String,
    pub created_at: String,
}

pub trait Iterable {
    fn feed_iter(&self) -> Iter<Feed>;
    fn images_iter(&self) -> Iter<String>;
}

pub struct Account {
    feeds: Vec<Feed>,
    images: Vec<String>,
}

impl Iterable for Account {
    fn feed_iter(&self) -> Iter<Feed> {
        self.feeds.iter()
    }

    fn images_iter(&self) -> Iter<String> {
        self.images.iter()
    }
}
