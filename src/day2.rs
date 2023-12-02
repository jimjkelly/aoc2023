use aoc_runner_derive::{aoc, aoc_generator};


#[derive(Debug)]
pub struct Set {
    blue: u32,
    green: u32,
    red: u32,
}

#[derive(Debug)]
pub struct Game {
    id: u32,
    sets: Vec<Set>,
}


#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();

    for line in input.lines() {
        if let Some((info, sets)) = line.split_once(":") {
            let id = info.split(" ").nth(1).unwrap().parse::<u32>().unwrap();

            let mut game = Game {
                id,
                sets: Vec::new(),
            };

            for set_data in sets.split(";") {
                let mut set = Set {
                    blue: 0,
                    green: 0,
                    red: 0,
                };

                for color_data in set_data.split(",") {
                    let mut parts = color_data.trim().split(" ");
                    let count = parts.next().unwrap().parse::<u32>().unwrap();
                    let color = parts.next().unwrap();

                    match color {
                        "blue" => set.blue = count,
                        "green" => set.green = count,
                        "red" => set.red = count,
                        _ => panic!("Unknown color: {}", color),
                    }
                }

                game.sets.push(set);
            }

            games.push(game);
        }
    }
    games
}


#[aoc(day2, part1)]
pub fn solve_part1(record: &Vec<Game>) -> u32 {
    let bag = Set {
        blue: 14,
        green: 13,
        red: 12,
    };
    record.iter().filter(|game| {
        game.sets.iter().all(|set| {
            set.blue <= bag.blue && set.green <= bag.green && set.red <= bag.red
        })
    }).map(|game| game.id).sum()
}


#[aoc(day2, part2)]
pub fn solve_part2(record: &Vec<Game>) -> u32 {
    record.iter().map(|game| {
        let blue = game.sets.iter().max_by_key(|set| set.blue).unwrap().blue;
        let green = game.sets.iter().max_by_key(|set| set.green).unwrap().green;
        let red = game.sets.iter().max_by_key(|set| set.red).unwrap().red;
        blue * green * red
    }).sum()
}



#[cfg(test)]
mod tests {
    use crate::{day2::solve_part1, day2::solve_part2, day2::input_generator};
    static INPUT_ONE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_day2_part1() {
        let input = input_generator(&INPUT_ONE);
        let result = solve_part1(&input);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_day2_part2() {
        let input = input_generator(&INPUT_ONE);
        let result = solve_part2(&input);
        assert_eq!(result, 2286);
    }
}
