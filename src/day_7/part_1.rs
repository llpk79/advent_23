use std::collections::HashMap;
use std::fs::read_to_string;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq, Hash)]
struct Card {
    label: char,
    rank: u8,
}

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
struct Hand {
    cards: Vec<Card>,
    counts: i64,
    bet: i64,
}

fn count_cards(cards: &Vec<Card>) -> Vec<i64> {
    let mut counts = HashMap::new();
    for card in cards {
        let count = counts.entry(card).or_insert(0);
        *count += 1
    }
    counts.values().map(|num| *num as i64).collect()
}

pub fn part_1() -> Option<()> {
    let chars = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
    let hand_types: Vec<Vec<i64>> = Vec::from([
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 2],
        vec![1, 2, 2],
        vec![1, 1, 3],
        vec![2, 3],
        vec![1, 4],
        vec![5],
    ]);

    let hand_ranks: HashMap<&Vec<i64>, i64> = hand_types
        .iter()
        .enumerate()
        .map(|(i, hand)| (hand, i as i64))
        .collect();
    let char_ranks = chars
        .iter()
        .enumerate()
        .map(|(i, c)| (*c, i as u8))
        .collect::<HashMap<char, u8>>();

    let input = read_to_string("src/day_7/input.txt").expect("file exists");
    let lines = input.lines();
    let cards = lines
        .map(|line| {
            let split = line.split_once(' ').unwrap();
            let cards = split
                .0
                .chars()
                .map(|c| {
                    let rank = char_ranks.get(&c).unwrap();
                    Card {
                        label: c,
                        rank: *rank,
                    }
                })
                .collect::<Vec<Card>>();
            let bet: i64 = split
                .1
                .chars()
                .filter(|d| d.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap();
            (cards, bet)
        })
        .collect::<Vec<(Vec<Card>, i64)>>();

    let mut hands = cards
        .iter()
        .map(|hand| {
            let mut count = count_cards(&hand.0.clone());
            count.sort();
            Hand {
                counts: *hand_ranks.get(&count).unwrap(),
                cards: hand.0.clone(),
                bet: hand.1,
            }
        })
        .collect::<Vec<Hand>>();

    hands.sort_by(|a: &Hand, b: &Hand| a.cards[4].rank.cmp(&b.cards[4].rank));
    hands.sort_by(|a: &Hand, b: &Hand| a.cards[3].rank.cmp(&b.cards[3].rank));
    hands.sort_by(|a: &Hand, b: &Hand| a.cards[2].rank.cmp(&b.cards[2].rank));
    hands.sort_by(|a: &Hand, b: &Hand| a.cards[1].rank.cmp(&b.cards[1].rank));
    hands.sort_by(|a: &Hand, b: &Hand| a.cards[0].rank.cmp(&b.cards[0].rank));
    hands.sort_by(|a: &Hand, b: &Hand| a.counts.cmp(&b.counts));

    let answer: i64 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as i64 * hand.bet)
        .sum();
    println!("answer {answer}");
    Some(())
}
