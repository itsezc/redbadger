use std::io;
use std::io::BufRead;

mod lib;

use lib::{
	robot::{Robot, RobotStatus},
	Grid, Orientation,
};

pub fn main() {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();

	// We get the grid size from the first input line
	let first_line = lines.next().unwrap().unwrap();
}
