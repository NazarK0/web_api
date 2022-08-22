use std::sync::{Arc, Mutex};
use iron::{Handler, IronResult, Request, Response, status};
use crate::Database;

pub struct PostFeedHandler {
    database: Arc<Mutex<Database>>,
}

impl PostFeedHandler {
    pub fn new(database: Arc<Mutex<Database>>) -> PostFeedHandler {
        PostFeedHandler { database }
    }
}

impl Handler for PostFeedHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let payload = try_handler!(serde_json::to_string(lock!(self.database).posts()));

        Ok(Response::with((status::Ok, payload)))
    }
}
