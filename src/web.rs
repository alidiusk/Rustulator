use std::sync::RwLock;
use std::collections::HashMap;

use rocket::{FromForm, get, post, State};
use rocket::request::Form;
use rocket_contrib::templates::Template;

use calculator::calc::Calculator;

#[derive(FromForm)]
pub struct Calculation {
  calc: String,
}

#[get("/")]
pub fn get_index() -> Template {
  let context: HashMap<String, String> = HashMap::new();
  Template::render("index", &context)
}

#[post("/", data = "<calculation>")]
pub fn post_index(calculator: State<RwLock<Calculator>>, calculation: Form<Calculation>) -> Template {
  let input = &calculation.calc;
  let mut calc = calculator.write().unwrap();
  let output = match calc.calculate(input) {
    Ok(n) => format!("{}", n),
    Err(e)  => format!("{}", e),
  };
  let mut context = HashMap::new();
  context.insert("output".to_string(), output);
  Template::render("output", &context)
}
