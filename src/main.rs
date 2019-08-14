#![feature(proc_macro_hygiene, decl_macro)]

mod repl;
mod web;

use std::sync::RwLock;

use calculator::calc::Calculator;
use rocket::{get, routes};
use rocket::http::Method;
use rocket_contrib::templates::Template;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};

use crate::repl::repl;
use crate::web::*;

fn main() {
  let allowed_origins = AllowedOrigins::all();

  // You can also deserialize this
  let cors = rocket_cors::CorsOptions {
      allowed_origins,
      allowed_methods: vec![Method::Get, Method::Post, Method::Options].into_iter().map(From::from).collect(),
      allowed_headers: AllowedHeaders::all(),
      allow_credentials: true,
      ..Default::default()
  }
  .to_cors().unwrap();

  rocket::ignite()
    .manage(RwLock::new(Calculator::new()))
    .mount("/", routes![get_index, calculate])
    .attach(Template::fairing())
    .attach(cors)
    .launch();
}
