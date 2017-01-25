pub mod animal;
pub mod mammal;
pub mod bird;
pub mod cat;
//pub mod platypus;

#[allow(unused_imports)]
use animal::Animal;
#[allow(unused_imports)]
use mammal::{Mammal,Mammilian};
#[allow(unused_imports)]
use bird::{Bird, Avian};
#[allow(unused_imports)]
use cat::Cat;
#[allow(unused_imports)]
//use platypus::Platypus;





















fn main() {
	println!("");
	println!("---------------");
	println!("");


	// let cat = Animal::default();
  // let cat = Animal::with_sound("meow");
  // let cat = Cat::default();

  // cat.speak();
  // cat.what_is_it();
  // (*cat).speak();


























  let poodle = Mammal::new("bark","fluffy");
  let ostridge = Bird::new("boom!","enourmous");

  poodle.speak();
  println!("A mammal with {} hair", poodle.hair());
  //println!("A mammal with {} eggs", poodle.eggs());

  ostridge.speak();
  //println!("A bird with {} hair", ostridge.hair());
  println!("A bird with {} eggs", ostridge.eggs());

  // let platypus;


























	println!("");
	println!("---------------");
	println!("");
}