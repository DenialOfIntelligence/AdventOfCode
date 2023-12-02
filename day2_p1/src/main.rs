use regex::Regex;
use std::fs;

fn main() {
    let data = fs::read_to_string("data.txt").expect("Unable to read file");
    let re = Regex::new("Game [0-9]*: ").unwrap();
    let match_game_number = Regex::new("[0-9]?[0-9]?[0-9]").unwrap();
    let mut possible_games:Vec<i32> = vec![];

    for line in data.lines() {
        let game_number = match_game_number.find(line).unwrap().as_str();
        let s = re.replace(line, "");
        let pulls = s.split("; ");
        let mut possible = true;
        for pull in pulls {
            
            let cubes = pull.split(", ");
            for cube in cubes {
                let vec:Vec<&str> = cube.split(" ").collect();
                match *vec.last().unwrap() {
                    "red" => {
                        if vec.first().unwrap().parse::<i32>().unwrap() > 12 {
                            possible=false;
                            break;
                        }
                    }
                    "green" => {
                        if vec.first().unwrap().parse::<i32>().unwrap() > 13 {
                            possible=false;
                            break;
                        }
                    }
                    "blue" => {
                        if vec.first().unwrap().parse::<i32>().unwrap() > 14 {
                            possible=false;
                            break;
                        }
                    }
                    _ => {panic!()}
                }
            }
        }
        if possible {
            possible_games.push(game_number.parse().unwrap());
        }
    }
    
    let total:i32 = possible_games.iter().sum();
    
    println!("Len of possible games is {}", possible_games.len());
    println!("Total is: {}", total)
}
