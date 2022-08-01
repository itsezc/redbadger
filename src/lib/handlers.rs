use std::collections::HashMap;

use crate::{Grid, Orientation, Robot, RobotStatus};

pub trait InstructionHandler {
	fn handle(&self, grid: &Grid, robot: &mut Robot, scents: &mut HashMap<String, bool>);
}

pub struct LeftHandler;

impl InstructionHandler for LeftHandler {
	fn handle(&self, _: &Grid, robot: &mut Robot, _: &mut HashMap<String, bool>) {
		// We ignore all the instructions if the robot is lost
		if matches!(robot.status, RobotStatus::Lost) {
			return;
		}

		robot.last_position.orientation = match robot.last_position.orientation {
			Orientation::North => Orientation::West,
			Orientation::East => Orientation::North,
			Orientation::South => Orientation::East,
			Orientation::West => Orientation::South,
		}
	}
}

pub struct RightHandler;

impl InstructionHandler for RightHandler {
	fn handle(&self, _: &Grid, robot: &mut Robot, _: &mut HashMap<String, bool>) {
		// We ignore all the instructions if the robot is lost
		if matches!(robot.status, RobotStatus::Lost) {
			return;
		}

		robot.last_position.orientation = match robot.last_position.orientation {
			Orientation::North => Orientation::East,
			Orientation::East => Orientation::South,
			Orientation::South => Orientation::West,
			Orientation::West => Orientation::North,
		}
	}
}

pub struct ForwardHandler;

impl InstructionHandler for ForwardHandler {
	fn handle(&self, grid: &Grid, robot: &mut Robot, scents: &mut HashMap<String, bool>) {
		// We ignore all the instructions if the robot is lost
		if matches!(robot.status, RobotStatus::Lost) {
			return;
		}

		let mut new_x: i8 = robot.last_position.x as i8;
		let mut new_y: i8 = robot.last_position.y as i8;

		match robot.last_position.orientation {
			Orientation::North => new_y += 1,
			Orientation::East => new_x += 1,
			Orientation::South => new_y -= 1,
			Orientation::West => new_x -= 1,
		}

		let scent_key = format!("{}-{}", new_x, new_y);

		// If the processed new position is gonna be same as one contained on the scent hashmap
		// we ignore the handling.
		if scents.contains_key(&scent_key) {
			return;
		}

		// The robot will fall
		if new_x < 0 || new_x > grid.map.max_x as i8 || new_y < 0 || new_y > grid.map.max_y as i8 {
			robot.status = RobotStatus::Lost;

			scents.insert(scent_key, true);

			return;
		}

		robot.last_position.x = new_x as u8;
		robot.last_position.y = new_y as u8;
	}
}
