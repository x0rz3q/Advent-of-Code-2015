use std::collections::HashSet;
use std::time::Instant;

#[derive(Hash, Clone, Debug, Copy)]
struct Coordinate {
	x: i64,
	y: i64
}

impl Coordinate {
	fn new(x: i64, y: i64) -> Coordinate {
		Coordinate{x, y}
	}

	fn right(&self) -> Coordinate {
		Coordinate::new(self.x + 1, self.y)
	}

	fn left(&self) -> Coordinate {
		Coordinate::new(self.x - 1, self.y)
	}

	fn up(&self) -> Coordinate {
		Coordinate::new(self.x, self.y - 1)
	}

	fn down(&self) -> Coordinate {
		Coordinate::new(self.x, self.y + 1)
	}
}

impl Eq for Coordinate {}

impl PartialEq for Coordinate {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x && self.y == other.y
	}
}

fn silver(input: &Vec<char>) -> usize {
	let mut coordinate = Coordinate::new(0, 0);

	input.iter().map(|x| {
		coordinate = match x {
			'>' => coordinate.right(),
			'<' => coordinate.left(),
			'^' => coordinate.up(),
			'v' => coordinate.down(),
			_ => unreachable!()
		};

		coordinate
	}).collect::<HashSet<Coordinate>>().len()
}

fn gold(input: &Vec<char>) -> usize {
	let mut santa = Coordinate::new(0, 0);
	let mut robot = Coordinate::new(0, 0);

	input.iter().enumerate().map(|(i, x)| {
		let mut coordinate = match i % 2 {
			0 => santa,
			1 => robot,
			_ => unreachable!()
		};

		coordinate = match x {
			'>' => coordinate.right(),
			'<' => coordinate.left(),
			'^' => coordinate.up(),
			'v' => coordinate.down(),
			_ => unreachable!()
		};

		if i % 2 == 0 {
			santa = coordinate;
		} else {
			robot = coordinate;
		}

		coordinate
	}).collect::<HashSet<Coordinate>>().len()
}

fn main() {
	let now = Instant::now();

	let input: Vec<char> = include_str!("input")
		.trim()
		.chars()
		.collect();

	println!("Silver: {}", silver(&input));
	println!("Gold: {}", gold(&input));

	println!("Time: {}ms", now.elapsed().as_millis());
}
