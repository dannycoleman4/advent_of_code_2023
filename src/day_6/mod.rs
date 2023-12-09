use crate::common;
use std::str::FromStr;


pub fn run() {
    print!("day 6: ");

    let start = std::time::SystemTime::now();

    let data = common::gather_data("./data/adventofcode.com_2023_day_6_input.txt");
    let lines = common::organize_data_into_lines(&data);



    let p1 = part_1(&lines);
    let p2 = part_2(&lines);

    let elapsed = start.elapsed().unwrap().as_millis();
    println!("{}, {} ({} ms) ", p1, p2, elapsed);
}




#[derive(Debug)]
struct Race {
    time: f64,
    distance: f64,
}

pub fn part_1(lines: &Vec<&str>) -> usize {
    let races = load_races(&lines);
    let mut solutions = 1;

    for race in &races {
        let s = race.winning_outcomes();
        solutions *= s;
    }

    solutions
}

pub fn part_2(lines: &Vec<&str>) -> usize {

    let race = load_race(&lines);
    let s = race.winning_outcomes();
    s
}

impl Race {
    fn winning_outcomes(&self) -> usize {
        let a = 1.0;
        let b = 0.0 - self.time;
        let c = self.distance; 

        let (r1, r2) = solve_quadratic_formula(a, b, c);

        let highest_possibility = (r1).ceil() as usize - 1;
        let lowest_possibility = (r2).floor() as usize + 1;



        let count = highest_possibility - lowest_possibility + 1;
        count
    }
}


fn solve_quadratic_formula(a: f64, b: f64, c: f64) -> (f64, f64) {
    let root_term = ((b*b) - 4.0*a*c).sqrt();

    let r1 =  ( - b + root_term ) / 2.0*a;
    let r2 =  ( - b - root_term ) / 2.0*a;

    (r1, r2)
}

fn load_races(lines: &Vec<&str>) -> Vec<Race> {

    let times: Vec<f64> = lines[0]
        .split_whitespace()
        .skip(1)
        .map( |x| f64::from_str(x).unwrap()).collect();

    let distances: Vec<f64> = lines[1]
        .split_whitespace()
        .skip(1)
        .map( |x| f64::from_str(x).unwrap()).collect();


    let mut races = Vec::new();
    for i in 0..times.len() {

        let race = Race {
            time: times[i],
            distance: distances[i],
        };
        races.push(race);
    }
    races
}

fn load_race(lines: &Vec<&str>) -> Race {

    let time_string: String = lines[0]
        .split(":")
        .nth(1).unwrap()
        .replace(" ", "");

    let time: f64 = time_string.parse().unwrap();

    let distance_string: String = lines[1]
        .split(":")
        .nth(1).unwrap()
        .replace(" ", "");

    let distance: f64 = distance_string.parse().unwrap();

    let race = Race {
        time,
        distance,
    };
    race 
}
