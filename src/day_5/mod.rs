use crate::common;


pub fn run() {
    print!("day 5: ");

    let start = std::time::SystemTime::now();

    let data = common::gather_data("./data/adventofcode.com_2023_day_5_input.txt");
    let lines = common::organize_data_into_lines(&data);

    let seeds = get_seeds(&lines);
    let maps = get_maps(&lines);

    let p1 = part_1(&seeds, &maps);
    let p2 = part_2(&seeds, &maps);

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

fn part_1(seeds: &Vec<usize>, maps: &Maps) -> usize {
   let mut locations = get_locations(&maps, &seeds);
   locations.sort();
   locations[0]
}

fn part_2(raw_seed_ranges: &Vec<usize>, maps: &Maps) -> usize {
    let seed_ranges = calc_seed_ranges(raw_seed_ranges);
    let starts_from_transitions = maps.get_all_possible_starts();
    let starts_to_try = starts_from_transitions.iter()
        .filter(|&x| {
            let mut contains = false;
            for min_max in &seed_ranges {
                if &min_max[0] < x && &min_max[1] > x {
                    contains = true;
                    break
                }
            }
            contains
        });
    let mut lowest = usize::MAX;
    for seed in starts_to_try {
        let loc = maps.process(*seed);
        if loc < lowest {
            lowest = loc;
        }
    }
    lowest
}


fn calc_seed_ranges(data: &Vec<usize>) -> Vec<[usize;2]> {
    let mut seeds = Vec::new();
    assert!(data.len() % 2 == 0);
    let mut ind_source = 0;
    while ind_source * 2 < data.len() {
        let lowest_seed_ind = 2*ind_source;
        let range_ind = 2*ind_source + 1;
        let lowest_seed = data[lowest_seed_ind];
        let highest_seed = lowest_seed + data[range_ind];
        seeds.push([lowest_seed, highest_seed]);
        ind_source += 1;
    }
    seeds
}


fn get_locations(maps: &Maps, seeds: &Vec<usize>) -> Vec<usize> {
    let locations = seeds.iter().map(|x| {
        let v = maps.process(*x);
        v

    }).collect::<Vec<usize>>();
    locations
}


#[derive(Debug)]
struct Map {
    input: String,
    output: String,
    conversions: Vec<Conversion>,

}

impl Map {

    fn get_all_map_starts(&self) -> Vec<usize> {
        let mut all_starts = Vec::new();
        for conversion in &self.conversions {
            all_starts.push(conversion.source_start);
            let out_of_range = conversion.source_start + conversion.range + 1;
            all_starts.push(out_of_range);
        }
        all_starts
    }

    fn convert(&self, input: usize) -> usize {
        for conversion in self.conversions.iter() {
            if conversion.is_relevant(input) {
                // dbg!(&conversion);
                let diff = conversion.destination_start as isize - conversion.source_start as isize;
                
                let output = (input as isize + diff) as usize;
                return output
                
            }
        }

        return input;
    }

    fn reverse_convert(&self, output: usize) -> usize {
        for conversion in &self.conversions {
            if conversion.is_reverse_relevant(output) {
                let diff = conversion.destination_start as isize - conversion.source_start as isize;
                
                let input = (output as isize - diff) as usize;
                
                return input
                
            }
        }

        return output;
    }

}

#[derive(Debug)]
struct Maps (Vec<Map>);

impl Maps {

    fn get_all_possible_starts(&self) -> Vec<usize> {
        let mut all_endpoints = Vec::new();
        for map in &self.0 {
            let map_starts = map.get_all_map_starts();
            
            for input_start in &map_starts {
                let seed_id = self.reverse_process(*input_start, &map.input);
                all_endpoints.push(seed_id);
            }
        }
        all_endpoints
    }

    fn process(&self, input: usize) -> usize{
        let mut var = input;
        for map in &self.0 {
            let v = map.convert(var);
            var = v;
        }
        var
    }

    fn reverse_process(&self, output_id: usize, output_type: &str) -> usize {

        if output_type == "seed" {
            return output_id;
        }
        let last_map_index = match self.0.iter().position(|x| x.output == output_type) {
            Some(v) => v,
            None => {
                println!("{}", output_type);
                panic!();
            }
        };

        let mut var = output_id;
        for map_index in (0..=last_map_index).rev() {
            var = self.0[map_index].reverse_convert(var);
        }
        var
    }
}


#[derive(Debug)]
struct Conversion {
    source_start: usize,
    destination_start: usize,
    range: usize,
}

impl Conversion {

    fn is_relevant(&self, input: usize) -> bool {
        input >= self.source_start && input <= self.source_start + self.range
    }

    fn is_reverse_relevant(&self, output: usize) -> bool {
        output >= self.destination_start && output <= self.destination_start + self.range
    }

}

fn get_seeds(lines: &Vec<&str>) -> Vec<usize> {

    let seeds = lines[0]
        .split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|x| x != &"")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    seeds
}

fn get_maps(lines: &Vec<&str>) -> Maps {

    let mut maps = Vec::new();

    let mut start_indices = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        if line.ends_with("map:") {
            start_indices.push(index);
        }
    }

    for start_index in start_indices {
        let map = get_map(start_index, lines);
        maps.push(map);
    }

    Maps(maps)

}

fn get_map(start_line: usize, lines: &Vec<&str>) -> Map {

    let input_output = lines[start_line]
        .trim_end_matches(" map:")
        .split("-to-")
        .collect::<Vec<&str>>();

    let input = input_output[0].to_string();
    let output = input_output[1].to_string();

    let mut conversions = Vec::new();
    let mut index = start_line + 1;
    loop {
        if lines[index] == "" {
            break
        } else {
            let conversion = get_conversion(index, lines);
            conversions.push(conversion);
            index += 1;
        }

    }
    Map {
        input,
        output, 
        conversions
    }
}

fn get_conversion(line: usize, lines: &Vec<&str>) -> Conversion {

    let values = lines[line]
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    Conversion {
        source_start: values[1],
        destination_start: values[0],
        range: values[2]
    }
}












