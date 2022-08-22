mod post_feed_handler;
mod post_post_handler;
mod post_handler;

use std::sync::{Arc, Mutex };
use super::Database;
pub use post_feed_handler::PostFeedHandler;
pub use post_handler::PostHandler;
pub use post_post_handler::PostPostHandler;

pub struct Handlers {
    pub post_feed: PostFeedHandler,
    pub post_post: PostPostHandler,
    pub post: PostHandler,
}

impl Handlers {
    pub fn new(db: Database) -> Handlers {
        let database = Arc::new(Mutex::new(db));

        Handlers {
            post_feed: PostFeedHandler::new(database.clone()),
            post_post: PostPostHandler::new(database.clone()),
            post: PostHandler::new(database.clone()),
        }
    }
}
