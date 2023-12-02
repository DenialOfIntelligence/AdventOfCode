use regex::Regex;
use std::fs;

fn main() {
    let data = fs::read_to_string("data.txt").expect("Unable to read file");
    let re = Regex::new("Game [0-9]*: ").unwrap();
    let mut lowest_cubes: Vec<i32> = vec![];

    for line in data.lines() {
        let s = re.replace(line, "");
        let pulls = s.split("; ");
        let [mut red, mut green, mut blue] = [0; 3];

        for pull in pulls {
            let cubes = pull.split(", ");
            for cube in cubes {
                let vec: Vec<&str> = cube.split(" ").collect();
                match *vec.last().unwrap() {
                    "red" => {
                        if vec.first().unwrap().parse::<i32>().unwrap() > red {
                            red = vec.first().unwrap().parse().unwrap();
                        }
                    }
                    "green" => {
                        if vec.first().unwrap().parse::<i32>().unwrap() > green {
                            green = vec.first().unwrap().parse().unwrap();
                        }
                    }
                    "blue" => {
                        if vec.first().unwrap().parse::<i32>().unwrap() > blue {
                            blue = vec.first().unwrap().parse().unwrap();
                        }
                    }
                    _ => {
                        panic!()
                    }
                }
            }
        }
        lowest_cubes.push(red * green * blue);
    }
    let total: i32 = lowest_cubes.iter().sum();
    println!("{}", total);
}
