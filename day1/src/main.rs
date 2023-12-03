use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut part1_num_sum = 0;
    let mut part2_num_sum = 0;

    if let Ok(lines) = read_lines("input/day1.txt".to_string()) {
        part1_num_sum = sum_part1(lines);
    }

    if let Ok(lines) = read_lines("input/day1.txt".to_string()) {
        part2_num_sum = sum_part2(lines);
    }

    println!("Part 1 result is: {}", part1_num_sum);
    println!("Part 2 result is: {}", part2_num_sum);
}

fn sum_part1(lines: std::io::Lines<io::BufReader<File>>) -> i64
{
    let mut num_sum = 0;

    for line in lines {
        let s_line = line.unwrap();
        let digits : Vec<i64> = s_line.chars().filter_map(|x| x.to_digit(10).map(|x| x as i64)).collect();
        let num = digits[0] * 10 + digits[digits.len() - 1];

        num_sum += num;
    }

    num_sum
}

fn sum_part2(lines: std::io::Lines<io::BufReader<File>>) -> i64
{
    let mut num_sum = 0;

    let str_digits = [("one", "1"), ("two", "2"), ("tw", "2"),
        ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"),
        ("eight", "8"), ("eigh", "8"), ("ight", "8"), ("nine", "9"), ("nin", "9"), ("ine", "9")];

    for line in lines {
        let mut s_line = line.unwrap();

        for i in 0..str_digits.len() {
            s_line = s_line.replace(str_digits[i].0, str_digits[i].1);
        }

        let digits : Vec<i64> = s_line.chars().filter_map(|x| x.to_digit(10).map(|x| x as i64)).collect();
        let num = digits[0] * 10 + digits[digits.len() - 1];

        num_sum += num;
    }

    num_sum
}

fn read_lines(filename : String) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
