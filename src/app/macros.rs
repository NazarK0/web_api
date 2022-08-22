macro_rules! try_handler {
    ($e: expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Ok(iron::Response::with((iron::status::InternalServerError, e.to_string())))
        }
    };
    ($e: expr, $error: expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Ok(iron::Response::with(($error, e.to_string())))
        }
    };
}

macro_rules! lock {
    ($e: expr) => { $e.lock().unwrap() }
}

macro_rules! get_http_param {
    ($r: expr, $e: expr) => {
        match $r.extensions.get::<router::Router>() {
            Some(router) => {
                match router.find($e) {
                    Some(v) => v,
                    None => return Ok(iron::Response::with(iron::status::BadRequest))
                }
            },
            None => return Ok(iron::Response::with(iron::status::InternalServerError))
        }
    }
}
