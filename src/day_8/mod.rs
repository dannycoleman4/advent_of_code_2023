use crate::common;
use std::collections::HashMap;


pub fn run() {
    print!("day 8: ");

    let start = std::time::SystemTime::now();

    let data = common::gather_data("./data/adventofcode.com_2023_day_8_input.txt");
    let lines = common::organize_data_into_lines(&data);

    let steps = get_steps(&lines);
    let nodes = get_nodes(&lines);

    let p1 = part_1(&steps, &nodes);
    let p2 = part_2(&steps, &nodes);

    let elapsed = start.elapsed().unwrap().as_millis();

    println!("{}, {} ({} ms) ", p1, p2, elapsed);
}


// fn lowest_for_ranges(raw_seed_data: &Vec<usize>, maps: &Maps) -> usize {
//     let ranges = calc_seed_ranges(raw_seed_data);
//     let mut global_min = usize::MAX;
//     for range in ranges {
//         let m = min_for_seed_range(range[0], range[1], maps);
//         if m < global_min {
//             global_min = m;
//         }
//     }
//     global_min
// }

fn part_1(steps: &Vec<Direction>, nodes: &Nodes) -> usize {
    let steps = nodes.calc_steps(&steps);
    steps
}

fn part_2(steps: &Vec<Direction>, nodes: &Nodes) -> usize {
    let steps = nodes.calc_steps2(&steps);
    steps
}


#[derive(Debug)]
enum Direction {
    Right, 
    Left,
}

#[derive(Debug)]
struct Neighbors {
    right: String,
    left: String,
}

#[derive(Debug)]
struct Nodes( HashMap<String, Neighbors> ); 

impl Nodes {

    fn calc_steps(&self, directions: &Vec<Direction>) -> usize {
        let mut steps = 0;
        let mut id = "AAA";

        while id != "ZZZ" {
            for direction in directions {

                steps += 1;
                match direction {
                    Direction::Right => {
                        id = &self.0[id].right;
                    }, 
                    Direction::Left => {
                        id = &self.0[id].left;
                    }
                }
            }
        }
        steps
    }

    fn calc_steps2(&self, directions: &Vec<Direction>) -> usize {
        let mut steps: usize = 1;
        let mut ids: Vec<String> = self.0.keys().filter(|x| x.ends_with("A")).cloned().collect();
        ids.sort();

        // for id in ids.iter_mut() {
        // while ids.iter().find(|x| !x.ends_with("Z")).is_some() {
        for id in ids.iter_mut() {
            let mut last = 0;

            let mut id_steps = 0;
            while !id.ends_with("Z") {
            // while true {

            
                for direction in directions {
                    id_steps += 1;

                    match direction {
                        Direction::Right => {
                            *id = self.0[id].right.clone();
                        }, 
                        Direction::Left => {
                            *id = self.0[id].left.clone();
                        }
                    }
                    if id.ends_with("Z") {
                        // println!("{}, {}", id_steps, id_steps - last);
                        steps = least_common_multiple(steps, id_steps);
                        // println!("{}", steps);
                        last = id_steps;
                        break
                    }
                }
            }
            // steps *= id_steps;
        }
        steps
    }
}

fn greatest_common_denominator(num: usize, den: usize) -> usize {

    let mut den = den;
    let mut num = num;

    loop {

        let m = num % den;
        if m == 0 {
            return den
        } else {
            den = m
        }
    }
}

fn least_common_multiple (a: usize, b: usize) -> usize {

    (a * b) / greatest_common_denominator(a, b)
}


fn get_steps(lines: &Vec<&str>) -> Vec<Direction> {
    let mut steps = Vec::new();
    let chars = lines[0].chars();
    for c in chars {
        if c == 'R' {
            steps.push(Direction::Right);
        } else if c == 'L' {
            steps.push(Direction::Left);
        } else {
            panic!("");
        }
    }
    steps
}

fn get_nodes(lines: &Vec<&str>) -> Nodes {
    let mut nodes = HashMap::new();

    for line in &lines[1..] {
        if line == &"" {
            continue
        };

        let chars: Vec<char> = line.chars().collect();
        let mut id = String::new();
        for i in 0..3 {
            id.push(chars[i]);
        }
        let mut left = String::new();
        for i in 7..10 {
            left.push(chars[i]);
        }
        let mut right = String::new();
        for i in 12..15 {
            right.push(chars[i]);
        }
        let neighbors = Neighbors {
            right, left
        };
        let previous = nodes.insert(id, neighbors);
        assert!(previous.is_none());
    }
    Nodes(nodes)
}



