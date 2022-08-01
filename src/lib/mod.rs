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

#[derive(Debug)]
pub struct Position {
	pub x: u8,
	pub y: u8,
	pub orientation: Orientation,
}

impl Display for Position {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {} {}", self.x, self.y, self.orientation)
	}
}

impl TryInto<Position> for String {
	type Error = &'static str;

	fn try_into(self) -> Result<Position, Self::Error> {
		let parts = self.split(" ").collect::<Vec<_>>();

		let err = "Error parsing coordinate";

		let x = parts[0].parse().map_err(|_| err)?;
		let y = parts[1].parse().map_err(|_| err)?;

		Ok(Position {
			x: validate_coordinate(x)?,
			y: validate_coordinate(y)?,
			orientation: parts[2].try_into()?,
		})
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
