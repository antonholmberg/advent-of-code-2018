extern crate rayon;

use rayon::prelude::*;
use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn get_input_data() -> String {
    let file_path = args().nth(2).expect("Input file path is required");
    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    return reader
        .lines()
        .nth(0)
        .expect("Could not read file")
        .expect("Could not read line");
}

fn filter_unit(unit: char, input: &String) -> String {
    input
        .chars()
        .filter(|&item| item != unit && item != upper_char(unit))
        .collect::<String>()
}

fn filter_pairs(input: &String) -> String {
    let mut next_input = input.clone();
    loop {
        let pairs = make_pairs(&next_input);
        let filtered = pairs
            .iter()
            .map(|&(x, y)| (cancels_out(x, y), (x, y)))
            .collect::<Vec<(bool, (char, char))>>();

        let unmarked = filtered
            .iter()
            .filter(|(marked, (_, _))| !marked)
            .collect::<Vec<_>>();

        if unmarked.len() == pairs.len() {
            break;
        }

        next_input = "".to_string();
        let mut iter = filtered.iter();
        while let Some((marked, (x, _))) = iter.next() {
            if !*marked {
                next_input.push(*x);
            } else {
                iter.next();
            }
        }
    }

    return next_input;
}

fn make_pairs(input: &String) -> Vec<(char, char)> {
    let mut skipped_input = input.clone();
    skipped_input.push(' ');
    let skipped = skipped_input.chars().skip(1).collect::<String>();

    input.chars().zip(skipped.chars()).collect()
}

fn cancels_out(x: char, y: char) -> bool {
    (upper_char(x) == y || upper_char(y) == x) && x != y
}

fn upper_char(ch: char) -> char {
    ch.to_uppercase()
        .collect::<String>()
        .chars()
        .nth(0)
        .expect("Could not upper case")
}

fn exercise1() {
    let input = get_input_data();
    let output = filter_pairs(&input);

    println!("{}", output.len());
}

fn exercise2() {
    let input = get_input_data();
    let chars = (97..=122).into_iter().map(|ascii| ascii as u8 as char);

    let filtered = chars
        .map(|unit| filter_unit(unit, &input))
        .collect::<Vec<String>>();

    let shortest = filtered
        .par_iter()
        .map(filter_pairs)
        .map(|s| s.len())
        .min()
        .unwrap();

    println!("Shortest is: {}", shortest);
}

fn main() {
    match args().nth(1) {
        Some(s) => {
            if s == "e1" {
                exercise1();
            } else if s == "e2" {
                exercise2();
            }
        }
        None => return,
    };
}
