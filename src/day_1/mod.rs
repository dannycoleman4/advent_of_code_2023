use crate::common;

pub fn run() {
    print!("day 1: ");

    let data = common::gather_data("./data/adventofcode.com_2023_day_1_input.txt");
    let lines = common::organize_data_into_lines(&data);
    let sum = sum_calibration_values(&lines);

    println!("{}", sum);
}

fn extract_calibration_value(line: &str) -> usize {

    let mut string_num = String::new();
    for c in line.chars() {
        if c.is_digit(10) {
            string_num.push(c);
            break
        }
    }

    for c in line.chars().rev() {
        if c.is_digit(10) {
            string_num.push(c);
            break
        }
    }

    if string_num.len() == 2 {
        let num: usize = string_num.parse().unwrap();
        num
    } else {
        assert!(string_num.len() == 0);
        0
    }
}


fn sum_calibration_values(lines: &Vec<&str>) -> usize {

    let mut sum = 0;
    for line in lines {
        let val = extract_calibration_value(line);
        sum += val
    }
    sum
}



