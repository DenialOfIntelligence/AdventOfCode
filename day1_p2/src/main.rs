use std::{fs, vec};

struct Item {
    first: u32,
    second: u32,
    total: u32,
}
impl Item {
    fn calc(&mut self) {
        let mut s = String::new();

        s.push_str(&self.first.to_string());
        s.push_str(&self.second.to_string());
        self.total = s.parse::<u32>().unwrap();
    }
}
fn regex_parse(line: &str) -> String {
    let mut s: String;
    s = line.replace("one", "o1e");
    s = s.replace("two", "t2o");
    s = s.replace("three", "t3e");
    s = s.replace("four", "f4r");
    s = s.replace("five", "f5e");
    s = s.replace("six", "s6x");
    s = s.replace("seven", "s7n");
    s = s.replace("eight", "e8t");
    s = s.replace("nine", "n9e");

    s
}

fn parse(input: &str) -> Option<Item> {
    let line = regex_parse(input);
    let mut v: Vec<u32> = vec![];
    for c in line.chars() {
        if c.is_numeric() {
            v.push(c.to_digit(10).unwrap());
        }
    }

    let item: Option<Item> = Some(Item {
        first: *v.first().unwrap(),
        second: *v.last().unwrap(),
        total: 0,
    });

    item
}
fn main() {
    let mut results: Vec<Item> = vec![];
    let data = fs::read_to_string("data.txt").expect("Unable to read file");
    let mut total: u128 = 0;
    for line in data.lines() {
        results.push(parse(line).unwrap());
    }

    for mut item in results {
        item.calc();
        total += u128::from(item.total);
    }
    println!("Our result is: {total}")
}
