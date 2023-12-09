use crate::common;


pub fn run() {
    print!("day 3: ");
    // let data = common::gather_data("./data/adventofcode.com_2023_day_2_input.txt");
    let data = common::gather_data("./data/adventofcode.com_2023_day_3_input.txt");
    let lines = common::organize_data_into_lines(&data);
    let matrix = build_matrix(&lines);
    let part_number_sum = sum_part_numbers(&matrix);
    let gear_ratios_sum = sum_gear_ratios(&matrix);

    println!("{}, {}", part_number_sum, gear_ratios_sum);
}


fn build_matrix (lines: &Vec<&str>) -> Vec<Vec<char>> {

    let mut matrix = Vec::new();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        if chars.len() > 0 {
            matrix.push(chars);
        }
    }
    matrix
}


pub fn sum_part_numbers(matrix: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;

    for (row_index, row) in matrix.iter().enumerate() {
        let columns = row.len();
        let mut column_index = 0;


        while column_index < columns { 
            if row[column_index].is_digit(10) {
                let start_column = column_index;
                let mut end_column = column_index;
                let mut string_num = String::new();
                string_num.push(row[column_index]);
                column_index += 1;
                while column_index < columns && row[column_index].is_digit(10) {
                    end_column = column_index;
                    string_num.push(row[column_index]);
                    column_index += 1;
                }

                if is_valid(row_index, start_column, end_column, matrix) {
                    let num: usize = string_num.parse().unwrap();
                    sum += num;
                } else {
                    // let num: usize = string_num.parse().unwrap();
                }


            } else {
                column_index += 1;
            }
        }
    }
    sum
}



fn is_valid(row: usize, start_column: usize, end_column: usize, matrix: &Vec<Vec<char>>) -> bool {

    if start_column > 0 {
        assert!(!matrix[row][start_column - 1].is_digit(10));
        if matrix[row][start_column - 1] != '.' {
            return true;
        }
    }

    if end_column + 1 < matrix[row].len() {
        assert!(!matrix[row][end_column + 1].is_digit(10));
        if matrix[row][end_column + 1] != '.' {
            return true;
        }
    }

    let lower_column = if start_column > 0 {
        start_column - 1
    } else {
        0
    };

    let upper_column = if end_column < matrix[row].len() - 1 {
        end_column + 1
    } else {
        end_column
    };

    if row > 0 {

        for column in lower_column..=upper_column {
            if matrix[row - 1][column] != '.' && !matrix[row -1][column].is_digit(10) {
                return true;
            }
        }
    }

    if row < matrix.len() - 1 {

        for column in lower_column..=upper_column {
            if matrix[row + 1][column] != '.' && !matrix[row + 1][column].is_digit(10) {
                return true;
            }
        }
    }
    false
}


fn adjacent_numbers(row: usize, column: usize, matrix: &Vec<Vec<char>>) -> Vec<usize> {
    let mut adjacent_numbers = Vec::new();
    if column > 0 {
        let mut string_number = String::new();
        for col_index in (0..column).rev() {
            if matrix[row][col_index].is_digit(10) {
                string_number.insert(0, matrix[row][col_index]);
            } else {
                break
            }
        }
        if string_number.len() > 0 {
            let num: usize = string_number.parse().unwrap();
            adjacent_numbers.push(num);
        }
    }

    if column < (matrix[row].len() - 1) {
        let mut string_number = String::new();
        for col_index in (column + 1)..matrix[row].len() {
            if matrix[row][col_index].is_digit(10) {
                string_number.push(matrix[row][col_index]);
            } else {
                break
            }
        }
        if string_number.len() > 0 {
            let num: usize = string_number.parse().unwrap();
            adjacent_numbers.push(num);
        }
    }

    if row > 0 {
        if matrix[row - 1][column].is_digit(10) {
            let num = complete_number(column, &matrix[row - 1]);
            adjacent_numbers.push(num);
        } else {
            if matrix[row - 1][column - 1].is_digit(10) {
                let num = complete_number(column - 1, &matrix[row - 1]);
                adjacent_numbers.push(num);
            } 
            if matrix[row - 1][column + 1].is_digit(10) {
                let num = complete_number(column + 1, &matrix[row - 1]);
                adjacent_numbers.push(num);
            }
        }
    }

    if row < (matrix.len() - 1) {
        if matrix[row + 1][column].is_digit(10) {
            let num = complete_number(column, &matrix[row + 1]);
            adjacent_numbers.push(num);
        } else {
            if matrix[row + 1][column - 1].is_digit(10) {
                let num = complete_number(column - 1, &matrix[row + 1]);
                adjacent_numbers.push(num);
            } 
            if matrix[row + 1][column + 1].is_digit(10) {
                let num = complete_number(column + 1, &matrix[row + 1]);
                adjacent_numbers.push(num);
            }
        }
    }

    adjacent_numbers
}


fn complete_number(column: usize, row: &Vec<char>) -> usize {
    assert!(row[column].is_digit(10));
    let mut string_num = String::new();

    for c in column..row.len() {
        if row[c].is_digit(10) {
            string_num.push(row[c])
        } else {
            break
        }
    }

    for c in (0..column).rev() {
        if row[c].is_digit(10) {
            string_num.insert(0, row[c])
        } else {
            break
        }

    }

    let num: usize = string_num.parse().unwrap();
    num
}

fn sum_gear_ratios(matrix: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    for (row_index, row) in matrix.iter().enumerate() {
        for (column_index, _) in row.iter().enumerate() {
            if matrix[row_index][column_index] == '*' {
                let adjacent_numbers = adjacent_numbers(row_index, column_index, matrix);
                if adjacent_numbers.len() == 2 {
                    let ratio = adjacent_numbers[0] * adjacent_numbers[1];
                    sum += ratio;
                }
            }
        }
    }
    sum
}









