#![feature(proc_macro_hygiene, decl_macro)]

mod repl;
mod web;

use std::sync::RwLock;

use calculator::calc::Calculator;
use rocket::{routes};
use rocket_contrib::templates::Template;

use crate::repl::repl;
use crate::web::*;

fn main() {
  rocket::ignite()
    .manage(RwLock::new(Calculator::new()))
    .mount("/", routes![get_index, post_index])
    .attach(Template::fairing())
    .launch();
}
