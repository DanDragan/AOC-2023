use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    if let Ok(lines) = read_lines("input/day2.txt".to_string()) {
        solution_games(lines);
    }
}

fn solution_games(lines: std::io::Lines<io::BufReader<File>>)
{
    let mut sum_games = 0;
    let mut prod_games = 0;

    for line in lines {
        let s_line = line.unwrap();
        let (id, data) = s_line.split_once(": ").unwrap();
        let game: Vec<&str> = data.split("; ").into_iter().collect();
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;

        let (_, game_id_str) = id.split_once(" ").unwrap();

        let game_id = game_id_str.parse::<i64>().unwrap();

        for r in game {
            for c in r.split(", ") {
                let (amount, color) = c.split_once(" ").unwrap();
                let amount_num = amount.parse::<i64>().unwrap();

                if color == "blue" {
                    if blue < amount_num {
                        blue = amount_num;
                    }
                }
                else if color == "red" {
                    if red < amount_num {
                        red = amount_num;
                    }
                }
                else if color == "green" {
                    if green < amount_num {
                        green = amount_num;
                    }
                }
            }
        }

        if red <= 12 && green <= 13 && blue <= 14 {
            sum_games += game_id;
        }

        prod_games += red * green * blue;
    }

    println!("Part 1 result is: {}", sum_games);
    println!("Part 2 result is: {}", prod_games);
}

fn read_lines(filename : String) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}