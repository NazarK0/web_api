use std::sync::{Arc, Mutex};
use std::io::Read;
use iron::{Handler, IronResult, Request, Response, status};
use crate::Database;

pub struct PostPostHandler {
    database: Arc<Mutex<Database>>,
}

impl PostPostHandler {
    pub fn new(database: Arc<Mutex<Database>>) -> PostPostHandler {
        PostPostHandler { database }
    }
}

impl Handler for PostPostHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        try_handler!(req.body.read_to_string(&mut payload));

        let post = try_handler!(serde_json::from_str(&payload), status::BadRequest);

        lock!(self.database).add_post(post);
        Ok(Response::with((status::Created, payload)))
    }
}
