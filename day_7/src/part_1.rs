use common::itertools::Itertools;
use sscanf::sscanf;
use std::cmp::Ordering;

pub fn part_1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .sorted_by(|(a, _), (b, _)| {
            let hand_type_ord = a.hand_type().cmp(&b.hand_type());
            if hand_type_ord != Ordering::Equal {
                hand_type_ord
            } else {
                a.0.cmp(&b.0)
            }
        })
        .enumerate()
        .map(|(idx, (_, bid))| bid * (idx + 1))
        .sum()
}

fn parse_input(input: &str) -> Vec<(Hand, usize)> {
    input
        .lines()
        .map(|line| {
            let (c1, c2, c3, c4, c5, bid) =
                sscanf!(line, "{char}{char}{char}{char}{char} {usize}").unwrap();
            let hand = Hand([
                parse_card(c1),
                parse_card(c2),
                parse_card(c3),
                parse_card(c4),
                parse_card(c5),
            ]);
            (hand, bid)
        })
        .collect_vec()
}

fn parse_card(c: char) -> Card {
    match c {
        'A' => Card::A,
        'K' => Card::K,
        'Q' => Card::Q,
        'J' => Card::J,
        'T' => Card::T,
        '9' => Card::_9,
        '8' => Card::_8,
        '7' => Card::_7,
        '6' => Card::_6,
        '5' => Card::_5,
        '4' => Card::_4,
        '3' => Card::_3,
        '2' => Card::_2,
        _ => panic!("Invalid card {}", c),
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct Hand([Card; 5]);

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut card_counts = self
            .0
            .iter()
            .into_group_map_by(|&&c| c)
            .into_iter()
            .map(|(_, copies)| copies.iter().count())
            .sorted_by(|a, b| b.cmp(a));
        let max = card_counts.next().unwrap();
        let max_2 = card_counts.next().unwrap_or_else(|| 0);

        if max == 5 {
            HandType::FiveOfAKind
        } else if max == 4 {
            HandType::FourOfAKind
        } else if max == 3 && max_2 == 2 {
            HandType::FullHouse
        } else if max == 3 {
            HandType::ThreeOfAKind
        } else if max == 2 && max_2 == 2 {
            HandType::TwoPair
        } else if max == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug)]
enum HandType {
    /// e.g. 23456
    HighCard,
    /// e.g. A23A4
    OnePair,
    /// e.g. 23432
    TwoPair,
    /// e.g. TTT98
    ThreeOfAKind,
    /// e.g. 23332
    FullHouse,
    /// e.g. AA8AA
    FourOfAKind,
    /// e.g. AAAAA
    FiveOfAKind,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Card {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    J,
    Q,
    K,
    A,
}

#[cfg(test)]
mod tests {
    use super::*;
    use Card::*;

    static TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_hand_type() {
        assert_eq!(Hand([_2, _3, _4, _5, _6]).hand_type(), HandType::HighCard);
        assert_eq!(Hand([A, _2, _3, A, _4]).hand_type(), HandType::OnePair);
        assert_eq!(Hand([_2, _3, _4, _3, _2]).hand_type(), HandType::TwoPair);
        assert_eq!(Hand([T, T, T, _9, _8]).hand_type(), HandType::ThreeOfAKind);
        assert_eq!(Hand([_2, _3, _3, _3, _2]).hand_type(), HandType::FullHouse);
        assert_eq!(Hand([A, A, _8, A, A]).hand_type(), HandType::FourOfAKind);
        assert_eq!(Hand([A, A, A, A, A]).hand_type(), HandType::FiveOfAKind);

        assert!(Hand([A, A, A, A, A]).hand_type() > Hand([A, A, _8, A, A]).hand_type());
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 6440);
    }
}
