pub mod animal;
pub mod mammal;
pub mod bird;
pub mod cat;
pub mod platypus;

#[allow(unused_imports)]
use animal::Animal;
#[allow(unused_imports)]
use mammal::{Mammal,Mammilian};
#[allow(unused_imports)]
use bird::{Bird, Avian};
#[allow(unused_imports)]
use cat::Cat;
#[allow(unused_imports)]
use platypus::{Platypus, Monotreme};





















fn main() {
	println!("");
	println!("---------------");
	println!("");


	let cat = Animal::default();
  // let cat = Animal::with_sound("meow");
  // let cat = Cat::default();

  cat.speak();
  cat.what_is_it();


























	//  let poodle = Mammal::new("bark","fluffy");
	//  let ostridge = Bird::new("boom!","enourmous");

	//  poodle.speak();
	//  println!("A mammal with {} hair", poodle.hair());
	//  // println!("A mammal with {} eggs", poodle.eggs());

	//  ostridge.speak();
	//  // println!("A bird with {} hair", ostridge.hair());
	//  println!("A bird with {} eggs", ostridge.eggs());

	// // let platypus = Platypus::new("growl","wet","tiny");
	// // platypus.what_is_it();






























  // let platypus = Platypus::new("growl","wet","tiny");
  // let poodle = Mammal::new("bark","fluffy");
  // let ostridge = Bird::new("boom!","enourmous");

  // fn mammal_surprize<M: Mammilian>(mammal: &M) {
  // 	println!("The fur is {}!", mammal.hair());
  // }
  // fn avian_surprize<A: Avian>(bird: &A) {
  // 	println!("The eggs are {}!", bird.eggs());
  // }
  // fn monotreme_surprize<M: Monotreme>(_weird: &M) {
  // 	println!("It's the craziest thing!");
  // }

  // mammal_surprize(&poodle);
  // // mammal_surprize(&ostridge);
  // mammal_surprize(&platypus);
  // // avian_surprize(&poodle);
  // avian_surprize(&ostridge);
  // avian_surprize(&platypus);
  // // monotreme_surprize(&poodle);
  // // monotreme_surprize(&ostridge);
  // monotreme_surprize(&platypus);















	println!("");
	println!("---------------");
	println!("");
}