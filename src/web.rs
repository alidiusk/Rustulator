use std::io;
use std::sync::RwLock;

use rocket::{get, post, State};
use rocket::response::NamedFile;
use rocket_contrib::json::Json;
use serde_derive::{Serialize, Deserialize};

use calculator::calc::Calculator;

#[derive(Serialize, Deserialize)]
pub struct Calculation {
  calc: String,
}

#[get("/")]
pub fn get_index() -> io::Result<NamedFile> {
  NamedFile::open("static/index.html")
}

#[post("/", format = "application/json", data = "<calculation>")]
pub fn calculate(calculator: State<RwLock<Calculator>>, calculation: Json<Calculation>) -> Json<String> {
  let input = &calculation.0.calc;
  let mut calc = calculator.write().unwrap();
  let output = match calc.calculate(input) {
    Ok(n) => format!("{}", n),
    Err(e)  => format!("{}", e),
  };
  Json(output)
}


