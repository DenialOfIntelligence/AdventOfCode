use regex::Regex;
use std::fs;

fn remove_start(line: &str) -> String {
    let remove_start = Regex::new("Card * [0-9]+:").unwrap();
    let s: String = remove_start.replace(line, "").to_string().to_owned();
    s
}
fn get_numbers(line: &str, index: usize) -> Vec<u32> {
    let parsed_string = numbers_split(line, index);
    let mut v: Vec<u32> = Vec::new();
    for s in parsed_string.split(" ") {
        if s.len() != 0 {
            println!("Number found: {s}");
            v.push(s.parse().unwrap());
        }
    }
    v
}

fn numbers_split(line: &str, index: usize) -> String {
    let s = remove_start(line);
    return s.split("|").nth(index).unwrap().to_owned();
}

fn find_winners(win: Vec<u32>, num: Vec<u32>) -> u32 {
    let mut points: u32 = 0;
    for x in win.iter() {
        print!("Win number is {x} ");
        if num.contains(x) {
            println!("and is in the numbers");
            if points == 0 {
                points = 1;
                println!("First match");
            } else {
                points *= 2;
                println!("another match");
            }
        } else {
            println!("and not in the numbers");
        }
    }
    points
}
fn main() {
    let data = fs::read_to_string("data.txt").expect("Can't read file :(");
    let mut total: u32 = 0;
    for (card_id, line) in data.lines().enumerate() {
        let winning_numbers = get_numbers(line, 0);
        let numbers = get_numbers(line, 1);
        println!("Card id: {}", card_id + 1);
        let card_value = find_winners(winning_numbers, numbers);
        println!("The value for this card is {card_value}");
        total += card_value;
    }
    println!("Our total is {total}");
}
