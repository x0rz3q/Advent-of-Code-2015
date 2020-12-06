use colored::*;
use itertools::Itertools;
use regex::Regex;
use std::{
	cmp,
	collections::{HashMap, HashSet},
	time::Instant,
};

#[derive(Hash, Clone, Debug, Copy)]
struct Coordinate {
	x: i64,
	y: i64,
}

impl Coordinate {
	fn new(x: i64, y: i64) -> Coordinate {
		Coordinate { x, y }
	}
}

impl Eq for Coordinate {
}

impl PartialEq for Coordinate {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x && self.y == other.y
	}
}

fn silver(input: &Vec<String>) -> usize {
	let vowels = vec!['a', 'e', 'i', 'o', 'u'];
	let pairs = vec!["ab", "cd", "pq", "xy"];

	input
		.iter()
		.filter(|x| {
			pairs.iter().all(|y| !x.contains(y))
				&& x.chars().filter(|x| vowels.contains(x)).count() >= 3
				&& x.chars().any(|y| x.contains(&format!("{}{}", y, y)))
		})
		.count()
}

fn gold(input: &Vec<String>) -> usize {
	input
		.iter()
		.filter(|x| {
			x.chars()
				.cartesian_product(x.chars())
				.any(|(a, b)| x.matches(&format!("{}{}{}", a, b, a)).count() > 0)
			&& 
			x.chars()
				.cartesian_product(x.chars())
				.any(|(a, b)| x.matches(&format!("{}{}", a, b)).count() > 1)
		})
		.count()
}

fn main() {
	let now = Instant::now();

	let input: Vec<String> = include_str!("input")
		.trim()
		.split('\n')
		.map(|x| x.to_string())
		.collect();

	assert_eq!(1, silver(&vec!["ugknbfddgicrmopn".to_string()]));
	assert_eq!(0, silver(&vec!["jchzalrnumimnmhp".to_string()]));

	println!("Silver: {}", silver(&input));
	println!("Gold: {}", gold(&input));

	println!("Time: {}ms", now.elapsed().as_millis());
}
