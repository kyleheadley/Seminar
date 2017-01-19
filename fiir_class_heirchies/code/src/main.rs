pub mod animal;
pub mod cat;

#[allow(unused_imports)]
use cat::Cat;
#[allow(unused_imports)]
use animal::Animal;

fn main() {
	println!("");
	println!("---------------");
	println!("");


	// let cat = Animal::default();
  // let cat = Animal::with_sound("meow");
  let cat = Cat::default();

  cat.speak();
  cat.what();


	println!("");
	println!("---------------");
	println!("");
}