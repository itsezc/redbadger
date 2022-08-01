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
