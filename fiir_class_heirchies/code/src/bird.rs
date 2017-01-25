use std::ops::Deref;
use std::default::Default;

use animal::Animal;

pub trait Avian {
	fn eggs(&self) -> &String;
}

pub struct Bird {
	animal: Animal,
	eggs: String,
}

impl Default for Bird {
	fn default() -> Self {
		Bird {
			animal: Animal::default(),
			eggs: String::from("round"),
		}
	}
}

impl Bird {
	pub fn new(sound: &str, eggs: &str) -> Bird {
		Bird {
			animal: Animal::with_sound(sound),
			eggs: String::from(eggs),
		}
	}
}

impl Avian for Bird {
	fn eggs(&self) -> &String {
		&self.eggs
	}
}

impl Deref for Bird {
	type Target = Animal;
	fn deref(&self) -> &Self::Target {
		&self.animal
	}
}

