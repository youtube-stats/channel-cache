extern crate hyper;
extern crate postgres;
extern crate quick_protobuf;
extern crate rand;

use crate::hyper::{Response, Server, Body};
use crate::hyper::rt::{Future, run};
use crate::hyper::service::service_fn_ok;
use ::std::net::SocketAddr;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use rust_channel_cache::Channels;

pub fn main() {
    let addr: SocketAddr = ([0u8, 0u8, 0u8, 0u8], 8082u16).into();

    let store: Channels = Channels::init();

    let new_service = move || {
        let store: &Channels = &store;
        let store: Channels = store.clone();

        service_fn_ok(move |_| {
            let mut rng: ThreadRng = thread_rng();
            let store: &Channels = &store;
            let store: Channels = store.clone();

            let vec: &Vec<u8> = &store.get_msg(&mut rng);
            let body: Body = Body::from(vec.clone());

            Response::new(body)
        })
    };

    let f = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    run(f);
}
