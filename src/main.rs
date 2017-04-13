#![feature(associated_consts)]

#[macro_use]
extern crate cargonauts;

mod resources;

mod routes;
use routes::routes;

fn main() {
    let socket_addr = "127.0.0.1:7878".parse().unwrap();
    cargonauts::server::Http::new().bind(&socket_addr, routes).unwrap().run().unwrap();
}
