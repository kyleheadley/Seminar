use std::ops::Deref;
use std::default::Default;

use animal::Animal;

pub struct Cat {
	animal: Animal,
	sound: String,
}

impl Default for Cat {
	fn default() -> Self {
		Cat {
			animal: Default::default(),
			sound: String::from("meow"),
		}
	}
}

impl Cat {
	pub fn speak(&self) {
		println!("{}", self.sound);
	}
}

impl Deref for Cat {
	type Target = Animal;
	fn deref(&self) -> &Self::Target {
		&self.animal
	}
}
