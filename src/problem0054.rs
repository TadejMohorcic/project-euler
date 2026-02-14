use std::collections::HashMap;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn solver() -> io::Result<()> {
    let input = File::open("input/54.txt")?;
    let reader = BufReader::new(input);
    
    let mut result = 0;

    for line in reader.lines() {
        let ok_line = line?;
        if compare_hands(&ok_line) {
            result += 1;
        }
    }
    
    println!("Problem 0054 - Poker Hands: {}", result);

    Ok(())
}

#[derive(Debug, Clone)]
struct Card {
    value: u32,
    suit: char
}

fn compare_hands(hand: &str) -> bool {
    let (player_1, player_2) = split_hand(hand);

    let rank_1 = score_hand(&player_1);
    let rank_2 = score_hand(&player_2);

    rank_1 > rank_2
}

fn score_hand(played_hand: &Vec<Card>) -> (u32, Vec<u32>) {
    let mut values: Vec<u32> = played_hand.iter().map(|c| c.value).collect();
    values.sort_by(|a, b| b.cmp(a));

    let mut counts = HashMap::new();

    for v in &values {
        *counts.entry(*v).or_insert(0) += 1;
    }

    let mut groups: Vec<(usize, u32)> = counts.iter().map(|(&v, &c)| (c, v)).collect();
    groups.sort_by(|a, b| b.cmp(a));

    let is_flush = played_hand.iter().all(|c| c.suit == played_hand[0].suit);

    let mut is_straight = false;

    if values.windows(2).all(|w| w[0] == w[1] + 1) {
        is_straight = true;
    }

    if values == [14, 5, 4, 3, 2].to_vec() {
        is_straight = true;
        values = [5, 4, 3, 2, 1].to_vec();
    }

    let rank = match (is_flush, is_straight, groups.as_slice()) {
        (true, true, _) if values[0] == 14 => 10,
        (true, true, _) => 9,
        (_, _, [(4, _), ..]) => 8,
        (_, _, [(3, _), (2, _), ..]) => 7,
        (true, _, _) => 6,
        (_, true, _) => 5,
        (_, _, [(3, _), ..]) => 4,
        (_, _, [(2, _), (2, _), ..]) => 3,
        (_, _, [(2, _), ..]) => 2,
        _ => 1
    };

    let mut tiebreak = Vec::new();
    for (count, value) in &groups {
        for _ in 0..*count {
            tiebreak.push(*value);
        }
    }

    (rank, tiebreak)
}

fn split_hand(hand: &str) -> (Vec<Card>, Vec<Card>) {
    let cards: Vec<Card> = hand.split_whitespace().map(|c| {
        let mut chars = c.chars();
        let value = match chars.next().unwrap() {
            '2'..='9' => c.chars().next().unwrap().to_digit(10).unwrap(),
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => unreachable!("Invalid card value!")
        };
        let suit = c.chars().next().unwrap();
        Card {value: value, suit: suit}
    }).collect();

    (cards[..5].to_vec(), cards[5..].to_vec())
}
 