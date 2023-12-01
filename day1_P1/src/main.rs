use std::{vec, fs};


struct Item {
    first: u32,
    second: u32,
    total: u32
}
impl Item {
    fn calc(&mut self){
        let mut s = String::new();

        
        s.push_str(&self.first.to_string());
        s.push_str(&self.second.to_string());
        self.total = s.parse::<u32>().unwrap();
    }   
}


fn parse(line: &str) -> Option<Item>{
    
    let mut v:Vec<u32> = vec![];
    for c in line.chars() {
        if c.is_numeric() {
            v.push(c.to_digit(10).unwrap());
        }
    }
    
    let item:Option<Item> = Some(Item{
        first: v.first().unwrap().clone(),
        second: v.last().unwrap().clone(),
        total: 0
    });
    
    item
    
    
}
fn main() {
    let mut results:Vec<Item> = vec![];
    let data = fs::read_to_string("data.txt").expect("Unable to read file");
    let mut total:u128 = 0;
    for line in data.lines(){
        results.push(parse(line).unwrap());
    }
    
    for mut item in results {
        item.calc();
        total= total + u128::from(item.total);
    }
    println!("Our result is: {total}")
}
