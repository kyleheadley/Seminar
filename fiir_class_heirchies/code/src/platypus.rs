use bird::{Bird, Avian};
use mammal::{Mammal, Mammilian};

pub trait Monotreme: Mammilian+Avian {
	fn what_is_it(&self) {
		println!("A creature with {} fur that lays {} eggs", self.hair(), self.eggs());
	}
}

pub struct Platypus {
	mammal: Mammal,
	bird: Bird,
}

impl Monotreme for Platypus {}

impl Platypus {
	pub fn new(sound: &str, hair: &str, eggs: &str) -> Platypus {
		Platypus {
			mammal: Mammal::new(sound, hair),
			bird: Bird::new(sound, eggs),
		}
	}
}



























// wrappers :-(
	
impl Avian for Platypus {
	#[inline(always)]
	fn eggs(&self) -> String { self.bird.eggs() }
}

impl Mammilian for Platypus {
	#[inline(always)]
	fn hair(&self) -> String { self.mammal.hair() }
}

