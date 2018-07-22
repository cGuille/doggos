extern crate actix_web;
#[macro_use] extern crate serde_derive;

use actix_web::{server, http, HttpResponse, Responder, App, State, Path, Json};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

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

struct AppState {
    locked_repo: Arc<RwLock<InMemoryDoggoRepository>>,
}

fn register_doggo((state, json_doggo): (State<AppState>, Json<Doggo>)) -> impl Responder {
    let mut repo = state.locked_repo.write().unwrap();

    repo.save(json_doggo.into_inner().to_owned());

    HttpResponse::Ok()
}

fn fetch_doggo((state, path) : (State<AppState>, Path<(String,)>)) -> impl Responder {
    let repo = state.locked_repo.read().unwrap();

    match repo.find(&path.0) {
        None => HttpResponse::NotFound().finish(),
        Some(doggo) => HttpResponse::Ok().body(doggo.name),
    }
}

fn main() {
    let locked_repo = Arc::new(RwLock::new(InMemoryDoggoRepository::new()));

    let server = server::new(move ||
        App::with_state(AppState { locked_repo: Arc::clone(&locked_repo) })
            .resource("/doggos", |r| r.method(http::Method::POST).with(register_doggo))
            .resource("/doggos/{doggo_id}", |r| r.method(http::Method::GET).with(fetch_doggo))
    );

    server.bind("127.0.0.1:8088").unwrap().run();
}
