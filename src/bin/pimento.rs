extern crate iron;
#[macro_use]
extern crate router;

extern crate pimento;

use iron::prelude::*;

fn main() {
    let router = router!(
        get "/ping" => pimento::handlers::pong::handler
    );

    Iron::new(router).http("localhost:3000").unwrap();
}
