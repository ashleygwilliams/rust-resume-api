use cargonauts;
use cargonauts::api::{Resource, Get, Index, Environment, Error};
use cargonauts::futures::{Future, BoxFuture, future};
use cargonauts::futures::stream::{self, Stream, BoxStream};

#[derive(Debug)]
pub struct Person { 
    name: String,
}

impl Resource for Person {
    type Identifier = String;
    fn identifier(&self) -> String { self.name.clone() }
}

impl Get for Person {
    fn get(name: String, _: Environment) -> BoxFuture<Person, Error> {
        future::ok(Person { name }).boxed()
    }
}

impl Index for Person {
    fn index(_: Environment) -> BoxStream<Person, Error> {
        stream::once(Ok(Person { name: String::from("ag_dubs") })).boxed()
    }
}
