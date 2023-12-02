use aoc_runner_derive::aoc;


fn replace_spelled_out_numbers(input: &str) -> String {
    let mut input = input.to_owned();
    for pair in [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ] {
        input = input.replace(pair.0, pair.1);
    }
    input
}


#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut all: Vec<u32> = Vec::new();

    for line in input.lines() {
        let clean = line;
        let mut current: Vec<char> = Vec::new();
        for c in clean.chars() {
            if c.is_numeric() {
                current.push(c);
            } else {
                continue;
            }
        }

        let mut number = String::new();
        number.push(*current.first().unwrap());
        number.push(*current.last().unwrap());

        let value = match number.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        all.push(value);
    }


    all.iter().sum()
}


#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    solve_part1(&replace_spelled_out_numbers(input))
}



#[cfg(test)]
mod tests {
    use crate::{day1::solve_part1, day1::solve_part2};
    static INPUT_ONE: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    static INPUT_TWO: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

    #[test]
    fn test_day1_part1() {
        let result = solve_part1(&INPUT_ONE);
        assert_eq!(result, 142);
    }

    #[test]
    fn test_day1_part2() {
        let result = solve_part2(INPUT_TWO);
        assert_eq!(result, 281);
    }
}
