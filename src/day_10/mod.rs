use crate::common;



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
    let matrix = get_matrix(lines);
    let steps = matrix.path_steps();
    steps / 2
}

#[derive(Debug)]
enum Position {
    Inside,
    Outside,
    WalkingAlongWall((FromAboveBelow, FromInsideOtside)),
    CrossingWall(FromInsideOtside),
}

#[derive(Debug)]
enum FromInsideOtside {
    Inside,
    Outside,
}

impl FromInsideOtside {
    fn opposite(&self) -> Self {
        match self {
            Self::Inside => Self::Outside,
            Self::Outside => Self::Inside,
        }
    }
}

#[derive(Debug)]
enum FromAboveBelow {
    Above, 
    Below
}




fn part_2(lines: &Vec<&str>) -> usize {
    let mut matrix = get_matrix(lines);
    let hidden_pipe = matrix.mark_path();
    let mut count = 0;
    let mut position = Position::Outside;
    for (row_index, row) in matrix.0.iter().enumerate() {
        for (colunm_index, _column) in row.iter().enumerate() {

            let symbol = if matrix.0[row_index][colunm_index].0 == 'S' {
                hidden_pipe
            } else {
                matrix.0[row_index][colunm_index].0
            };

            
            match position {
                Position::Outside => {
                    // if path.contains(&(row_index, colunm_index)) {
                    if matrix.0[row_index][colunm_index].1 {

                        match symbol {
                            'F' => {
                                position = Position::WalkingAlongWall((FromAboveBelow::Above, FromInsideOtside::Outside));
                            },
                            'L' => {
                                position = Position::WalkingAlongWall((FromAboveBelow::Below, FromInsideOtside::Outside));
                            },
                            '|' => {
                                position = Position::CrossingWall(FromInsideOtside::Outside);
                            },
                            
                            _ => {

                                dbg!(count);
                                dbg!(&symbol);
                                dbg!(row_index, colunm_index);
                                dbg!(&position);
                                panic!();

                            }

                        }

                    }
                    
                },
                Position::Inside => {
                    
                    if matrix.0[row_index][colunm_index].1 {

                        match symbol {
                            'F' => {
                                position = Position::WalkingAlongWall((FromAboveBelow::Above, FromInsideOtside::Inside));
                            },
                            'L' => {
                                position = Position::WalkingAlongWall((FromAboveBelow::Below, FromInsideOtside::Inside));
                            },
                            '|' => {
                                position = Position::CrossingWall(FromInsideOtside::Inside);
                            },
                            _ => panic!(""),

                        }

                    } else {
                        
                        count += 1;

                        if count == 1 {
                            dbg!(&symbol);
                            dbg!(row_index, colunm_index);
                            dbg!(&position);
                            panic!();


                        }
                    }
                    
                },
                Position::WalkingAlongWall((from_above_below, from_inside_outside)) => {

                    if matrix.0[row_index][colunm_index].1 {
                        match symbol {
                            '-' => {
                                position = Position::WalkingAlongWall((from_above_below, from_inside_outside));
                            },
                            'J' => {
                                match from_above_below {
                                    FromAboveBelow::Above => {
                                        position = Position::CrossingWall(from_inside_outside);
                                    },
                                    FromAboveBelow::Below => {
                                        position = Position::CrossingWall(from_inside_outside.opposite());
                                    }
                                }

                            },
                            '7' => {
                                match from_above_below {
                                    FromAboveBelow::Above => {
                                        position = Position::CrossingWall(from_inside_outside.opposite());
                                    },
                                    FromAboveBelow::Below => {
                                        position = Position::CrossingWall(from_inside_outside);
                                    }
                                }
                            },
                            _ => {
                                dbg!(row_index, colunm_index);
                                dbg!(&symbol);
                                panic!("");
                            },

                        }

                    } else {
                        panic!();
                    }
                    
                },
                Position::CrossingWall(from_inside_outside) => {

                    if matrix.0[row_index][colunm_index].1 {
                        let other_way = match from_inside_outside {
                            FromInsideOtside::Inside => {
                                FromInsideOtside::Outside
                            },
                            FromInsideOtside::Outside => {
                                FromInsideOtside::Inside
                            },
                        };
                        match symbol {
                            '|' => {
                                position = Position::CrossingWall(other_way);
                            },
                            'L' => {
                                position = Position::WalkingAlongWall((FromAboveBelow::Below, other_way));
                            },
                            'F' => {
                                position = Position::WalkingAlongWall((FromAboveBelow::Above, other_way));
                            },
                            _ => panic!(""),

                        }

                    } else {
                        match from_inside_outside {
                            FromInsideOtside::Inside => {
                                position = Position::Outside;
                            },
                            FromInsideOtside::Outside => {

                                position = Position::Inside;

                                count += 1;
                                if count == 1 {

                                }
                            },
                        }
                    }
                    
                },
                
            }
        }
    }

    count

}


fn get_matrix(lines: &Vec<&str>) -> Matrix {
    let mut inner = Vec::new();
    for line in lines {
        if line == &"" {continue};
        let chars = line.chars().map(|x| (x,false)).collect();
        inner.push(chars);
    }
    Matrix(inner)
}


#[derive(Debug)]
struct Matrix (Vec<Vec<(char, bool)>>);

impl Matrix {

    fn path_steps (&self) -> usize {
        let mut steps = 0;
        for row in &self.0 {
            for tile in row {
                if tile.1 {
                    steps += 1;
                }
            }
        }
        steps
    }

    fn find_start(&self) -> (usize, usize){ 
        for (row_pos, row) in self.0.iter().enumerate() {
            for (col_pos, _column) in row.iter().enumerate() {
                if self.0[row_pos][col_pos].0 == 'S' {
                    return (row_pos, col_pos)
                }
            }
        }
        panic!();
    }


    fn mark_path (&mut self) -> char {


        let accessable_from_left = vec!['7', '-', 'J'];
        let accessable_from_right = vec!['L', '-', 'F'];
        let accessable_from_above = vec!['L', '|', 'J'];
        let accessable_from_below = vec!['7', '|', 'F'];

        let (mut row, mut col) = self.find_start();

        self.0[row][col].1 = true;

        let mut coming_from: char = 'X';
        let mut hidden_pipe: char = 'X';

        if false {}
        else if row + 1 < self.0[row].len() && accessable_from_above.contains(&self.0[row + 1][col].0) {
        
            if col + 1 < self.0[row].len() && accessable_from_left.contains(&self.0[row][col + 1].0) {
                hidden_pipe = 'F';
            } else if col > 0 && accessable_from_right.contains(&self.0[row][col - 1].0) {
                hidden_pipe = '7';
            } else if row > 0 && accessable_from_below.contains(&self.0[row - 1][col].0) {
                hidden_pipe = '|';
            } else {
                panic!();
            }
            row += 1; 
            coming_from = 'N';
        } else if col + 1 < self.0[row].len() && accessable_from_left.contains(&self.0[row][col + 1].0) { 

            if col > 0 && accessable_from_right.contains(&self.0[row][col - 1].0) {
                hidden_pipe = '-';
            } else if row > 0 && accessable_from_below.contains(&self.0[row - 1][col].0) {
                hidden_pipe = 'L';
            } else {
                panic!();
            }
            col += 1; 
            coming_from = 'W';
        } else if col > 0 && accessable_from_right.contains(&self.0[row][col -1].0) { 

            if row > 0 && accessable_from_below.contains(&self.0[row - 1][col].0) {
                hidden_pipe = 'J';
            } else {
                panic!();
            }
            col -= 1; 
            coming_from = 'E';
        } else {panic!()};

        while self.0[row][col].0 != 'S' {
            self.0[row][col].1 = true;

            match self.0[row][col].0 {
                '|' => {
                    match coming_from {
                        'N' => {
                            row += 1;
                            coming_from = 'N';
                        },
                        'S' => {
                            row -= 1;
                            coming_from = 'S';
                        },
                        _ => panic!(),
                    }
                },
                '-' => {
                    match coming_from {
                        'E' => {
                            col -= 1;
                            coming_from = 'E';
                        },
                        'W' => {
                            col += 1;
                            coming_from = 'W';
                        },
                        _ => panic!(),
                    }

                },
                'L' => {
                    match coming_from {
                        'E' => {
                            row -= 1;
                            coming_from = 'S';
                        },
                        'N' => {
                            col += 1;
                            coming_from = 'W';
                        },
                        _ => panic!(),
                    }

                },
                'J' => {
                    match coming_from {
                        'W' => {
                            row -= 1;
                            coming_from = 'S';
                        },
                        'N' => {
                            col -= 1;
                            coming_from = 'E';
                        },
                        _ => panic!(),
                    }

                },
                '7' => {
                    match coming_from {
                        'W' => {
                            row += 1;
                            coming_from = 'N';
                        },
                        'S' => {
                            col -= 1;
                            coming_from = 'E';
                        },
                        _ => panic!(),
                    }

                },
                'F' => {
                    match coming_from {
                        'E' => {
                            row += 1;
                            coming_from = 'N';
                        },
                        'S' => {
                            col += 1;
                            coming_from = 'W';
                        },
                        _ => panic!(),
                    }

                },
                _ => panic!(),

            }
        }
        hidden_pipe
    }
}
