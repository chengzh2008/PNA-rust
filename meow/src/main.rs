#[macro_use]
extern crate clap;
#[macro_use]
extern crate dotenv_codegen;

use clap::App;
use std::env;

enum MyErr {
  Reason1,
  Reazon2,
}

fn foo() -> Result<(), MyErr> {
  match Some(1) {
    Some(_) => Ok(()),
    None => Err(MyErr::Reason1),
  }
}

fn main() {
  let yml = load_yaml!("../cli.yml");
  let m = App::from_yaml(yml).get_matches();

  match m.occurrences_of("v") {
    0 => println!("No verbose info"),
    1 => println!("Some verbose info"),
    2 => println!("Tons of verbose info"),
    _ => println!("Don't be crazy"),
  }

  println!("{}", dotenv!("PORT"));

  let key = "HOME";
  match env::var_os(key) {
    Some(val) => println!("{}: {:?}", key, val),
    None => println!("{} is not defined", key),
  }

  match foo() {
    Ok(()) => println!(
      "
correct!"
    ),
    MyErr => println!("error"),
  }

  let path: &str = env!("PATH");
  println!(" the $PATH variable at compile time was: {}", path)
}
