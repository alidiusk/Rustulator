#![feature(proc_macro_hygiene, decl_macro)]

mod repl;
mod web;

use std::sync::RwLock;

use calculator::calc::Calculator;
use clap::{App, SubCommand};
use rocket::{routes};
use rocket::http::Method;
use rocket_contrib::templates::Template;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

use crate::repl::repl;
use crate::web::*;

fn run_rocket() {
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

fn run_repl() {
  repl();
}

fn main() {
  let matches = App::new("Rustulator")
                  .version("0.9")
                  .author("Liam Woodward <liamowoodward@gmail.com>")
                  .about("Calculator REPL/Web interface")
                  .subcommand(SubCommand::with_name("repl")
                    .about("Starts the Rustulator repl"))
                  .subcommand(SubCommand::with_name("web")
                    .about("Starts the Rustulator web interface"))
                  .get_matches();

  if let Some(_) = matches.subcommand_matches("repl") {
    run_repl();
  }
  if let Some(_) = matches.subcommand_matches("web") {
    run_rocket();
  }
}
