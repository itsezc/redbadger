use std::collections::HashMap;

use crate::lib::{Grid, Robot};

pub trait InstructionHandler {
	fn handle(&self, grid: &Grid, robot: &mut Robot, scents: &mut HashMap<String, bool>);
}
