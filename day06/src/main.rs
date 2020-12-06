use std::collections::HashMap;
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

	fn from_string(s: &str) -> Coordinate {
		let parts: Vec<&str> = s.split(",").collect();

		let x = parts.get(0).unwrap().parse().unwrap();
		let y = parts.get(1).unwrap().parse().unwrap();

		Coordinate::new(x, y)
	}
}

impl Eq for Coordinate {}

impl PartialEq for Coordinate {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x && self.y == other.y
	}
}

struct Grid {
	coordinates: HashMap<Coordinate, bool>
}

impl Grid {
	fn new() -> Grid {
		Grid {
			coordinates: HashMap::new()
		}
	}

	fn toggle(&mut self, from: Coordinate, to: Coordinate) {
		for x in from.x..=to.x {
			for y in from.y..=to.y {
				let entry = self.coordinates.entry(Coordinate::new(x, y)).or_insert(false);
				*entry = !*entry;
			}
		}
	}

	fn set(&mut self, from: Coordinate, to: Coordinate, action: bool) {
		for x in from.x..=to.x {
			for y in from.y..=to.y {
				let entry = self.coordinates.entry(Coordinate::new(x, y)).or_insert(false);
				*entry = action;
			}
		}
	}
}

struct BrightnessGrid {
	coordinates: HashMap<Coordinate, u128>
}

impl BrightnessGrid {
	fn new() -> BrightnessGrid {
		BrightnessGrid {
			coordinates: HashMap::new()
		}
	}

	fn toggle(&mut self, from: Coordinate, to: Coordinate) {
		for x in from.x..=to.x {
			for y in from.y..=to.y {
				let entry = self.coordinates.entry(Coordinate::new(x, y)).or_insert(0);
				*entry = *entry + 2;
			}
		}
	}

	fn set(&mut self, from: Coordinate, to: Coordinate, action: bool) {
		for x in from.x..=to.x {
			for y in from.y..=to.y {
				let entry = self.coordinates.entry(Coordinate::new(x, y)).or_insert(0);

				if action {
					*entry = *entry + 1;
				} else if *entry > 0{
					*entry = *entry - 1;
				}
			}
		}
	}
}

fn silver(input: &Vec<String>) -> usize {
	let mut grid = Grid::new();

	input.iter().for_each(|x| {
		let parts: Vec<&str> = x.split(" ").collect();
		let action: &str = parts.get(0).unwrap();

		match action {
			"turn" => {
				let sub_action: &str = parts.get(1).unwrap();
				grid.set(Coordinate::from_string(parts.get(2).unwrap()), Coordinate::from_string(parts.get(4).unwrap()), sub_action == "on");
			},
			"toggle" => {
				grid.toggle(Coordinate::from_string(parts.get(1).unwrap()), Coordinate::from_string(parts.get(3).unwrap()));
			},
			_ => unreachable!()
		};
	});

	grid.coordinates.iter().filter(|(_, val)| **val).count()
}

fn gold(input: &Vec<String>) -> u128 {
	let mut grid = BrightnessGrid::new();

	input.iter().for_each(|x| {
		let parts: Vec<&str> = x.split(" ").collect();
		let action: &str = parts.get(0).unwrap();

		match action {
			"turn" => {
				let sub_action: &str = parts.get(1).unwrap();
				grid.set(Coordinate::from_string(parts.get(2).unwrap()), Coordinate::from_string(parts.get(4).unwrap()), sub_action == "on");
			},
			"toggle" => {
				grid.toggle(Coordinate::from_string(parts.get(1).unwrap()), Coordinate::from_string(parts.get(3).unwrap()));
			},
			_ => unreachable!()
		};
	});

	grid.coordinates.iter().map(|(_, val)| val).sum()
}

fn main() {
	let now = Instant::now();

	let input: Vec<String> = include_str!("input")
		.trim()
		.split('\n')
		.map(|x| x.to_string())
		.collect();

	println!("Silver: {}", silver(&input));
	println!("Gold: {}", gold(&input));

	println!("Time: {}ms", now.elapsed().as_millis());
}
