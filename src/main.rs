extern crate hyper;
extern crate postgres;
extern crate quick_protobuf;
extern crate rand;

use crate::hyper::{Response, Server, Body};
use crate::hyper::rt::{Future, run};
use crate::hyper::service::service_fn_ok;
use crate::rand::rngs::ThreadRng;
use crate::rand::thread_rng;
use ::std::net::SocketAddr;
use rust_channel_cache::Channels;
use hyper::Request;

pub fn main() {
    let addr: SocketAddr = ([0u8, 0u8, 0u8, 0u8], 8082u16).into();

    let store: Channels = Channels::init();

    let new_service = move || {
        let store: &Channels = &store;
        let store: Channels = store.clone();

        service_fn_ok(move |req: Request<Body>| {
            println!("Got request - sending 50 channels");

            let num: String = format!("{}", req.uri().path());
            let num = num.split("/").last().unwrap();
            let num = num.parse::<usize>();

            let mut length: usize = store.len();
            if num.is_ok() {
                let num = num.unwrap();
                if num >= 50 && num <= store.len() {
                    length = num;
                }

                println!("Getting items from top {} rows", num);
            }

            let mut rng: ThreadRng = thread_rng();
            let store: &Channels = &store;
            let store: Channels = store.clone();

            let vec: &Vec<u8> = &store.get_msg(&mut rng, length);
            let body: Body = Body::from(vec.clone());

            Response::new(body)
        })
    };

    let f = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    run(f);
}
