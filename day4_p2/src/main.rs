use regex::Regex;
use std::fs;

#[derive(Clone, Debug, Copy)]
struct Card {
    wins: u32,
    id: u32,
    owned: u32,
}

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
        
        if num.contains(x) {
            
            points += 1;
        } else {
            
        }
    }
    points
}
fn main() {
    let data = fs::read_to_string("data.txt").expect("Can't read file :(");
    let mut total: Vec<Card> = Vec::new();
    for (card_id, line) in data.lines().enumerate() {
        let winning_numbers = get_numbers(line, 0);
        let numbers = get_numbers(line, 1);
        
        let card_value = find_winners(winning_numbers, numbers);
        
        let card = Card {
            id: card_id as u32,
            wins: card_value,
            owned: 1,
        };
        total.push(card);
    }
    
    for (index, _) in total.clone().iter().enumerate() {
        
        let card = total.get(index).unwrap().clone().to_owned();
        
        for i in card.id + 1..card.wins + card.id + 1 {
            println!("i is {i}");
            let index = total.iter().position(|&r| r.id == i).unwrap();
            let mut c = total.get(index).unwrap().to_owned();
            
            c.owned += card.owned;
            
            total[c.id as usize] = c;
        }
    }
    let mut all: u32 = 0;
    for c in total {
        all += c.owned;
    }
    println!("{all}");
}
