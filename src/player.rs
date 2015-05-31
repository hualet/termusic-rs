use std::process::{Command, Child};

pub struct Player {
	child: Option<Child>,
}

impl Player {
	pub fn new() -> Player {
		return Player{child: None};
	}

	pub fn play(&mut self, source: &str) {
		// match self.child {
		// 	Some(_child) => _child.kill(),
		// 	None => (),
		// }

		let child = Command::new("afplay")
							.arg(source)
							.spawn()
							.unwrap();

		self.child = Some(child);
	}
}