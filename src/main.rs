use std::io::BufRead;
use std::{collections::HashMap, io};

mod lib;

use lib::{
	robot::{Robot, RobotStatus},
	validate_coordinate, Grid, Orientation,
};

pub fn main() -> Result<(), &'static str> {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();

	// We get the grid size from the first input line
	let first_line = lines.next().unwrap().unwrap();

	// We assume all the input is right formatted
	let parts = first_line.split(' ').collect::<Vec<_>>();
	let max_x = parts[0].parse::<u8>().unwrap();
	let max_y = parts[1].parse::<u8>().unwrap();

	let mut grid = Grid::new(validate_coordinate(max_x)?, validate_coordinate(max_y)?);
	let mut scents: HashMap<String, bool> = HashMap::new();

	loop {
		let robot_position = lines.next().unwrap().unwrap();
		let robot_instructions = lines.next().unwrap().unwrap();

		if robot_instructions.len() > 100 {
			return Err("Instruction strings has to be less than 100 characters in length.");
		}

		let mut robot = Robot::new(robot_position.try_into()?);

		grid.process_instructions(&mut robot, &mut scents, robot_instructions);

		println!(
			"{}{}",
			robot.last_position,
			if matches!(robot.status, RobotStatus::Lost) {
				" LOST"
			} else {
				""
			}
		)
	}
}

#[cfg(test)]
mod tests {
	use std::collections::HashMap;

	use crate::{lib::Position, Grid, Orientation, Robot, RobotStatus};

	#[test]
	fn it_parses_robot_position() {
		let position: Position = "10 5 E".to_string().try_into().unwrap();

		assert_eq!(position.x, 10);
		assert_eq!(position.y, 5);
		assert!(matches!(position.orientation, Orientation::East));
	}

	#[test]
	fn it_processes_instructions() {
		let mut scents = HashMap::new();
		let mut grid = Grid::new(50, 50);

		// Robot#1
		{
			let mut robot = Robot::new("1 1 E".to_string().try_into().unwrap());

			grid.process_instructions(&mut robot, &mut scents, "RFRFRFRF".to_string());

			assert_eq!(robot.last_position.x, 1);
			assert_eq!(robot.last_position.y, 1);
			assert!(matches!(robot.last_position.orientation, Orientation::East));
		}

		// Robot#2
		{
			let mut robot = Robot::new("10 10 W".to_string().try_into().unwrap());

			grid.process_instructions(&mut robot, &mut scents, "RLLFFRLFRRRFFFFLRF".to_string());

			assert_eq!(robot.last_position.x, 15);
			assert_eq!(robot.last_position.y, 7);
			assert!(matches!(robot.last_position.orientation, Orientation::East));
		}
	}

	#[test]
	fn it_can_fall_to_void() {
		let mut scents = HashMap::new();
		let mut grid = Grid::new(50, 50);

		let mut robot = Robot::new("0 0 S".to_string().try_into().unwrap());

		grid.process_instructions(&mut robot, &mut scents, "F".to_string());

		// Last position is not changed
		assert_eq!(robot.last_position.x, 0);
		assert_eq!(robot.last_position.y, 0);
		assert!(matches!(
			robot.last_position.orientation,
			Orientation::South
		));

		// It sets robot status to lost and adds scent
		assert!(matches!(robot.status, RobotStatus::Lost));
		assert_eq!(scents.len(), 1);
		assert!(scents.get("0--1").is_some());
	}

	#[test]
	fn it_cant_fall_to_void_twice_on_same_spot() {
		let mut scents = HashMap::new();
		let mut grid = Grid::new(50, 50);

		// Robot#1
		{
			let mut robot = Robot::new("0 0 S".to_string().try_into().unwrap());

			grid.process_instructions(&mut robot, &mut scents, "F".to_string());

			// Last position is not changed
			assert_eq!(robot.last_position.x, 0);
			assert_eq!(robot.last_position.y, 0);
			assert!(matches!(
				robot.last_position.orientation,
				Orientation::South
			));

			// It sets robot status to lost and adds scent
			assert!(matches!(robot.status, RobotStatus::Lost));
			assert_eq!(scents.len(), 1);
			assert!(scents.get("0--1").is_some());
		}

		// Robot#2
		{
			let mut robot = Robot::new("0 0 S".to_string().try_into().unwrap());

			grid.process_instructions(&mut robot, &mut scents, "FFFFFFFF".to_string());

			// Last position is not changed
			assert_eq!(robot.last_position.x, 0);
			assert_eq!(robot.last_position.y, 0);
			assert!(matches!(
				robot.last_position.orientation,
				Orientation::South
			));

			// And this robot didn't fall so status is Located
			assert!(matches!(robot.status, RobotStatus::Located));
		}
	}
}
