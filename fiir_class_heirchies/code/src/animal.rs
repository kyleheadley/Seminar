use std::default::Default;

pub struct Animal {
	lifeform: String,
	sound: String,
}

impl Default for Animal {
	fn default() -> Self {
		Animal {
			lifeform: String::from("animal"),
			sound: String::from("animal sound"),
		}
	}
}

impl Animal {
	pub fn with_sound(sound: &str) -> Animal {
		Animal{
			sound: String::from(sound),
			..Default::default()
		}
	}

	pub fn what_is_it(&self) {
		println!("{}", self.lifeform);
	}
	pub fn speak(&self) {
		println!("{}", self.sound);
	}
}
