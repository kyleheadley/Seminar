use std::ops::Deref;
use std::default::Default;

use animal::Animal;

pub trait Mammilian {
	fn hair(&self) -> &String;
}

pub struct Mammal {
	animal: Animal,
	hairstyle: String,
}

impl Default for Mammal {
	fn default() -> Self {
		Mammal {
			animal: Animal::default(),
			hairstyle: String::from("straight"),
		}
	}
}

impl Mammal {
	pub fn new(sound: &str, hair: &str) -> Mammal {
		Mammal {
			animal: Animal::with_sound(sound),
			hairstyle: String::from(hair),
		}
	}
}

impl Mammilian for Mammal {
	fn hair(&self) -> &String {
		&self.hairstyle
	}
}

impl Deref for Mammal {
	type Target = Animal;
	fn deref(&self) -> &Self::Target {
		&self.animal
	}
}

