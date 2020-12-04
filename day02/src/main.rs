use colored::*;
use itertools::Itertools;
use regex::Regex;
use std::{
    cmp,
    collections::{HashMap, HashSet},
    time::Instant,
};

fn silver(input: &Vec<Vec<u64>>) -> u64 {
    input
        .iter()
        .map(|x| {
            2 * x[0] * x[1]
                + 2 * x[1] * x[2]
                + 2 * x[0] * x[2]
                + cmp::min(x[0] * x[1], cmp::min(x[1] * x[2], x[2] * x[0]))
        })
        .sum()
}

fn gold(input: &Vec<Vec<u64>>) -> u64 {
    input
        .iter()
        .map(|x| {
            x.iter().product1::<u64>().unwrap()
                + cmp::min(
                    x[0] * 2 + x[1] * 2,
                    cmp::min(x[1] * 2 + x[2] * 2, x[2] * 2 + x[0] * 2),
                )
        })
        .sum()
}

fn main() {
    let now = Instant::now();

    let input: Vec<Vec<u64>> = include_str!("input")
        .trim()
        .split('\n')
        .map(|entry| entry.split('x').map(|x| x.parse().unwrap()).collect())
        .collect();

    println!("Silver: {}", silver(&input));
    println!("Gold: {}", gold(&input));

    println!("Time: {}ms", now.elapsed().as_millis());
}
