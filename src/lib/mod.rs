use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// Maximum coordinate value for the grid
pub const MAX_COORDINATE_VALUE: u8 = 50;

pub fn validate_coordinate(c: u8) -> Result<u8, &'static str> {
	if c > MAX_COORDINATE_VALUE {
		Err("Maximum value for any coordinate on the grid exceeded.")
	} else {
		Ok(c)
	}
}

#[derive(Debug)]
pub enum Orientation {
	North,
	East,
	South,
	West,
}

impl Display for Orientation {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Orientation::North => "N",
				Orientation::East => "E",
				Orientation::South => "S",
				Orientation::West => "W",
			}
		)
	}
}

pub struct Map {
	max_x: u8,
	max_y: u8,
}

impl Map {
	pub fn new(max_x: u8, max_y: u8) -> Self {
		Self { max_x, max_y }
	}
}

pub struct Grid {
	map: Map,
	instruction_handlers: HashMap<char, char>,
}
