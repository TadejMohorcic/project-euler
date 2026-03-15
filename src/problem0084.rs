use rand::prelude::*;

use std::collections::HashMap;
use std::collections::VecDeque;

pub fn solver() {
    let top_three = play_monopoly(4, 1000000);
    let result = 100 * (100 * top_three[0] + top_three[1]) + top_three[2];
    println!("Problem 0084 - Monopoly Odds: {}", result);
}

fn throw_dice(n: usize) -> (usize, bool) {
    let mut rng = rand::rng();

    let d1: usize = rng.random_range(1..=n);
    let d2: usize = rng.random_range(1..=n);

    (d1 + d2, d1 == d2)
}

fn community_chest(cards: &Vec<&str>, index: usize, position: usize) -> usize {
    match cards[index] {
        "GO" => return 0,
        "JAIL" => return 10,
        _ => return position,
    }
}

fn chance_chest(cards: &Vec<&str>, index: usize, position: usize) -> usize {
    match cards[index] {
        "GO" => return 0,
        "JAIL" => return 10,
        "C1" => return 11,
        "E3" => return 24,
        "H2" => return 39,
        "R1" => return 5,
        "U" => {
            if 11 < position && position < 28 {
                return 28;
            } else {
                return 12;
            }
        }
        "R" => {
            if 4 < position && position < 15 {
                return 15;
            } else if 14 < position && position < 25 {
                return 25;
            } else if 24 < position && position < 35 {
                return 35;
            } else {
                return 5;
            }
        }
        "REV3" => return ((position - 3) % 40 + 40) % 40,
        _ => return position,
    }
}

fn play_monopoly(dice_size: usize, max_steps: usize) -> Vec<usize> {
    let mut visited = [0usize; 40];

    let mut rng = rand::rng();

    let mut cc_index = 0;
    let mut cc_cards: Vec<&str> = vec!["NONE"; 14];
    cc_cards.push("JAIL");
    cc_cards.push("GO");

    cc_cards.shuffle(&mut rng);

    let mut chance_index = 0;
    let mut chance_cards: Vec<&str> = vec!["NONE"; 6];
    chance_cards.push("GO");
    chance_cards.push("JAIL");
    chance_cards.push("C1");
    chance_cards.push("E3");
    chance_cards.push("H2");
    chance_cards.push("R1");
    chance_cards.push("U");
    chance_cards.push("REV3");

    for _ in 0..2 {
        chance_cards.push("R");
    }

    chance_cards.shuffle(&mut rng);

    let mut position = 0;
    let mut number_of_doubles = 0;

    for _ in 0..max_steps {
        let (dice_roll, double) = throw_dice(dice_size);
        position = (position + dice_roll) % 40;

        if double {
            number_of_doubles += 1;
        } else {
            number_of_doubles = 0;
        }

        if number_of_doubles == 3 {
            position = 10;
            number_of_doubles = 0;
        }

        loop {
            let old_position = position;

            match position {
                30 => position = 10,
                2 | 17 | 33 => {
                    position = community_chest(&cc_cards, cc_index, position);
                    cc_index = (cc_index + 1) % 16;
                }
                7 | 22 | 36 => {
                    position = chance_chest(&chance_cards, chance_index, position);
                    chance_index = (chance_index + 1) % 16;
                }
                _ => {}
            }

            if old_position == position {
                break;
            }
        }

        visited[position] += 1;
    }

    let mut indexed: Vec<(usize, &usize)> = visited.iter().enumerate().collect();
    indexed.sort_by(|a, b| b.1.cmp(a.1));
    let top_three: Vec<usize> = indexed.iter().take(3).map(|(idx, _)| *idx).collect();

    top_three
}
