use crate::common;
use std::collections::HashMap;


pub fn run() {
    print!("day 10: ");

    let start = std::time::SystemTime::now();

    let data = common::gather_data("./data/adventofcode.com_2023_day_10_input.txt");
    let lines = common::organize_data_into_lines(&data);

    let p1 = part_1(&lines);
    let p2 = part_2(&lines);

    let elapsed = start.elapsed().unwrap().as_millis();

    println!("{}, {} ({} ms) ", p1, p2, elapsed);
}

fn part_1(lines: &Vec<&str>) -> usize {
    0
}

fn part_2(lines: &Vec<&str>) -> usize {
    0
}
