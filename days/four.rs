use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let file = File::open("four.txt").unwrap();
    let mut valid_count = 0;
    for line in BufReader::new(file).lines().map(|l| l.unwrap()) {
        let mut words = HashSet::new();
        let mut valid = true;
        'wordloop: for word in line.split(' ') {
            if (words.contains(word)) {
                valid = false;
                break 'wordloop;
            }
            words.insert(word);
        }
        if (valid) {
            valid_count += 1;
        }
    }
    println!("Part One Result: {}", valid_count);
}

fn part_two() {
    let file = File::open("four.txt").unwrap();
    let mut valid_count = 0;
    for line in BufReader::new(file).lines().map(|l| l.unwrap()) {
        let mut words = HashSet::new();
        let mut valid = true;
        'wordloop: for word in line.split(' ') {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort_by(|a, b| b.cmp(a));
            let sorted_word = String::from_iter(chars);
            if (words.contains(&sorted_word)) {
                valid = false;
                break 'wordloop;
            }
            words.insert(sorted_word);
        }
        if (valid) {
            valid_count += 1;
        }
    }
    println!("Part One Result: {}", valid_count);
}
