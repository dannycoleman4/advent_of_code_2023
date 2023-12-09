use crate::common;


pub fn run() {
    print!("day 4: ");
    // let data = common::gather_data("./data/adventofcode.com_2023_day_2_input.txt");
    let data = common::gather_data("./data/adventofcode.com_2023_day_4_input.txt");
    let lines = common::organize_data_into_lines(&data);
    let mut deck = Deck::from(lines);
    let score = deck.score();
    deck.add_reward_cards();
    let count = deck.count();
    
    println!("{}, {}", score, count);

    // println!("{}, {}", part_number_sum, gear_ratios_sum);
}


#[derive(Debug)]
struct Card {
    count: usize,
    id: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}


impl From<&str> for Card {
    fn from(line: &str) -> Self {
        let name_and_data: Vec<&str> = line.split(": ").collect();
        let count = 1;
        let id = {
            let junk_and_num: Vec<&str> = name_and_data[0].split(" ").collect();
            let num: usize = junk_and_num.last().unwrap().parse().unwrap();
            num
        };
        let winning_and_numbers: Vec<&str> = name_and_data[1].split(" | ").collect();

        let winning_numbers = {
            let winning_strs: Vec<&str> = winning_and_numbers[0].split(" ").collect();
            let mut winning = Vec::new();
            for s in winning_strs {
                if s != "" {
                    let n: usize = s.parse().unwrap();
                    winning.push(n);
                }
            }
            winning
        };

        let numbers = {
            let strs: Vec<&str> = winning_and_numbers[1].split(" ").collect();
            let mut nums = Vec::new();
            for s in strs {
                if s != "" {
                    let n: usize = s.parse().unwrap();
                    nums.push(n);
                }
            }
            nums
        };

        Self { count, id, winning_numbers, numbers }

    }
}

impl Card {

    fn score(&self) -> usize {
        let mut score = 0;
        for w in &self.winning_numbers {
            if self.numbers.contains(w) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        score
    }

    fn matches(&self) -> usize {
        let mut matches = 0;
        for c in &self.numbers {
            if self.winning_numbers.contains(c) {
                matches += 1;
            }

        }
        matches

    }
}


#[derive(Debug)]
struct Deck (Vec<Card>);

impl From<Vec<&str>> for Deck {
    fn from (lines: Vec<&str>) -> Self {
        let mut deck = Vec::new();
        for line in lines {
            if line != "" {
                let card = Card::from(line);
                deck.push(card);
            }
        }
        Deck(deck)
    }
}


impl Deck {
    fn score(&self) -> usize {
        let mut score = 0;
        for card in &self.0 {
            score += card.score();
        }
        score
    }

    fn count(&self) -> usize {
        let mut count = 0;
        for c in &self.0 {
            count += c.count;
        }
        count
    }

    fn add_reward_cards(&mut self) {
        let mut index = 0;
        while index < self.0.len() {
            assert!(self.0[index].id == index + 1);

            let matches = self.0[index].matches();
            for i in (index + 1)..=(index + matches) {
                self.0[i].count += self.0[index].count;
            }

            index += 1;
            // dbg!(index);


        }
    }
}













