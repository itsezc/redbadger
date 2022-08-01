use std::collections::HashMap;

use crate::{Grid, Robot};

pub trait InstructionHandler {
	fn handle(&self, grid: &Grid, robot: &mut Robot, scents: &mut HashMap<String, bool>);
}

pub struct LeftHandler;
pub struct RightHandler;
pub struct ForwardHandler;
