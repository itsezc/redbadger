use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use handlers::{ForwardHandler, InstructionHandler, LeftHandler, RightHandler};
use robot::Robot;

pub mod handlers;
pub mod robot;

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

impl TryInto<Orientation> for &str {
	type Error = &'static str;

	fn try_into(self) -> Result<Orientation, Self::Error> {
		match self {
			"N" => Ok(Orientation::North),
			"E" => Ok(Orientation::East),
			"S" => Ok(Orientation::South),
			"W" => Ok(Orientation::West),
			_ => Err("Invalid coordinate orientation"),
		}
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
		let parts = self.split(' ').collect::<Vec<_>>();

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
	instruction_handlers: HashMap<char, Box<dyn InstructionHandler>>,
}

impl Grid {
	pub fn new(max_x: u8, max_y: u8) -> Self {
		let mut instruction_handlers: HashMap<_, Box<dyn InstructionHandler>> = HashMap::new();

		instruction_handlers.insert('L', Box::new(LeftHandler));
		instruction_handlers.insert('R', Box::new(RightHandler));
		instruction_handlers.insert('F', Box::new(ForwardHandler));

		Self {
			map: Map::new(max_x, max_y),
			instruction_handlers,
		}
	}

	pub fn process_instructions(
		&mut self,
		robot: &mut Robot,
		scents: &mut HashMap<String, bool>,
		instruction_line: String,
	) {
		for instruction in instruction_line.chars() {
			if let Some(handler) = self.instruction_handlers.get(&instruction) {
				handler.handle(self, robot, scents)
			}
		}
	}
}
