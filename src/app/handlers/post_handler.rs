use std::sync::{Arc, Mutex};
use iron::{Handler, IronResult, Request, Response, status};
use uuid::Uuid;
use crate::Database;
use crate::app::models::Post;

pub struct PostHandler {
    database: Arc<Mutex<Database>>,
}

impl PostHandler {
    pub fn new(database: Arc<Mutex<Database>>) -> PostHandler {
        PostHandler { database }
    }

    pub fn find_post(&self, id: &Uuid) -> Option<Post> {
        let locked = lock!(self.database);
        let mut iterator = locked.posts().iter();
        iterator.find(|p| p.uuid() == id).map(|p| p.clone())
    }
}

impl Handler for PostHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref post_id = get_http_param!(req, "id");

        let id = try_handler!(Uuid::parse_str(post_id), status::BadRequest);

        if let Some(post) = self.find_post(&id) {
            let payload = try_handler!(serde_json::to_string(&post), status::InternalServerError);

            Ok(Response::with((status::Ok, payload)))
        } else {
            Ok(Response::with(status::NotFound))
        }
    }
}
