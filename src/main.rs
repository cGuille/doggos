extern crate actix_web;

#[macro_use] extern crate serde_derive;
use actix_web::{server, http, App, Path, Json};

#[derive(Deserialize)]
struct Doggo {
    id: String,
    name: String,
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
