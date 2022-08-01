use super::Position;

pub enum RobotStatus {
	Located,
	Lost,
}

pub struct Robot {
	pub last_position: Position,
	pub status: RobotStatus,
}

impl Robot {
	pub fn new(position: Position) -> Self {
		Self {
			last_position: position,
			status: RobotStatus::Located,
		}
	}
}
