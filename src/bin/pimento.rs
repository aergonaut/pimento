extern crate iron;
extern crate logger;
#[macro_use]
extern crate router;

extern crate pimento;

use iron::prelude::*;
use logger::Logger;

fn main() {
    let (logger_before, logger_after) = Logger::new(None);

    let router = router!(
        get "/ping" => pimento::handlers::pong::handler
    );

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:3000").unwrap();
}
