use std::sync::RwLock;
use std::collections::HashMap;

use rocket::{get, post, State};
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use serde_derive::{Serialize, Deserialize};

use calculator::calc::Calculator;

#[derive(Serialize, Deserialize)]
pub struct Calculation {
  calc: String,
}

#[get("/")]
pub fn get_index() -> Template {
  let context: HashMap<String, String> = HashMap::new();
  Template::render("index", &context)
}

#[post("/calculate", format = "application/json", data = "<calculation>")]
pub fn calculate(calculator: State<RwLock<Calculator>>, calculation: Json<Calculation>) -> Json<String> {
  let input = &calculation.0.calc;
  let mut calc = calculator.write().unwrap();
  let output = match calc.calculate(input) {
    Ok(n) => format!("{}", n),
    Err(e)  => format!("{}", e),
  };
  Json(output)
}


