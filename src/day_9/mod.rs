
use crate::common;


pub fn run() {
    print!("day 9: ");

    let start = std::time::SystemTime::now();

    let data = common::gather_data("./data/adventofcode.com_2023_day_9_input.txt");
    let lines = common::organize_data_into_lines(&data);
    let sequences = get_sequences(&lines);


    let (p1, p2) = parts_1_and_2(&sequences);

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

fn parts_1_and_2(sequences: &Vec<Vec<isize>>) -> (isize, isize) {

    let mut next_sum = 0;
    let mut previous_sum = 0;

    for sequence in sequences {
        let (starts, ends) = process(sequence);
        let next = ends.iter().fold(0, |acc, x| acc + x);
        next_sum += next;
        let previous = starts.iter().rev().fold(0, |acc, x| x - acc);
        previous_sum += previous;
    }
    (next_sum, previous_sum)
}


fn get_sequences(lines: &Vec<&str>) -> Vec<Vec<isize>> {
    let mut sequences = Vec::new();
    for line in lines {
        if line == &"" {continue};
        let seq = line.split_whitespace().map(|x| x.parse::<isize>().unwrap()).collect();
        sequences.push(seq);
    }
    sequences
}


fn reduce(input: &Vec<isize>) -> Vec<isize> {

    let mut output = Vec::new();
    for i in 1..input.len() {
        output.push(input[i] - input[i-1]);
    }
    output

}

fn process(input: &Vec<isize>) -> (Vec<isize>, Vec<isize>) {
    let mut ends = Vec::new();
    let mut starts = Vec::new();
    
    ends.push(input.last().unwrap().clone());
    starts.push(input[0].clone());
    let mut reduction = reduce(input);
    ends.push(reduction.last().unwrap().clone());
    starts.push(reduction[0].clone());
    while reduction.iter().position(|x| x != &0).is_some() {
        reduction = reduce(&reduction);
        ends.push(reduction.last().unwrap().clone());
        starts.push(reduction[0].clone());
    }
    (starts, ends)
}
