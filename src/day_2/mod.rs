use crate::common;

pub fn run() {
    print!("day 2: ");

    // let data = common::gather_data("./data/adventofcode.com_2023_day_2_input.txt");
    let data = common::gather_data("./data/adventofcode.com_2023_day_2_input.txt");
    let games = common::organize_data_into_lines(&data);
    let possible_sum = sum_possible_games(&games);
    let game_power_sum = sum_game_powers(&games);

    println!("{}, {}", possible_sum, game_power_sum);
}


pub fn game_is_possible(game: &str) -> bool {

    let title_handfuls: Vec<&str> = game.split(": ").collect();
    if title_handfuls.len() == 2 {

        // println!("{}", title_handfuls[0]);
        let handfuls = title_handfuls[1];
        for handful in handfuls.split("; ") {

            for color_count in handful.split(", ") {
                let color_and_count: Vec<&str> = color_count.split(" ").collect();
                let color = color_and_count[1];
                let count: usize = color_and_count[0].parse().unwrap();
                if color == "red" {
                    if count > 12 {
                        return false
                    }
                } else if color == "green" {
                    if count > 13 {
                        return false
                    }

                } else if color == "blue" {
                    if count > 14 {
                        return false
                    }

                } else {
                    panic!();
                };
            }
        }
    } else {
        return false
    }
    true
}


pub fn sum_possible_games(games: &Vec<&str>) -> usize {
    let mut sum = 0;
    for (index, game) in games.iter().enumerate()  {
        // println!("{}", index);
        // println!("");
        if game != &" " {
            if game_is_possible(game) {

                let id = index + 1;
                sum += id;
            }
        } else {
            dbg!(index);
        }
    }
    sum
}

pub fn game_power(game: &str) -> usize {

    let mut min_viable_red = 0;
    let mut min_viable_green = 0;
    let mut min_viable_blue = 0;

    let title_handfuls: Vec<&str> = game.split(": ").collect();


    if title_handfuls.len() == 2 {

        // println!("{}", title_handfuls[0]);
        let handfuls = title_handfuls[1];


        for handful in handfuls.split("; ") {

            for color_count in handful.split(", ") {
                let color_and_count: Vec<&str> = color_count.split(" ").collect();
                let color = color_and_count[1];
                let count: usize = color_and_count[0].parse().unwrap();
                if color == "red" {
                    if count > min_viable_red {
                        min_viable_red = count;
                    }
                } else if color == "green" {
                    if count > min_viable_green {
                        min_viable_green = count;
                    }
                    
                } else if color == "blue" {
                    if count > min_viable_blue {
                        min_viable_blue = count;
                    }

                } else {
                    panic!();
                };
            }
        }
    } else {
    }

    let power = min_viable_red * min_viable_green * min_viable_blue;
    power
}

pub fn sum_game_powers(games: &Vec<&str>) -> usize {
    let mut sum = 0;
    for game in games.iter()  {
        // println!("{}", index);
        // println!("");
        if game != &" " {
            sum += game_power(game);
        } else {
        }
    }
    sum
}
