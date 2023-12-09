use crate::common;
use std::cmp::{Ord, Eq, Ordering};

pub fn run() {
    print!("day 7: ");

    let start = std::time::SystemTime::now();

    let data = common::gather_data("./data/adventofcode.com_2023_day_7_input_scrap.txt");
    let lines = common::organize_data_into_lines(&data);



    let p1 = part_1(&lines);
    let p2 = part_2(&lines);

    let elapsed = start.elapsed().unwrap().as_millis();
    println!("{}, {} ({} ms) ", p1, p2, elapsed);

}

fn part_1(lines: &Vec<&str>) -> usize {
    let mut hands = get_hands(lines, false);
    let mut winnings = 0;
    hands.sort();
    for (pos, hand) in hands.iter().enumerate() {
        winnings += hand.bid * (pos+1);
    }
    winnings
}


fn part_2(lines: &Vec<&str>) -> usize {
    let mut hands = get_hands(lines, true);
    let mut winnings = 0;
    hands.sort();
    for (pos, hand) in hands.iter().enumerate() {
        winnings += hand.bid * (pos+1);
    }
    winnings
}


fn get_hands(lines: &Vec<&str>, jokers: bool) -> Vec<Hand> {
    let mut hands = Vec::new();
    for line in lines {
        if line == &"" {
            continue
        };
        let strs: Vec<&str> = line.split_whitespace().collect();
        let hand = Hand::from_strs(&strs, jokers);
        hands.push(hand);
    }
    hands
}


#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct CardGroup {
    card: Card,
    count: u8
}

impl HandType {
    fn from_cards(cards: &[Card; 5]) -> Self {

        let mut groups: Vec<CardGroup>= Vec::new();

        for card in cards {
            match groups.iter().position(|x| &x.card == card) {
                Some(pos) => {
                    groups[pos].count += 1;
                }, 
                None => {
                    groups.push(CardGroup{card: *card, count: 1})

                },
            }
        }
        groups.sort_by(|a, b| b.count.cmp(&a.count));

        if groups[0].count == 5 {
            HandType::FiveOfAKind
        } else if groups[0].count == 4 {
            HandType::FourOfAKind
        } else if groups[0].count == 3 && groups[1].count == 2 {
            HandType::FullHouse
        } else if groups[0].count == 3 {
            HandType::ThreeOfAKind
        } else if groups[0].count == 2 && groups[1].count == 2 {
            HandType::TwoPair
        } else if groups[0].count == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Card (u8);

impl Card {

    fn from_char(c: char, jokers: bool) -> Self {
        if c.is_digit(10) {
            let inner = c.to_digit(10).unwrap() as u8;
            Self(inner)
        } 
        else if c == 'T' {Self(10)}
        else if c == 'J' {
            if !jokers {
                Self(11) 
            } else {
                Self(1)
            }
        }
        else if c == 'Q' {Self(12)}
        else if c == 'K' {Self(13)}
        else if c == 'A' {Self(14)}
        else { panic!() }
    }
}


#[derive(Debug, PartialEq, Eq)]
struct Hand {
    hand_type: HandType,
    contents: [Card; 5],
    bid: usize,
}

impl Hand {
    fn from_strs (strs: &Vec<&str>, jokers: bool) -> Self {
        assert!(strs.len() == 2);
        let mut contents = [Card(0);5];
        for i in 0..5 {

            let chars: Vec<char> = strs[0].chars().collect();
            contents[i] = Card::from_char(chars[i], jokers);
        }
        let hand_type = HandType::from_cards(&contents);
        let bid: usize = strs[1].parse().unwrap();
        Self {
            hand_type, 
            contents,
            bid
        }
    }
}



impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {

        let self_type = if self.contents.contains (&Card(1)) {

            if self.contents == [Card(1);5] {
                self.hand_type.clone()
            } else {

                let mut best_type = HandType::HighCard;

                let mut fixed_cards = Vec::new();
                let mut joker_positions = Vec::new();

                for (pos, card) in self.contents.iter().enumerate() {
                    if card == &Card(1) {
                        joker_positions.push(pos);
                    } else {
                        fixed_cards.push(*card);
                    }
                }


                let mut mut_contents = self.contents.clone();
                for card in &fixed_cards {
                    for position in &joker_positions {
                        mut_contents[*position] = *card;
                    }
                    let new_type = HandType::from_cards(&mut_contents);
                    if new_type > best_type {
                        best_type = new_type;
                    }
                }
                best_type
            }
        } else {
            self.hand_type.clone()
        };

        let other_type = if other.contents.contains (&Card(1)) {

            if other.contents == [Card(1);5] {
                other.hand_type.clone()
            } else {

                let mut best_type = HandType::HighCard;

                let mut fixed_cards = Vec::new();
                let mut joker_positions = Vec::new();

                for (pos, card) in other.contents.iter().enumerate() {
                    if card == &Card(1) {
                        joker_positions.push(pos);
                    } else {
                        fixed_cards.push(*card);
                    }
                }


                let mut mut_contents = other.contents.clone();
                for card in &fixed_cards {
                    for position in &joker_positions {
                        mut_contents[*position] = *card;
                    }
                    let new_type = HandType::from_cards(&mut_contents);
                    if new_type > best_type {
                        best_type = new_type;
                    }
                }
                best_type
            }
        
        } else {
            other.hand_type.clone()
        };

        // println!("");
        // dbg!(&self_type);
        // dbg!(&other_type);

        if self_type == other_type {
            if self.contents[0] == other.contents[0] {
                if self.contents[1] == other.contents[1] {
                    if self.contents[2] == other.contents[2] {
                        if self.contents[3] == other.contents[3] {
                            if self.contents[4] == other.contents[4] {
                                panic!();
                            } else {
                                self.contents[4].0.cmp(&other.contents[4].0)
                            }
                        } else {
                            self.contents[3].0.cmp(&other.contents[3].0)
                        }
                    } else {
                        self.contents[2].0.cmp(&other.contents[2].0)
                    }
                } else {
                    self.contents[1].0.cmp(&other.contents[1].0)
                }
            } else {
                self.contents[0].0.cmp(&other.contents[0].0)
            }
        } else {
            self_type.cmp(&other_type)
        }
    }
}

