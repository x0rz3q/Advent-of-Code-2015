use std::time::Instant;
use md5::{Md5, Digest};

fn silver(input: &str) -> u128 {
	let mut value: u128 = 0;
	let mut hasher = Md5::new();

	loop {
		hasher.update(input);
		hasher.update(value.to_string());

		let result = format!("{:x}", hasher.finalize_reset());

		if result[..5] == *"00000" {
			return value;
		}

		value += 1;
	}
}

fn gold(input: &str) -> u128 {
	let mut value: u128 = 0;
	let mut hasher = Md5::new();

	loop {
		hasher.update(input);
		hasher.update(value.to_string());

		let result = format!("{:x}", hasher.finalize_reset());

		if result[..6] == *"000000" {
			return value;
		}

		value += 1;
	}
}

fn main() {
	let now = Instant::now();

	let input = "iwrupvqb";
	println!("Silver: {}", silver(input));
	println!("Silver: {}", gold(input));

	println!("Time: {}ms", now.elapsed().as_millis());
}
