extern crate actix_web;
#[macro_use] extern crate serde_derive;

use actix_web::{server, http, App, Path, Json};
use std::collections::HashMap;

#[derive(Clone,Deserialize)]
struct Doggo {
    id: String,
    name: String,
}

struct InMemoryDoggoRepository {
    map: HashMap<String, Doggo>,
}

impl InMemoryDoggoRepository {
    pub fn new() -> InMemoryDoggoRepository {
        InMemoryDoggoRepository {
            map: HashMap::new(),
        }
    }

    fn save(&mut self, doggo: Doggo) {
        self.map.insert(doggo.id.to_owned(), doggo.to_owned());
    }

    fn find(&self, doggo_id: &String) -> Option<Doggo> {
        match self.map.get(doggo_id) {
            None => None,
            Some(doggo) => Some(doggo.to_owned()),
        }
    }
}

fn register_doggo(doggo: Json<Doggo>) -> String {
    format!("Welcome {}! Good boy.", doggo.name)
}

fn fetch_doggo(path: Path<(String,)>) -> String {
    format!("Hello again, good boy {}!", path.0)
}

fn main() {
    let server = server::new(||
        App::new()
            .resource("/doggos", |r| r.method(http::Method::POST).with(register_doggo))
            .resource("/doggos/{doggo_id}", |r| r.method(http::Method::GET).with(fetch_doggo))
    );

    server.bind("127.0.0.1:8088").unwrap().run();
}
