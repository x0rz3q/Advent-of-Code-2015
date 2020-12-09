use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools;
use std::time::Instant;
use regex::Regex;
use colored::*;
use std::cmp;
use std::collections::VecDeque;

struct Circuit {
	symbols: HashMap<String, String>, // key <- value, maps symbols to actions
	cache: HashMap<String, u16> // evaluation command of first part
}

impl Circuit {
	fn new() -> Circuit {
		Circuit {
			symbols: HashMap::new(),
			cache: HashMap::new()
		}
	}

	fn insert(&mut self, line: &str) {
		let mut parts = line.splitn(2, " -> ");
		let command = parts.next().unwrap().clone();
		let symbol = parts.next().unwrap().clone();

		self.symbols.insert(symbol.to_string(), command.to_string());
	}

	fn evaluate(&mut self, expression: &str) -> u16 {
		if self.cache.contains_key(expression) {
			return *self.cache.get(expression).unwrap();
		}

		if expression.chars().all(|x| x.is_ascii_digit()) {
			return expression.parse().unwrap();
		}

		// It is in the format of symbol -> symbol
		let value = if self.symbols.contains_key(expression) {
			self.evaluate(&self.symbols.get(expression).unwrap().clone())
		} else if expression.contains("NOT") {
			let symbol = expression.split(" ").skip(1).next().unwrap();
			!self.evaluate(symbol)
		} else {
			let mut parts = expression.split(" ");
			let first_symbol = self.evaluate(parts.next().unwrap().clone());
			let operation = parts.next().unwrap().clone();
			let second_symbol = self.evaluate(parts.next().unwrap().clone());

			match operation {
				"OR" => {
					first_symbol | second_symbol
				},
				"AND" => {
					first_symbol & second_symbol 
				},
				"LSHIFT" => {
					first_symbol << second_symbol as usize
				},
				"RSHIFT" => {
					first_symbol >> second_symbol
				},
				_ => unreachable!()	
			}
		}; 
	
		self.cache.insert(expression.to_string(), value);

		value
	}
}

fn silver(circuit: &mut Circuit) -> u16 {
	circuit.evaluate("a")
}

fn gold(circuit: &mut Circuit, silver: u16) -> u16 {
	circuit.cache.clear();
	circuit.insert(&format!("{} -> b", silver));

	circuit.evaluate("a")
}

fn main() {
	let now = Instant::now();
	
	let mut circuit = Circuit::new();
	include_str!("input")
		.trim()
		.split('\n')
		.for_each(|x| circuit.insert(x));

	let silver = silver(&mut circuit);
	println!("Silver: {}", silver);
	println!("Gold: {}", gold(&mut circuit, silver));

	println!("Time: {}ms", now.elapsed().as_millis());
}
